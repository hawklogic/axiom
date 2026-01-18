// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Settings command handlers.

use crate::state::AppState;
use axiom_settings::Settings;
use tauri::State;

/// Get current settings.
#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<Settings, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(settings.clone())
}

/// Update settings.
#[tauri::command]
pub fn set_settings(state: State<AppState>, settings: Settings) -> Result<(), String> {
    // Validate and save
    axiom_settings::save_default(&settings).map_err(|e| e.to_string())?;

    // Update state
    let mut current = state.settings.lock().map_err(|e| e.to_string())?;
    *current = settings;

    Ok(())
}

/// Reset settings to defaults.
#[tauri::command]
pub fn reset_settings(state: State<AppState>) -> Result<Settings, String> {
    let default_settings = Settings::default();
    axiom_settings::save_default(&default_settings).map_err(|e| e.to_string())?;

    let mut current = state.settings.lock().map_err(|e| e.to_string())?;
    *current = default_settings.clone();

    Ok(default_settings)
}
