// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! ARM MCU configuration and compiler flag generation.
//!
//! This module provides configuration structures and helper functions for ARM Cortex-M
//! microcontrollers. It handles the generation of MCU-specific compiler and linker flags
//! including CPU type, FPU configuration, and floating-point ABI settings.
//!
//! # Example
//!
//! ```no_run
//! use axiom_toolchain::{ArmMcuConfig, FloatAbi, LinkerConfig};
//!
//! // Create a Cortex-M4 configuration with FPU
//! let mcu = ArmMcuConfig::cortex_m4()
//!     .with_define("STM32F407xx")
//!     .with_define("USE_HAL_DRIVER");
//!
//! // Generate compiler flags
//! let compiler_flags = mcu.compiler_flags();
//! // Results in: ["-mcpu=cortex-m4", "-mthumb", "-mfpu=fpv4-sp-d16",
//! //              "-mfloat-abi=hard", "-DSTM32F407xx", "-DUSE_HAL_DRIVER"]
//!
//! // Create linker configuration
//! let linker = LinkerConfig::new("STM32F407.ld")
//!     .with_map("output.map");
//!
//! // Generate linker flags
//! let linker_flags = mcu.linker_flags(&linker);
//! ```

use serde::{Deserialize, Serialize};

/// Floating-point ABI (Application Binary Interface) options.
///
/// Determines how floating-point values are passed between functions and
/// whether hardware FPU instructions are used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatAbi {
    /// Software floating-point emulation only.
    ///
    /// All floating-point operations are emulated in software. No FPU required.
    /// Slowest but most compatible option.
    Soft,

    /// Hardware FPU with software calling convention.
    ///
    /// FPU instructions are used for calculations, but floating-point values
    /// are passed in integer registers. Provides compatibility with soft-float code.
    SoftFp,

    /// Hardware FPU with hardware calling convention.
    ///
    /// FPU instructions are used and floating-point values are passed in FPU registers.
    /// Fastest option but requires FPU and all code must be compiled with hard-float.
    Hard,
}

impl FloatAbi {
    /// Get the compiler flag string for this ABI.
    ///
    /// Returns the value to use with the `-mfloat-abi=` compiler flag.
    pub fn as_flag(&self) -> &'static str {
        match self {
            FloatAbi::Soft => "soft",
            FloatAbi::SoftFp => "softfp",
            FloatAbi::Hard => "hard",
        }
    }
}

/// ARM MCU configuration for compilation and linking.
///
/// Encapsulates all MCU-specific settings needed to compile code for ARM Cortex-M
/// microcontrollers, including CPU type, FPU configuration, and preprocessor defines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmMcuConfig {
    /// CPU type (e.g., "cortex-m3", "cortex-m4", "cortex-m7").
    pub cpu: String,

    /// Use Thumb instruction set (should always be true for Cortex-M).
    pub thumb: bool,

    /// FPU type (e.g., "fpv4-sp-d16", "fpv5-d16"), empty string if no FPU.
    pub fpu: String,

    /// Floating-point ABI selection.
    pub float_abi: FloatAbi,

    /// Preprocessor defines (e.g., "STM32F103xB", "USE_HAL_DRIVER").
    pub defines: Vec<String>,
}

impl ArmMcuConfig {
    /// Create a new ARM MCU configuration with the specified CPU type.
    ///
    /// Default settings: Thumb mode enabled, no FPU, soft float ABI, no defines.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use axiom_toolchain::ArmMcuConfig;
    ///
    /// let mcu = ArmMcuConfig::new("cortex-m3");
    /// ```
    pub fn new(cpu: impl Into<String>) -> Self {
        Self {
            cpu: cpu.into(),
            thumb: true,
            fpu: String::new(),
            float_abi: FloatAbi::Soft,
            defines: Vec::new(),
        }
    }

    /// Set the FPU type.
    ///
    /// Common FPU types:
    /// - `"fpv4-sp-d16"` - Single-precision FPU (Cortex-M4)
    /// - `"fpv5-d16"` - Double-precision FPU (Cortex-M7)
    pub fn with_fpu(mut self, fpu: impl Into<String>) -> Self {
        self.fpu = fpu.into();
        self
    }

