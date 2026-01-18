// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Settings schema definition.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Root settings structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Schema version for migrations.
    #[serde(default = "default_version")]
    pub version: u32,

    /// Toolchain settings.
    #[serde(default)]
    pub toolchains: ToolchainSettings,

    /// Build settings.
    #[serde(default)]
    pub build: BuildSettings,

    /// Editor settings.
    #[serde(default)]
    pub editor: EditorSettings,

    /// Assembly view settings.
    #[serde(default)]
    pub assembly: AssemblySettings,

    /// Debug settings.
    #[serde(default)]
    pub debug: DebugSettings,

    /// UI settings.
    #[serde(default)]
    pub ui: UiSettings,
}

fn default_version() -> u32 {
    crate::SCHEMA_VERSION
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: crate::SCHEMA_VERSION,
            toolchains: ToolchainSettings::default(),
            build: BuildSettings::default(),
            editor: EditorSettings::default(),
            assembly: AssemblySettings::default(),
            debug: DebugSettings::default(),
            ui: UiSettings::default(),
        }
    }
}

/// Toolchain configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ToolchainSettings {
    /// Path to clang binary.
    pub clang_path: Option<PathBuf>,

    /// Path to gcc binary.
    pub gcc_path: Option<PathBuf>,

    /// Path to arm-none-eabi-gcc binary.
    pub arm_gcc_path: Option<PathBuf>,

    /// Whether to auto-detect toolchains.
    #[serde(default = "default_true")]
    pub auto_detect: bool,
}

/// Build configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildSettings {
    /// Output directory for builds.
    #[serde(default = "default_build_dir")]
    pub output_dir: PathBuf,

    /// Optimization level (0-3).
    #[serde(default = "default_opt_level")]
    pub optimization_level: u8,

    /// Enable debug symbols.
    #[serde(default = "default_true")]
    pub debug_symbols: bool,
}

impl Default for BuildSettings {
    fn default() -> Self {
        Self {
            output_dir: default_build_dir(),
            optimization_level: default_opt_level(),
            debug_symbols: true,
        }
    }
}

fn default_build_dir() -> PathBuf {
    PathBuf::from("build")
}

fn default_opt_level() -> u8 {
    0
}

/// Editor configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditorSettings {
    /// Font size in pixels.
    #[serde(default = "default_font_size")]
    pub font_size: u32,

    /// Tab size in spaces.
    #[serde(default = "default_tab_size")]
    pub tab_size: u32,

    /// Font family.
    #[serde(default = "default_font_family")]
    pub font_family: String,

    /// Show line numbers.
    #[serde(default = "default_true")]
    pub line_numbers: bool,

    /// Enable word wrap.
    #[serde(default)]
    pub word_wrap: bool,

    /// Enable autocomplete.
    #[serde(default = "default_true")]
    pub autocomplete: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: default_font_size(),
            tab_size: default_tab_size(),
            font_family: default_font_family(),
            line_numbers: true,
            word_wrap: false,
            autocomplete: true,
        }
    }
}

fn default_font_size() -> u32 {
    14
}

fn default_tab_size() -> u32 {
    4
}

fn default_font_family() -> String {
    "JetBrains Mono, SF Mono, monospace".to_string()
}

/// Assembly view configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssemblySettings {
    /// Assembly syntax style.
    #[serde(default)]
    pub syntax: AssemblySyntax,

    /// Target architecture.
    #[serde(default)]
    pub architecture: Option<String>,
}

impl Default for AssemblySettings {
    fn default() -> Self {
        Self {
            syntax: AssemblySyntax::default(),
            architecture: None,
        }
    }
}

/// Assembly syntax style.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AssemblySyntax {
    /// Intel syntax (mov eax, 1).
    #[default]
    Intel,
    /// AT&T syntax (movl $1, %eax).
    Att,
}

/// Debug configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DebugSettings {
    /// Debug probe type.
    pub probe_type: Option<String>,

    /// Reset mode on connect.
    #[serde(default)]
    pub reset_on_connect: bool,
}

/// UI configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UiSettings {
    /// Theme name.
    #[serde(default = "default_theme")]
    pub theme: Theme,

    /// UI font size.
    #[serde(default = "default_ui_font_size")]
    pub font_size: u32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            font_size: default_ui_font_size(),
        }
    }
}

fn default_ui_font_size() -> u32 {
    13
}

/// UI theme.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    /// Dark theme (default).
    #[default]
    Dark,
    /// Light theme.
    Light,
}

fn default_theme() -> Theme {
    Theme::Dark
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.version, crate::SCHEMA_VERSION);
        assert_eq!(settings.ui.theme, Theme::Dark);
        assert_eq!(settings.editor.font_size, 14);
        assert!(settings.toolchains.auto_detect);
    }

    #[test]
    fn test_serialize_deserialize() {
        let settings = Settings::default();
        let toml = toml::to_string(&settings).unwrap();
        let parsed: Settings = toml::from_str(&toml).unwrap();
        assert_eq!(settings, parsed);
    }
}
