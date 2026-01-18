// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Symbol command handlers.

use crate::state::AppState;
use axiom_symbols::Symbol;
use std::path::PathBuf;
use tauri::State;

/// Get completions for a prefix.
///
/// Returns symbols matching the prefix, sorted deterministically:
/// by kind (types first), then alphabetically within kind.
#[tauri::command]
pub fn get_completions(state: State<AppState>, prefix: String) -> Result<Vec<Symbol>, String> {
    let index = state.symbol_index.lock().map_err(|e| e.to_string())?;
    let completions: Vec<Symbol> = index.complete(&prefix).into_iter().cloned().collect();
    Ok(completions)
}

/// Index a file for symbols.
#[tauri::command]
pub fn index_file(state: State<AppState>, path: String) -> Result<usize, String> {
    let path_buf = PathBuf::from(&path);

    // Parse the file first
    let mut parser = state.parser.lock().map_err(|e| e.to_string())?;
    let ast = parser.parse_file(&path_buf).map_err(|e| e.to_string())?;

    // Index the symbols
    let mut index = state.symbol_index.lock().map_err(|e| e.to_string())?;
    index.index_file(path_buf, &ast);

    Ok(index.len())
}
