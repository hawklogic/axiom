// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Settings persistence to TOML files.

use crate::{migrate, Settings};
use std::fs;
use std::path::{Path, PathBuf};

/// Error type for persistence operations.
#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] crate::MigrationError),
}

/// Get the default settings file path.
pub fn default_settings_path() -> PathBuf {
    if cfg!(target_os = "macos") {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("com.hawklogic.axiom")
            .join("settings.toml")
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("axiom")
            .join("settings.toml")
    }
}

/// Load settings from a file.
///
/// If the file doesn't exist, returns default settings.
/// If the file exists but has an older schema, migrates automatically.
pub fn load(path: &Path) -> Result<Settings, PersistenceError> {
    if !path.exists() {
        return Ok(Settings::default());
    }

    let content = fs::read_to_string(path)?;
    let settings: Settings = toml::from_str(&content)?;
    let result = migrate(settings)?;

    // If migration occurred, save the migrated settings
    if result.migrated {
        save(path, &result.settings)?;
    }

    Ok(result.settings)
}

/// Save settings to a file.
///
/// Creates parent directories if they don't exist.
pub fn save(path: &Path, settings: &Settings) -> Result<(), PersistenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = toml::to_string_pretty(settings)?;
    fs::write(path, content)?;
    Ok(())
}

/// Load settings from the default path.
pub fn load_default() -> Result<Settings, PersistenceError> {
    load(&default_settings_path())
}

/// Save settings to the default path.
pub fn save_default(settings: &Settings) -> Result<(), PersistenceError> {
    save(&default_settings_path(), settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_nonexistent_returns_default() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("settings.toml");
        let settings = load(&path).unwrap();
        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn test_save_and_load() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("settings.toml");

        let mut settings = Settings::default();
        settings.editor.font_size = 18;

        save(&path, &settings).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.editor.font_size, 18);
    }

    #[test]
    fn test_creates_parent_dirs() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nested").join("deep").join("settings.toml");

        let settings = Settings::default();
        save(&path, &settings).unwrap();

        assert!(path.exists());
    }
}