    /// Set the floating-point ABI.
    ///
    /// Should match the FPU configuration:
    /// - Use `FloatAbi::Soft` when no FPU is present
    /// - Use `FloatAbi::Hard` when FPU is present for best performance
    pub fn with_float_abi(mut self, abi: FloatAbi) -> Self {
        self.float_abi = abi;
        self
    }

    /// Add a preprocessor define.
    ///
    /// Defines are passed to the compiler with `-D` prefix.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use axiom_toolchain::ArmMcuConfig;
    /// let mcu = ArmMcuConfig::cortex_m4()
    ///     .with_define("STM32F407xx")
    ///     .with_define("USE_HAL_DRIVER");
    /// ```
    pub fn with_define(mut self, define: impl Into<String>) -> Self {
        self.defines.push(define.into());
        self
    }

    /// Generate compiler flags for this MCU configuration.
    ///
    /// Returns a vector of flags suitable for passing to arm-none-eabi-gcc.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use axiom_toolchain::ArmMcuConfig;
    /// let mcu = ArmMcuConfig::cortex_m4();
    /// let flags = mcu.compiler_flags();
    /// // flags = ["-mcpu=cortex-m4", "-mthumb", "-mfpu=fpv4-sp-d16", "-mfloat-abi=hard"]
    /// ```
    pub fn compiler_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();

        // CPU type
        flags.push(format!("-mcpu={}", self.cpu));

        // Thumb mode
        if self.thumb {
            flags.push("-mthumb".to_string());
        }

        // FPU
        if !self.fpu.is_empty() {
            flags.push(format!("-mfpu={}", self.fpu));
        }

        // Float ABI
        flags.push(format!("-mfloat-abi={}", self.float_abi.as_flag()));

        // Defines
        for define in &self.defines {
            flags.push(format!("-D{}", define));
        }

        flags
    }

    /// Generate linker flags for this MCU configuration.
    ///
    /// Combines MCU-specific flags with linker configuration to produce
    /// the complete set of linker flags.
    ///
    /// # Arguments
    ///
    /// * `linker_config` - Linker configuration with script path and options
    ///
    /// # Returns
    ///
    /// Vector of flags suitable for passing to arm-none-eabi-gcc during linking
    pub fn linker_flags(&self, linker_config: &LinkerConfig) -> Vec<String> {
        let mut flags = Vec::new();

        // CPU type
        flags.push(format!("-mcpu={}", self.cpu));

        // Thumb mode
        if self.thumb {
            flags.push("-mthumb".to_string());
        }

        // FPU
        if !self.fpu.is_empty() {
            flags.push(format!("-mfpu={}", self.fpu));
        }

        // Float ABI
        flags.push(format!("-mfloat-abi={}", self.float_abi.as_flag()));

        // Linker script
        flags.push(format!("-T{}", linker_config.script.display()));

        // Map file
        if linker_config.generate_map {
            if let Some(map_path) = &linker_config.map_path {
                flags.push(format!("-Wl,-Map={}", map_path.display()));
            }
        }

        // Garbage collection
        flags.push("-Wl,--gc-sections".to_string());

        // Additional flags
        for flag in &linker_config.flags {
            if flag.starts_with("-Wl,") {
                flags.push(flag.clone());
            } else {
                flags.push(format!("-Wl,{}", flag));
            }
        }

        flags
    }

    /// Create a Cortex-M0 configuration.
    ///
    /// Cortex-M0 has no FPU and uses software floating-point.
    pub fn cortex_m0() -> Self {
        Self::new("cortex-m0")
    }

    /// Create a Cortex-M3 configuration.
    ///
    /// Cortex-M3 has no FPU and uses software floating-point.
    pub fn cortex_m3() -> Self {
        Self::new("cortex-m3")
    }

    /// Create a Cortex-M4 configuration with FPU.
    ///
    /// Cortex-M4 has a single-precision FPU (fpv4-sp-d16) and uses hard-float ABI.
    pub fn cortex_m4() -> Self {
        Self::new("cortex-m4")
            .with_fpu("fpv4-sp-d16")
            .with_float_abi(FloatAbi::Hard)
    }

    /// Create a Cortex-M7 configuration with FPU.
    ///
    /// Cortex-M7 has a double-precision FPU (fpv5-d16) and uses hard-float ABI.
    pub fn cortex_m7() -> Self {
        Self::new("cortex-m7")
            .with_fpu("fpv5-d16")
            .with_float_abi(FloatAbi::Hard)
    }
}

