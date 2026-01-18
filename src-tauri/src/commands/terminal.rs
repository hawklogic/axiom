// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Terminal command handlers.

use crate::logging;
use crate::state::AppState;
use axiom_terminal::{SessionId, TerminalSize};
use tauri::{AppHandle, Emitter, State};
use serde::Serialize;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::Mutex;

/// Terminal output event payload
#[derive(Clone, Serialize)]
struct TerminalOutput {
    id: SessionId,
    data: Vec<u8>,
}

/// Global registry for terminal reader threads
static TERMINAL_READERS: std::sync::OnceLock<Mutex<HashMap<SessionId, Arc<AtomicBool>>>> = std::sync::OnceLock::new();

fn get_readers() -> &'static Mutex<HashMap<SessionId, Arc<AtomicBool>>> {
    TERMINAL_READERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Create a new terminal session.
#[tauri::command]
pub fn terminal_create(state: State<AppState>, app: AppHandle) -> Result<SessionId, String> {
    logging::info("terminal", "Creating new PTY session");
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let id = manager.create_session().map_err(|e| {
        logging::error("terminal", &format!("Failed to create session: {}", e));
        e.to_string()
    })?;
    logging::info("terminal", &format!("PTY session {} created", id));
    
    // Start reader thread for this session
    // Uses poll() with timeout - efficient waiting in kernel, can still be stopped
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    // Store the running flag so we can stop it later
    if let Ok(mut readers) = get_readers().lock() {
        readers.insert(id, running.clone());
    }
    
    // Get the reader and fd from the session for polling
    if let Some(session) = manager.get(id) {
        let reader = session.pty.reader();
        let reader_fd = session.pty.get_fd();
        let app_handle = app.clone();
        
        println!("[TERMINAL] Starting reader thread for session {}, fd={:?}", id, reader_fd);
        logging::info("terminal", &format!("Starting reader thread for session {}, fd={:?}", id, reader_fd));
        
        std::thread::spawn(move || {
            let mut buf = vec![0u8; 4096];
            println!("[TERMINAL] Reader thread running for session {}, fd={:?}", id, reader_fd);
            logging::info("terminal", &format!("Reader thread started for session {}, fd={:?}", id, reader_fd));
            
            loop {
                // Check if we should stop
                if !running_clone.load(Ordering::Relaxed) {
                    break;
                }
                
                // Wait for data with timeout using poll()
                // This is NOT polling in a loop - it's a single efficient kernel wait
                if let Some(fd) = reader_fd {
                    let mut pollfd = libc::pollfd {
                        fd,
                        events: libc::POLLIN,
                        revents: 0,
                    };
                    
                    // Wait up to 100ms for data (allows checking running flag periodically)
                    let result = unsafe { libc::poll(&mut pollfd, 1, 100) };
                    
                    if result < 0 {
                        // Error
                        logging::warn("terminal", &format!("poll() error on session {}", id));
                        continue;
                    }
                    
                    if result == 0 {
                        // Timeout - this is normal, just continue to check running flag
                        continue;
                    }
                    
                    println!("[TERMINAL] poll() returned {} revents={}", result, pollfd.revents);
                    logging::debug("terminal", &format!("poll() returned {} revents={}", result, pollfd.revents));
                    
                    if (pollfd.revents & (libc::POLLHUP | libc::POLLERR)) != 0 {
                        // PTY closed
                        logging::info("terminal", &format!("PTY hangup on session {}", id));
                        break;
                    }
                    
                    if (pollfd.revents & libc::POLLIN) == 0 {
                        // No data ready
                        continue;
                    }
                } else {
                    // No fd available - can't poll, just try reading with small sleep
                    println!("[TERMINAL] WARNING: No fd available for polling!");
                    logging::warn("terminal", "No fd available for polling, falling back to sleep");
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                
                // Data is available - read it
                let mut reader_guard = match reader.lock() {
                    Ok(guard) => guard,
                    Err(_) => break,
                };
                
                match reader_guard.read(&mut buf) {
                    Ok(0) => {
                        // EOF - PTY closed
                        drop(reader_guard);
                        logging::debug("terminal", &format!("EOF on session {}", id));
                        break;
                    }
                    Ok(n) => {
                        drop(reader_guard);
                        println!("[TERMINAL] Read {} bytes from session {}", n, id);
                        logging::debug("terminal", &format!("Read {} bytes from session {}", n, id));
                        let data = buf[..n].to_vec();
                        if let Err(e) = app_handle.emit("terminal-output", TerminalOutput { id, data }) {
                            println!("[TERMINAL] Failed to emit: {}", e);
                            logging::error("terminal", &format!("Failed to emit terminal-output: {}", e));
                        }
                    }
                    Err(e) => {
                        drop(reader_guard);
                        if e.kind() == std::io::ErrorKind::Interrupted {
                            continue;
                        }
                        logging::debug("terminal", &format!("Read error on session {}: {}", id, e));
                        break;
                    }
                }
            }
            
            // Cleanup
            if let Ok(mut readers) = get_readers().lock() {
                readers.remove(&id);
            }
            logging::debug("terminal", &format!("Reader thread stopped for session {}", id));
        });
    }
    
    Ok(id)
}

/// Write to a terminal session.
#[tauri::command]
pub fn terminal_write(state: State<AppState>, id: SessionId, data: String) -> Result<usize, String> {
    logging::debug("terminal", &format!("Writing {} bytes to session {}", data.len(), id));
    let manager = state.terminal_manager.lock().map_err(|e| {
        logging::error("terminal", &format!("Failed to lock manager for write: {}", e));
        e.to_string()
    })?;
    let session = manager.get(id).ok_or_else(|| {
        logging::warn("terminal", &format!("Session {} not found for write", id));
        "Session not found".to_string()
    })?;
    session.write(data.as_bytes()).map_err(|e| {
        logging::error("terminal", &format!("Write error on session {}: {}", id, e));
        e.to_string()
    })
}

/// Read from a terminal session (deprecated - use terminal-output events instead).
/// Kept for backward compatibility.
#[tauri::command]
pub fn terminal_read(_state: State<AppState>, _id: SessionId) -> Result<Vec<u8>, String> {
    // With event-driven output, reads are no longer needed
    // The backend pushes data via terminal-output events
    Ok(vec![])
}

/// Resize a terminal session.
#[tauri::command]
pub fn terminal_resize(
    state: State<AppState>,
    id: SessionId,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let session = manager.get(id).ok_or("Session not found")?;
    session
        .resize(TerminalSize { rows, cols })
        .map_err(|e| e.to_string())
}

/// Close a terminal session.
#[tauri::command]
pub fn terminal_close(state: State<AppState>, id: SessionId) -> Result<(), String> {
    logging::info("terminal", &format!("Closing PTY session {}", id));
    
    // Stop the reader thread
    if let Ok(mut readers) = get_readers().lock() {
        if let Some(running) = readers.remove(&id) {
            running.store(false, Ordering::Relaxed);
        }
    }
    
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    manager.remove(id);
    logging::debug("terminal", &format!("PTY session {} closed", id));
    Ok(())
}
