// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Binary generation from ELF files for ARM microcontrollers.
//!
//! This module provides functions to convert ELF executables into formats suitable
//! for flashing to ARM microcontrollers (Intel HEX and raw binary), as well as
//! extracting size statistics.
//!
//! # Example
//!
//! ```no_run
//! use axiom_toolchain::{generate_hex, generate_bin, get_size_stats};
//! use std::path::Path;
//!
//! let objcopy = Path::new("/usr/bin/arm-none-eabi-objcopy");
//! let size_tool = Path::new("/usr/bin/arm-none-eabi-size");
//! let elf = Path::new("firmware.elf");
//!
//! // Generate HEX file for flashing
//! generate_hex(objcopy, elf, Path::new("firmware.hex")).unwrap();
//!
//! // Generate raw binary
//! generate_bin(objcopy, elf, Path::new("firmware.bin")).unwrap();
//!
//! // Get size statistics
//! let stats = get_size_stats(size_tool, elf).unwrap();
//! println!("Text: {} bytes, Data: {} bytes, BSS: {} bytes",
//!          stats.text, stats.data, stats.bss);
//! ```

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Binary output configuration.
///
/// Specifies which output formats to generate from an ELF file.
#[derive(Debug, Clone)]
pub struct BinaryOutputConfig {
    /// Generate Intel HEX file (.hex).
    pub hex: bool,
    /// Generate raw binary file (.bin).
    pub bin: bool,
    /// Generate size report showing memory usage.
    pub size_report: bool,
}

impl BinaryOutputConfig {
    /// Create a new binary output configuration with all formats enabled.
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

/// Size statistics from arm-none-eabi-size tool.
///
/// Provides memory usage breakdown for an ELF executable, showing how much
/// space is used by code, initialized data, and uninitialized data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeStats {
    /// Text section size in bytes (executable code).
    pub text: u32,
    /// Data section size in bytes (initialized global/static variables).
    pub data: u32,
    /// BSS section size in bytes (uninitialized global/static variables).
    pub bss: u32,
    /// Total size in bytes (text + data + bss).
    pub total: u32,
}

/// Result of binary generation operations.
///
/// Contains paths to generated files and size statistics if requested.
#[derive(Debug, Clone)]
pub struct BinaryResult {
    /// Path to generated Intel HEX file (if requested).
    pub hex_path: Option<PathBuf>,
    /// Path to generated raw binary file (if requested).
    pub bin_path: Option<PathBuf>,
    /// Size statistics (if requested).
    pub size_stats: Option<SizeStats>,
}

/// Build objcopy command arguments for HEX generation.
///
/// Creates the argument list for converting an ELF file to Intel HEX format.
///
/// # Arguments
///
/// * `_objcopy_path` - Path to arm-none-eabi-objcopy (unused, for API consistency)
/// * `elf_path` - Input ELF file path
/// * `hex_path` - Output HEX file path
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

/// Build objcopy command arguments for binary generation.
///
/// Creates the argument list for converting an ELF file to raw binary format.
///
/// # Arguments
///
/// * `_objcopy_path` - Path to arm-none-eabi-objcopy (unused, for API consistency)
/// * `elf_path` - Input ELF file path
/// * `bin_path` - Output binary file path
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

/// Generate Intel HEX file from ELF executable.
///
/// Intel HEX format is commonly used for flashing firmware to microcontrollers.
/// It includes address information and checksums for data integrity.
///
/// # Arguments
///
/// * `objcopy_path` - Path to arm-none-eabi-objcopy tool
/// * `elf_path` - Input ELF file
/// * `hex_path` - Output HEX file path
///
/// # Errors
///
/// Returns an error if objcopy execution fails or the ELF file is invalid.
pub fn generate_hex(objcopy_path: &Path, elf_path: &Path, hex_path: &Path) -> Result<(), String> {
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

/// Generate raw binary file from ELF executable.
///
/// Raw binary format contains only the executable code and data without
/// any metadata or address information. Useful for direct memory programming.
///
/// # Arguments
///
/// * `objcopy_path` - Path to arm-none-eabi-objcopy tool
/// * `elf_path` - Input ELF file
/// * `bin_path` - Output binary file path
///
/// # Errors
///
/// Returns an error if objcopy execution fails or the ELF file is invalid.
pub fn generate_bin(objcopy_path: &Path, elf_path: &Path, bin_path: &Path) -> Result<(), String> {
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

/// Parse size output from arm-none-eabi-size tool.
///
/// Extracts memory usage statistics from the size tool's output.
///
/// Expected format:
/// ```text
///    text    data     bss     dec     hex filename
///   12345    1234     567   14146    3742 test.elf
/// ```
///
/// # Arguments
///
/// * `output` - Standard output from arm-none-eabi-size
///
/// # Returns
///
/// Parsed size statistics, or None if the output format is invalid.
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

/// Get size statistics from an ELF file.
///
/// Runs arm-none-eabi-size on the ELF file and parses the output to extract
/// memory usage information.
///
/// # Arguments
///
/// * `size_path` - Path to arm-none-eabi-size tool
/// * `elf_path` - ELF file to analyze
///
/// # Returns
///
/// Size statistics showing text, data, and BSS section sizes.
///
/// # Errors
///
/// Returns an error if the size tool fails to execute or the output cannot be parsed.
pub fn get_size_stats(size_path: &Path, elf_path: &Path) -> Result<SizeStats, String> {
    let output = Command::new(size_path)
        .arg(elf_path)
        .output()
        .map_err(|e| format!("Failed to run size: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("size failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_size_output(&stdout).ok_or_else(|| "Failed to parse size output".to_string())
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