/// Linker configuration for ARM projects.
///
/// Specifies the linker script, map file generation, and additional linker flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkerConfig {
    /// Path to linker script (.ld file).
    pub script: std::path::PathBuf,

    /// Generate memory map file.
    pub generate_map: bool,

    /// Path to map file (if generate_map is true).
    pub map_path: Option<std::path::PathBuf>,

    /// Additional linker flags.
    pub flags: Vec<String>,
}

impl LinkerConfig {
    /// Create a new linker configuration with the specified script.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use axiom_toolchain::LinkerConfig;
    ///
    /// let linker = LinkerConfig::new("STM32F407.ld");
    /// ```
    pub fn new(script: impl Into<std::path::PathBuf>) -> Self {
        Self {
            script: script.into(),
            generate_map: false,
            map_path: None,
            flags: Vec::new(),
        }
    }

    /// Enable map file generation with the specified path.
    ///
    /// The map file contains detailed information about symbol addresses
    /// and memory usage, useful for debugging and optimization.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use axiom_toolchain::LinkerConfig;
    /// let linker = LinkerConfig::new("STM32F407.ld")
    ///     .with_map("firmware.map");
    /// ```
    pub fn with_map(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.generate_map = true;
        self.map_path = Some(path.into());
        self
    }

    /// Add a custom linker flag.
    ///
    /// Flags are passed to the linker. If they don't start with `-Wl,`,
    /// that prefix will be added automatically.
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }
}

/// Validate linker configuration.
///
/// Checks that the linker script file exists.
///
/// # Errors
///
/// Returns an error if the linker script file does not exist.
pub fn validate_linker_config(config: &LinkerConfig) -> Result<(), String> {
    if !config.script.exists() {
        return Err(format!(
            "Linker script not found: {}",
            config.script.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_flags_includes_cpu() {
        let config = ArmMcuConfig::cortex_m3();
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mcpu=cortex-m3".to_string()));
    }

    #[test]
    fn test_compiler_flags_includes_thumb() {
        let config = ArmMcuConfig::cortex_m3();
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mthumb".to_string()));
    }

    #[test]
    fn test_compiler_flags_includes_fpu() {
        let config = ArmMcuConfig::cortex_m4();
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mfpu=fpv4-sp-d16".to_string()));
    }

    #[test]
    fn test_compiler_flags_includes_float_abi_hard() {
        let config = ArmMcuConfig::cortex_m4();
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mfloat-abi=hard".to_string()));
    }

    #[test]
    fn test_compiler_flags_includes_defines() {
        let config = ArmMcuConfig::cortex_m3().with_define("STM32H750xx");
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-DSTM32H750xx".to_string()));
    }

    #[test]
    fn test_linker_flags_includes_script() {
        let config = ArmMcuConfig::cortex_m3();
        let linker = LinkerConfig::new("test.ld");
        let flags = config.linker_flags(&linker);
        assert!(flags.iter().any(|f| f.starts_with("-Ttest.ld")));
    }

    #[test]
    fn test_linker_flags_includes_map() {
        let config = ArmMcuConfig::cortex_m3();
        let linker = LinkerConfig::new("test.ld").with_map("output.map");
        let flags = config.linker_flags(&linker);
        assert!(flags.iter().any(|f| f.contains("-Map=output.map")));
    }

    #[test]
    fn test_linker_flags_includes_gc_sections() {
        let config = ArmMcuConfig::cortex_m3();
        let linker = LinkerConfig::new("test.ld");
        let flags = config.linker_flags(&linker);
        assert!(flags.contains(&"-Wl,--gc-sections".to_string()));
    }

    #[test]
    fn test_validate_linker_config_nonexistent() {
        let linker = LinkerConfig::new("/nonexistent/script.ld");
        let result = validate_linker_config(&linker);
        assert!(result.is_err());
    }
}
