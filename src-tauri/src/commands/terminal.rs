// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Terminal command handlers.

use crate::state::AppState;
use axiom_terminal::{SessionId, TerminalSize};
use tauri::State;

/// Create a new terminal session.
#[tauri::command]
pub fn terminal_create(state: State<AppState>) -> Result<SessionId, String> {
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    manager.create_session().map_err(|e| e.to_string())
}

/// Write to a terminal session.
#[tauri::command]
pub fn terminal_write(state: State<AppState>, id: SessionId, data: String) -> Result<usize, String> {
    let manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    let session = manager.get(id).ok_or("Session not found")?;
    session.write(data.as_bytes()).map_err(|e| e.to_string())
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
    let mut manager = state.terminal_manager.lock().map_err(|e| e.to_string())?;
    manager.remove(id);
    Ok(())
}
