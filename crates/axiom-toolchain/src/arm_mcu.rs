// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! ARM MCU configuration and compiler flags.

use serde::{Deserialize, Serialize};

/// Floating-point ABI options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatAbi {
    /// Software floating-point emulation.
    Soft,
    /// Hardware floating-point with soft-float calling convention.
    SoftFp,
    /// Hardware floating-point with hard-float calling convention.
    Hard,
}

impl FloatAbi {
    /// Get the compiler flag for this ABI.
    pub fn as_flag(&self) -> &'static str {
        match self {
            FloatAbi::Soft => "soft",
            FloatAbi::SoftFp => "softfp",
            FloatAbi::Hard => "hard",
        }
    }
}

/// ARM MCU configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmMcuConfig {
    /// CPU type (e.g., "cortex-m3", "cortex-m4").
    pub cpu: String,
    /// Use Thumb instruction set.
    pub thumb: bool,
    /// FPU type (e.g., "fpv4-sp-d16", "fpv5-d16"), empty if no FPU.
    pub fpu: String,
    /// Floating-point ABI.
    pub float_abi: FloatAbi,
    /// Preprocessor defines (e.g., "STM32F103xB").
    pub defines: Vec<String>,
}

impl ArmMcuConfig {
    /// Create a new ARM MCU configuration.
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
    pub fn with_fpu(mut self, fpu: impl Into<String>) -> Self {
        self.fpu = fpu.into();
        self
    }
    
    /// Set the floating-point ABI.
    pub fn with_float_abi(mut self, abi: FloatAbi) -> Self {
        self.float_abi = abi;
        self
    }
    
    /// Add a preprocessor define.
    pub fn with_define(mut self, define: impl Into<String>) -> Self {
        self.defines.push(define.into());
        self
    }
    
    /// Generate compiler flags for this configuration.
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
    
    /// Generate linker flags for this configuration.
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
    pub fn cortex_m0() -> Self {
        Self::new("cortex-m0")
    }
    
    /// Create a Cortex-M3 configuration.
    pub fn cortex_m3() -> Self {
        Self::new("cortex-m3")
    }
    
    /// Create a Cortex-M4 configuration with FPU.
    pub fn cortex_m4() -> Self {
        Self::new("cortex-m4")
            .with_fpu("fpv4-sp-d16")
            .with_float_abi(FloatAbi::Hard)
    }
    
    /// Create a Cortex-M7 configuration with FPU.
    pub fn cortex_m7() -> Self {
        Self::new("cortex-m7")
            .with_fpu("fpv5-d16")
            .with_float_abi(FloatAbi::Hard)
    }
}

/// Linker configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkerConfig {
    /// Path to linker script.
    pub script: std::path::PathBuf,
    /// Generate map file.
    pub generate_map: bool,
    /// Path to map file (if generate_map is true).
    pub map_path: Option<std::path::PathBuf>,
    /// Additional linker flags.
    pub flags: Vec<String>,
}

impl LinkerConfig {
    /// Create a new linker configuration.
    pub fn new(script: impl Into<std::path::PathBuf>) -> Self {
        Self {
            script: script.into(),
            generate_map: false,
            map_path: None,
            flags: Vec::new(),
        }
    }
    
    /// Enable map file generation.
    pub fn with_map(mut self, path: impl Into<std::path::PathBuf>) -> Self {
        self.generate_map = true;
        self.map_path = Some(path.into());
        self
    }
    
    /// Add a linker flag.
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }
}

/// Validate linker configuration.
pub fn validate_linker_config(config: &LinkerConfig) -> Result<(), String> {
    if !config.script.exists() {
        return Err(format!("Linker script not found: {}", config.script.display()));
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
        let config = ArmMcuConfig::cortex_m3()
            .with_define("STM32H750xx");
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
        let linker = LinkerConfig::new("test.ld")
            .with_map("output.map");
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
