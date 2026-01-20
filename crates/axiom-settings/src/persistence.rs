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

/// Load project-specific settings from .axiom/toolchain.toml.
///
/// Returns None if the project settings file doesn't exist.
pub fn load_project_settings(project_path: &Path) -> Result<Option<Settings>, PersistenceError> {
    let settings_path = project_path.join(".axiom").join("toolchain.toml");

    if !settings_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&settings_path)?;
    let settings: Settings = toml::from_str(&content)?;
    Ok(Some(settings))
}

/// Merge project settings with global settings.
///
/// Project settings take precedence over global settings.
/// For toolchain configurations, project settings override global settings per toolchain type.
pub fn merge_settings(global: Settings, project: Option<Settings>) -> Settings {
    let Some(project) = project else {
        return global;
    };

    Settings {
        version: global.version,
        toolchains: merge_toolchain_settings(global.toolchains, project.toolchains),
        build: merge_build_settings(global.build, project.build),
        editor: merge_editor_settings(global.editor, project.editor),
        assembly: merge_assembly_settings(global.assembly, project.assembly),
        debug: merge_debug_settings(global.debug, project.debug),
        ui: merge_ui_settings(global.ui, project.ui),
        compliance: merge_compliance_settings(global.compliance, project.compliance),
    }
}

fn merge_toolchain_settings(
    mut global: crate::ToolchainSettings,
    project: crate::ToolchainSettings,
) -> crate::ToolchainSettings {
    // Project auto_detect overrides global
    let auto_detect = project.auto_detect;

    // Project paths override global paths if specified
    let clang_path = project.clang_path.or(global.clang_path);
    let gcc_path = project.gcc_path.or(global.gcc_path);
    let arm_gcc_path = project.arm_gcc_path.or(global.arm_gcc_path);

    // Merge toolchain configurations - project overrides global per toolchain type
    for (key, value) in project.toolchains {
        global.toolchains.insert(key, value);
    }

    crate::ToolchainSettings {
        clang_path,
        gcc_path,
        arm_gcc_path,
        auto_detect,
        toolchains: global.toolchains,
    }
}

fn merge_build_settings(
    global: crate::BuildSettings,
    project: crate::BuildSettings,
) -> crate::BuildSettings {
    crate::BuildSettings {
        output_dir: if project.output_dir.as_os_str() != "build" {
            project.output_dir
        } else {
            global.output_dir
        },
        optimization_level: project.optimization_level,
        debug_symbols: project.debug_symbols,
    }
}

fn merge_editor_settings(
    global: crate::EditorSettings,
    _project: crate::EditorSettings,
) -> crate::EditorSettings {
    // Editor settings are typically global-only
    global
}

fn merge_assembly_settings(
    global: crate::AssemblySettings,
    project: crate::AssemblySettings,
) -> crate::AssemblySettings {
    crate::AssemblySettings {
        syntax: project.syntax,
        architecture: project.architecture.or(global.architecture),
    }
}

fn merge_debug_settings(
    global: crate::DebugSettings,
    project: crate::DebugSettings,
) -> crate::DebugSettings {
    crate::DebugSettings {
        probe_type: project.probe_type.or(global.probe_type),
        reset_on_connect: project.reset_on_connect,
    }
}

fn merge_ui_settings(global: crate::UiSettings, _project: crate::UiSettings) -> crate::UiSettings {
    // UI settings are typically global-only
    global
}

fn merge_compliance_settings(
    global: crate::ComplianceSettings,
    project: crate::ComplianceSettings,
) -> crate::ComplianceSettings {
    crate::ComplianceSettings {
        do178c_enabled: project.do178c_enabled || global.do178c_enabled,
        do330_enabled: project.do330_enabled || global.do330_enabled,
        arp4754a_enabled: project.arp4754a_enabled || global.arp4754a_enabled,
        dal: project.dal.or(global.dal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
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

    #[test]
    fn test_merge_settings_project_path_overrides_global() {
        let mut global = Settings::default();
        global.toolchains.arm_gcc_path = Some(PathBuf::from("/usr/bin/arm-none-eabi-gcc"));

        let mut project = Settings::default();
        project.toolchains.arm_gcc_path = Some(PathBuf::from("/opt/arm/bin/arm-none-eabi-gcc"));

        let merged = merge_settings(global, Some(project));
        assert_eq!(
            merged.toolchains.arm_gcc_path,
            Some(PathBuf::from("/opt/arm/bin/arm-none-eabi-gcc"))
        );
    }

    #[test]
    fn test_merge_settings_project_auto_detect_overrides_global() {
        let mut global = Settings::default();
        global.toolchains.auto_detect = true;

        let mut project = Settings::default();
        project.toolchains.auto_detect = false;

        let merged = merge_settings(global, Some(project));
        assert!(!merged.toolchains.auto_detect);
    }

    #[test]
    fn test_load_project_settings_from_reference_project() {
        // Test loading from the reference project
        let project_path = PathBuf::from("tests/fixtures/arm-reference-project");

        if project_path.exists() {
            let result = load_project_settings(&project_path);
            assert!(result.is_ok());

            if let Ok(Some(settings)) = result {
                // Verify the settings were loaded
                assert!(
                    settings.toolchains.toolchains.contains_key("arm")
                        || !settings.toolchains.toolchains.is_empty()
                );
            }
        }
    }

    #[test]
    fn test_merge_with_global_settings() {
        let mut global = Settings::default();
        global.toolchains.arm_gcc_path = Some(PathBuf::from("/usr/bin/arm-none-eabi-gcc"));
        global.toolchains.auto_detect = true;
        global.toolchains.toolchains.insert(
            "arm".to_string(),
            crate::ToolchainConfig {
                path: Some(PathBuf::from("/usr/bin/arm-none-eabi-gcc")),
                search_paths: vec![],
                settings: HashMap::new(),
            },
        );

        let mut project = Settings::default();
        project.toolchains.toolchains.insert(
            "arm".to_string(),
            crate::ToolchainConfig {
                path: Some(PathBuf::from("/opt/arm/bin/arm-none-eabi-gcc")),
                search_paths: vec![PathBuf::from("/opt/arm")],
                settings: HashMap::new(),
            },
        );

        let merged = merge_settings(global, Some(project));

        // Project ARM toolchain should override global
        assert_eq!(
            merged.toolchains.toolchains.get("arm").unwrap().path,
            Some(PathBuf::from("/opt/arm/bin/arm-none-eabi-gcc"))
        );
        assert_eq!(
            merged
                .toolchains
                .toolchains
                .get("arm")
                .unwrap()
                .search_paths,
            vec![PathBuf::from("/opt/arm")]
        );
    }
}
