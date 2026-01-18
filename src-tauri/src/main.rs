// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom IDE - Main Entry Point
//!
//! Deterministic. Inspectable. Offline.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // Settings commands
            commands::settings::get_settings,
            commands::settings::set_settings,
            commands::settings::reset_settings,
            // Toolchain commands
            commands::toolchain::detect_toolchains,
            commands::toolchain::get_toolchains,
            commands::toolchain::compile_file,
            commands::toolchain::compile_dry_run,
            // Parser commands
            commands::parser::parse_file,
            commands::parser::get_ast,
            // Symbol commands
            commands::symbols::get_completions,
            commands::symbols::index_file,
            // Git commands
            commands::git::git_status,
            commands::git::git_diff,
            commands::git::git_stage,
            commands::git::git_unstage,
            commands::git::git_commit,
            commands::git::git_branch,
            // Terminal commands
            commands::terminal::terminal_create,
            commands::terminal::terminal_write,
            commands::terminal::terminal_resize,
            commands::terminal::terminal_close,
            // Filesystem commands
            commands::fs::read_dir,
            commands::fs::read_file,
            commands::fs::write_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Axiom");
}
