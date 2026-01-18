// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Parser command handlers.

use crate::state::AppState;
use axiom_parser::{AstNode, Language};
use std::path::PathBuf;
use tauri::State;

/// Parse a file and return the AST.
#[tauri::command]
pub fn parse_file(state: State<AppState>, path: String) -> Result<AstNode, String> {
    let path = PathBuf::from(&path);

    let mut parser = state.parser.lock().map_err(|e| e.to_string())?;
    let ast = parser.parse_file(&path).map_err(|e| e.to_string())?;

    // Also index the file for symbols
    let mut index = state.symbol_index.lock().map_err(|e| e.to_string())?;
    index.index_file(path, &ast);

    Ok(ast)
}

/// Get AST for source code directly.
#[tauri::command]
pub fn get_ast(state: State<AppState>, source: String, language: String) -> Result<AstNode, String> {
    let lang = match language.to_lowercase().as_str() {
        "c" => Language::C,
        "cpp" | "c++" => Language::Cpp,
        _ => return Err(format!("Unsupported language: {}", language)),
    };

    let mut parser = state.parser.lock().map_err(|e| e.to_string())?;
    parser.parse(&source, lang).map_err(|e| e.to_string())
}
