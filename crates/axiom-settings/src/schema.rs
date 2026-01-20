// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Settings schema definition.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    /// Compliance settings.
    #[serde(default)]
    pub compliance: ComplianceSettings,
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
            compliance: ComplianceSettings::default(),
        }
    }
}

/// Toolchain configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    /// Generic toolchain configurations keyed by toolchain type.
    #[serde(default)]
    pub toolchains: HashMap<String, ToolchainConfig>,
}

impl Default for ToolchainSettings {
    fn default() -> Self {
        Self {
            clang_path: None,
            gcc_path: None,
            arm_gcc_path: None,
            auto_detect: true,
            toolchains: HashMap::new(),
        }
    }
}

/// Generic toolchain configuration for extensibility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolchainConfig {
    /// Path to primary toolchain binary.
    pub path: Option<PathBuf>,

    /// Additional search paths for toolchain detection.
    #[serde(default)]
    pub search_paths: Vec<PathBuf>,

    /// Toolchain-specific settings (extensible).
    #[serde(default)]
    pub settings: HashMap<String, toml::Value>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AssemblySettings {
    /// Assembly syntax style.
    #[serde(default)]
    pub syntax: AssemblySyntax,

    /// Target architecture.
    #[serde(default)]
    pub architecture: Option<String>,
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

/// ARM-specific toolchain settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ArmToolchainSettings {
    /// Target MCU (e.g., "cortex-m7", "cortex-m4").
    pub mcu: Option<String>,

    /// FPU type (e.g., "fpv5-d16", "fpv4-sp-d16").
    pub fpu: Option<String>,

    /// Float ABI (hard, soft, softfp).
    pub float_abi: Option<String>,

    /// Default linker script path.
    pub linker_script: Option<PathBuf>,

    /// Default include paths.
    #[serde(default)]
    pub include_paths: Vec<PathBuf>,

    /// Default preprocessor defines.
    #[serde(default)]
    pub defines: Vec<String>,
}

/// Compliance mode settings for safety-critical development.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ComplianceSettings {
    /// DO-178C mode enabled (software airworthiness).
    #[serde(default)]
    pub do178c_enabled: bool,

    /// DO-330 mode enabled (tool qualification).
    #[serde(default)]
    pub do330_enabled: bool,

    /// ARP4754A mode enabled (system safety).
    #[serde(default)]
    pub arp4754a_enabled: bool,

    /// Design Assurance Level (A-E for DO-178C).
    pub dal: Option<String>,
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

    #[test]
    fn test_arm_toolchain_settings_default() {
        let settings = ArmToolchainSettings::default();
        assert!(settings.mcu.is_none());
        assert!(settings.fpu.is_none());
        assert!(settings.float_abi.is_none());
        assert!(settings.linker_script.is_none());
        assert!(settings.include_paths.is_empty());
        assert!(settings.defines.is_empty());
    }

    #[test]
    fn test_settings_serialization_roundtrip() {
        let mut settings = Settings::default();
        settings.toolchains.toolchains.insert(
            "arm".to_string(),
            ToolchainConfig {
                path: Some(PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc")),
                search_paths: vec![PathBuf::from("/opt/arm")],
                settings: {
                    let mut map = HashMap::new();
                    map.insert("mcu".to_string(), toml::Value::String("cortex-m7".to_string()));
                    map
                },
            },
        );

        let toml_str = toml::to_string(&settings).unwrap();
        let parsed: Settings = toml::from_str(&toml_str).unwrap();
        assert_eq!(settings, parsed);
    }

    #[test]
    fn test_unknown_toolchain_preserved() {
        let toml_str = r#"
            version = 1
            
            [toolchains]
            auto_detect = true
            
            [toolchains.toolchains.riscv]
            path = "/opt/riscv/bin/riscv-gcc"
            search_paths = []
            
            [toolchains.toolchains.riscv.settings]
            arch = "rv32imac"
        "#;

        let settings: Settings = toml::from_str(toml_str).unwrap();
        assert!(settings.toolchains.toolchains.contains_key("riscv"));
        
        // Verify it survives roundtrip
        let serialized = toml::to_string(&settings).unwrap();
        let parsed: Settings = toml::from_str(&serialized).unwrap();
        assert!(parsed.toolchains.toolchains.contains_key("riscv"));
        assert_eq!(
            parsed.toolchains.toolchains.get("riscv").unwrap().path,
            Some(PathBuf::from("/opt/riscv/bin/riscv-gcc"))
        );
    }

    // Property-based tests
    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        // Strategy for generating arbitrary ToolchainSettings
        fn arb_toolchain_settings() -> impl Strategy<Value = ToolchainSettings> {
            (
                any::<bool>(),
                prop::option::of(any::<String>().prop_map(PathBuf::from)),
                prop::option::of(any::<String>().prop_map(PathBuf::from)),
                prop::option::of(any::<String>().prop_map(PathBuf::from)),
            )
                .prop_map(|(auto_detect, clang_path, gcc_path, arm_gcc_path)| {
                    ToolchainSettings {
                        clang_path,
                        gcc_path,
                        arm_gcc_path,
                        auto_detect,
                        toolchains: HashMap::new(),
                    }
                })
        }

        proptest! {
            #[test]
            fn prop_toolchain_settings_roundtrip(settings in arb_toolchain_settings()) {
                // Serialize to TOML
                let toml_str = toml::to_string(&settings).unwrap();
                
                // Deserialize back
                let parsed: ToolchainSettings = toml::from_str(&toml_str).unwrap();
                
                // Should be identical
                prop_assert_eq!(settings, parsed);
            }
        }
    }
}
