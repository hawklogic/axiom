// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom IDE - Main Entry Point
//!
//! Deterministic. Inspectable. Offline.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
pub mod logging;
mod state;

use state::AppState;
use tauri::{Emitter, Manager};
use tauri::menu::{AboutMetadataBuilder, MenuBuilder, SubmenuBuilder};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new())  // Register state BEFORE setup
        .setup(|app| {
            // Initialize logging with app handle for event emission
            logging::init(app.handle().clone());
            logging::info("core", "Axiom IDE starting...");
            println!("[Axiom] Setup hook started");
            
            // Log initial state
            let state = app.state::<AppState>();
            let toolchains = state.toolchains.lock().unwrap();
            logging::info("toolchain", &format!("Detected {} toolchain(s)", toolchains.len()));
            for tc in toolchains.iter() {
                logging::debug("toolchain", &format!("  {:?} at {}", tc.kind, tc.path.display()));
            }
            drop(toolchains);
            
            // Build the application menu
            let about_metadata = AboutMetadataBuilder::new()
                .name(Some("Axiom"))
                .version(Some("0.1.0"))
                .copyright(Some("Copyright © 2024 HawkLogic Systems"))
                .comments(Some("Avionics-grade IDE for embedded systems.\nDeterministic. Inspectable. Offline."))
                .website(Some("https://github.com/hawklogic/axiom"))
                .build();
            
            let app_menu = SubmenuBuilder::new(app, "Axiom")
                .about(Some(about_metadata))
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .quit()
                .build()?;
            
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open-folder", "Open Folder...")
                .separator()
                .close_window()
                .build()?;
            
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .undo()
                .redo()
                .separator()
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;
            
            let view_menu = SubmenuBuilder::new(app, "View")
                .text("toggle-terminal", "Toggle Terminal")
                .text("toggle-sidebar", "Toggle Sidebar")
                .separator()
                .fullscreen()
                .build()?;
            
            let window_menu = SubmenuBuilder::new(app, "Window")
                .minimize()
                .separator()
                .close_window()
                .build()?;
            
            let help_menu = SubmenuBuilder::new(app, "Help")
                .text("documentation", "Documentation")
                .text("report-issue", "Report Issue")
                .separator()
                .text("about-axiom", "About Axiom")
                .build()?;
            
            let menu = MenuBuilder::new(app)
                .item(&app_menu)
                .item(&file_menu)
                .item(&edit_menu)
                .item(&view_menu)
                .item(&window_menu)
                .item(&help_menu)
                .build()?;
            
            app.set_menu(menu)?;
            
            // Handle menu events
            app.on_menu_event(|app, event| {
                match event.id().as_ref() {
                    "about-axiom" => {
                        // Emit event to frontend to show about dialog
                        let _ = app.emit("show-about", ());
                    }
                    "documentation" => {
                        let _ = tauri::async_runtime::spawn(async {
                            let _ = open::that("https://github.com/hawklogic/axiom");
                        });
                    }
                    "report-issue" => {
                        let _ = tauri::async_runtime::spawn(async {
                            let _ = open::that("https://github.com/hawklogic/axiom/issues");
                        });
                    }
                    _ => {}
                }
            });
            
            logging::info("core", "Backend ready");
            println!("[Axiom] Setup hook completed, backend ready");
            Ok(())
        })
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
            commands::git::git_file_diff,
            commands::git::git_stage,
            commands::git::git_unstage,
            commands::git::git_commit,
            commands::git::git_branch,
            commands::git::git_push,
            commands::git::git_pull,
            commands::git::git_last_commit,
            commands::git::git_remote_status,
            commands::git::git_log,
            commands::git::git_commit_files,
            // Terminal commands
            commands::terminal::terminal_create,
            commands::terminal::terminal_write,
            commands::terminal::terminal_read,
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
