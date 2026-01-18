// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Application state management.

use axiom_parser::Parser;
use axiom_settings::Settings;
use axiom_symbols::SymbolIndex;
use axiom_terminal::SessionManager;
use axiom_toolchain::DetectedToolchain;
use std::path::PathBuf;
use std::sync::Mutex;

/// Global application state.
pub struct AppState {
    /// Current settings.
    pub settings: Mutex<Settings>,
    /// Detected toolchains.
    pub toolchains: Mutex<Vec<DetectedToolchain>>,
    /// Parser instance.
    pub parser: Mutex<Parser>,
    /// Symbol index.
    pub symbol_index: Mutex<SymbolIndex>,
    /// Terminal session manager.
    pub terminal_manager: Mutex<SessionManager>,
    /// Current project path.
    #[allow(dead_code)]
    pub project_path: Mutex<Option<PathBuf>>,
}

impl AppState {
    /// Create new application state.
    pub fn new() -> Self {
        // Load settings from default path
        let settings = axiom_settings::load_default().unwrap_or_default();

        // Detect toolchains
        let toolchains = axiom_toolchain::detect_all();

        // Create parser
        let parser = Parser::default();

        Self {
            settings: Mutex::new(settings),
            toolchains: Mutex::new(toolchains),
            parser: Mutex::new(parser),
            symbol_index: Mutex::new(SymbolIndex::new()),
            terminal_manager: Mutex::new(SessionManager::new()),
            project_path: Mutex::new(None),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
