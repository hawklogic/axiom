// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Terminal command handlers.

use crate::logging;
use crate::state::AppState;
use axiom_terminal::{SessionId, TerminalSize};
use tauri::State;

/// Create a new terminal session.
#[tauri::command]
pub fn terminal_create(state: State<AppState>) -> Result<SessionId, String> {
    logging::info("terminal", "Creating new PTY session");
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let id = manager.create_session().map_err(|e| {
        logging::error("terminal", &format!("Failed to create session: {}", e));
        e.to_string()
    })?;
    logging::info("terminal", &format!("PTY session {} created", id));
    Ok(id)
}

/// Write to a terminal session.
#[tauri::command]
pub fn terminal_write(state: State<AppState>, id: SessionId, data: String) -> Result<usize, String> {
    let manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let session = manager.get(id).ok_or("Session not found")?;
    session.write(data.as_bytes()).map_err(|e| e.to_string())
}

/// Read from a terminal session.
#[tauri::command]
pub fn terminal_read(state: State<AppState>, id: SessionId) -> Result<Vec<u8>, String> {
    let manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let session = manager.get(id).ok_or("Session not found")?;
    
    let mut buf = vec![0u8; 4096];
    match session.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            Ok(buf)
        }
        Err(e) => {
            // Return empty on would-block (no data available)
            if e.to_string().contains("would block") {
                Ok(vec![])
            } else {
                Err(e.to_string())
            }
        }
    }
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
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    manager.remove(id);
    logging::debug("terminal", &format!("PTY session {} closed", id));
    Ok(())
}
