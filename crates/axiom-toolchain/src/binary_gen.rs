// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Binary generation from ELF files.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Binary output configuration.
#[derive(Debug, Clone)]
pub struct BinaryOutputConfig {
    /// Generate Intel HEX file.
    pub hex: bool,
    /// Generate raw binary file.
    pub bin: bool,
    /// Generate size report.
    pub size_report: bool,
}

impl BinaryOutputConfig {
    /// Create a new binary output configuration.
    pub fn new() -> Self {
        Self {
            hex: true,
            bin: true,
            size_report: true,
        }
    }
}

impl Default for BinaryOutputConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Size statistics from arm-none-eabi-size.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeStats {
    /// Text section size (code).
    pub text: u32,
    /// Data section size (initialized data).
    pub data: u32,
    /// BSS section size (uninitialized data).
    pub bss: u32,
    /// Total size.
    pub total: u32,
}

/// Result of binary generation.
#[derive(Debug, Clone)]
pub struct BinaryResult {
    /// Path to generated HEX file (if requested).
    pub hex_path: Option<PathBuf>,
    /// Path to generated BIN file (if requested).
    pub bin_path: Option<PathBuf>,
    /// Size statistics (if requested).
    pub size_stats: Option<SizeStats>,
}

/// Build objcopy command for HEX generation.
pub fn build_objcopy_hex_command(
    _objcopy_path: &Path,
    elf_path: &Path,
    hex_path: &Path,
) -> Vec<String> {
    vec![
        "-O".to_string(),
        "ihex".to_string(),
        elf_path.display().to_string(),
        hex_path.display().to_string(),
    ]
}

/// Build objcopy command for binary generation.
pub fn build_objcopy_bin_command(
    _objcopy_path: &Path,
    elf_path: &Path,
    bin_path: &Path,
) -> Vec<String> {
    vec![
        "-O".to_string(),
        "binary".to_string(),
        elf_path.display().to_string(),
        bin_path.display().to_string(),
    ]
}

/// Generate Intel HEX file from ELF.
pub fn generate_hex(
    objcopy_path: &Path,
    elf_path: &Path,
    hex_path: &Path,
) -> Result<(), String> {
    let args = build_objcopy_hex_command(objcopy_path, elf_path, hex_path);
    
    let output = Command::new(objcopy_path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run objcopy: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("objcopy failed: {}", stderr));
    }
    
    Ok(())
}

/// Generate raw binary file from ELF.
pub fn generate_bin(
    objcopy_path: &Path,
    elf_path: &Path,
    bin_path: &Path,
) -> Result<(), String> {
    let args = build_objcopy_bin_command(objcopy_path, elf_path, bin_path);
    
    let output = Command::new(objcopy_path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run objcopy: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("objcopy failed: {}", stderr));
    }
    
    Ok(())
}

/// Parse size output from arm-none-eabi-size.
pub fn parse_size_output(output: &str) -> Option<SizeStats> {
    // Expected format:
    //    text    data     bss     dec     hex filename
    //   12345    1234     567   14146    3742 test.elf
    
    let lines: Vec<&str> = output.lines().collect();
    if lines.len() < 2 {
        return None;
    }
    
    // Parse the second line (first line is header)
    let parts: Vec<&str> = lines[1].split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    
    let text = parts[0].parse().ok()?;
    let data = parts[1].parse().ok()?;
    let bss = parts[2].parse().ok()?;
    let total = text + data + bss;
    
    Some(SizeStats {
        text,
        data,
        bss,
        total,
    })
}

/// Get size statistics from ELF file.
pub fn get_size_stats(
    size_path: &Path,
    elf_path: &Path,
) -> Result<SizeStats, String> {
    let output = Command::new(size_path)
        .arg(elf_path)
        .output()
        .map_err(|e| format!("Failed to run size: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("size failed: {}", stderr));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_size_output(&stdout)
        .ok_or_else(|| "Failed to parse size output".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_objcopy_hex_command() {
        let objcopy = Path::new("/usr/bin/arm-none-eabi-objcopy");
        let elf = Path::new("test.elf");
        let hex = Path::new("test.hex");
        
        let args = build_objcopy_hex_command(objcopy, elf, hex);
        
        assert!(args.contains(&"-O".to_string()));
        assert!(args.contains(&"ihex".to_string()));
        assert!(args.contains(&"test.elf".to_string()));
        assert!(args.contains(&"test.hex".to_string()));
    }
    
    #[test]
    fn test_build_objcopy_bin_command() {
        let objcopy = Path::new("/usr/bin/arm-none-eabi-objcopy");
        let elf = Path::new("test.elf");
        let bin = Path::new("test.bin");
        
        let args = build_objcopy_bin_command(objcopy, elf, bin);
        
        assert!(args.contains(&"-O".to_string()));
        assert!(args.contains(&"binary".to_string()));
        assert!(args.contains(&"test.elf".to_string()));
        assert!(args.contains(&"test.bin".to_string()));
    }
    
    #[test]
    fn test_parse_size_output() {
        let output = r#"   text    data     bss     dec     hex filename
   12345    1234     567   14146    3742 test.elf
"#;
        
        let stats = parse_size_output(output).unwrap();
        
        assert_eq!(stats.text, 12345);
        assert_eq!(stats.data, 1234);
        assert_eq!(stats.bss, 567);
        assert_eq!(stats.total, 14146);
    }
    
    #[test]
    fn test_parse_size_output_malformed() {
        let output = "invalid output";
        
        let stats = parse_size_output(output);
        
        assert!(stats.is_none());
    }
}
