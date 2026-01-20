// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compiler stage visualization for ARM toolchain.
//!
//! This module provides functions to capture and display intermediate outputs from
//! various stages of the ARM GCC compilation process, including preprocessor output,
//! assembly code, disassembly, symbol tables, and section headers.
//!
//! These visualizations are useful for:
//! - Understanding how macros and includes are expanded
//! - Analyzing generated assembly code for optimization
//! - Debugging linking issues with symbol tables
//! - Verifying memory layout with section headers
//!
//! # Example
//!
//! ```no_run
//! use axiom_toolchain::{get_preprocessor_output, get_assembly_output, ArmMcuConfig};
//! use std::path::Path;
//!
//! let gcc = Path::new("/usr/bin/arm-none-eabi-gcc");
//! let source = Path::new("main.c");
//! let mcu = ArmMcuConfig::cortex_m4();
//! let flags = mcu.compiler_flags();
//!
//! // Get preprocessor output
//! let preprocessed = get_preprocessor_output(gcc, source, &flags).unwrap();
//! println!("Preprocessed code:\n{}", preprocessed);
//!
//! // Get assembly output
//! let asm_output = Path::new("main.s");
//! let assembly = get_assembly_output(gcc, source, asm_output, &flags).unwrap();
//! println!("Assembly code:\n{}", assembly);
//! ```

use std::path::Path;
use std::process::Command;

/// Build preprocessor flags (-E).
///
/// Returns the flags needed to run only the preprocessor stage,
/// which expands macros and includes.
pub fn build_preprocessor_flags() -> Vec<String> {
    vec!["-E".to_string()]
}

/// Get preprocessor output for a source file.
///
/// Runs the preprocessor on the source file and returns the expanded code
/// with all macros and includes resolved.
///
/// # Arguments
///
/// * `gcc_path` - Path to arm-none-eabi-gcc
/// * `source` - Source file to preprocess
/// * `mcu_flags` - MCU-specific compiler flags (CPU, FPU, defines, includes)
///
/// # Returns
///
/// Preprocessed source code as a string.
///
/// # Errors
///
/// Returns an error if the preprocessor fails or the source file has errors.
pub fn get_preprocessor_output(
    gcc_path: &Path,
    source: &Path,
    mcu_flags: &[String],
) -> Result<String, String> {
    let mut args = build_preprocessor_flags();
    args.extend(mcu_flags.iter().cloned());
    args.push(source.display().to_string());

    let output = Command::new(gcc_path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run preprocessor: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build assembly flags (-S).
///
/// Returns the flags needed to generate assembly output instead of
/// compiling to object code.
pub fn build_assembly_flags() -> Vec<String> {
    vec!["-S".to_string()]
}

/// Get assembly output for a source file.
///
/// Compiles the source file to assembly language and returns the generated
/// assembly code. Useful for understanding code generation and optimization.
///
/// # Arguments
///
/// * `gcc_path` - Path to arm-none-eabi-gcc
/// * `source` - Source file to compile
/// * `output` - Path where assembly file will be written
/// * `mcu_flags` - MCU-specific compiler flags
///
/// # Returns
///
/// Generated assembly code as a string.
///
/// # Errors
///
/// Returns an error if compilation fails or the assembly file cannot be read.
pub fn get_assembly_output(
    gcc_path: &Path,
    source: &Path,
    output: &Path,
    mcu_flags: &[String],
) -> Result<String, String> {
    let mut args = build_assembly_flags();
    args.extend(mcu_flags.iter().cloned());
    args.push(source.display().to_string());
    args.push("-o".to_string());
    args.push(output.display().to_string());

    let result = Command::new(gcc_path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to run assembler: {}", e))?;

    if result.status.success() {
        // Read the generated assembly file
        std::fs::read_to_string(output)
            .map_err(|e| format!("Failed to read assembly output: {}", e))
    } else {
        Err(String::from_utf8_lossy(&result.stderr).to_string())
    }
}

/// Build disassembly flags (objdump -d).
///
/// Returns the flags needed to disassemble an object or executable file.
pub fn build_disassembly_flags() -> Vec<String> {
    vec!["-d".to_string()]
}

/// Get disassembly of an object file.
///
/// Disassembles compiled object code back to assembly language with
/// addresses and opcodes. Useful for verifying code generation and
/// debugging optimization issues.
///
/// # Arguments
///
/// * `objdump_path` - Path to arm-none-eabi-objdump
/// * `object_file` - Object or ELF file to disassemble
///
/// # Returns
///
/// Disassembled code with addresses, opcodes, and mnemonics.
///
/// # Errors
///
/// Returns an error if objdump fails or the file is not a valid object file.
pub fn get_disassembly(objdump_path: &Path, object_file: &Path) -> Result<String, String> {
    let args = build_disassembly_flags();

    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run objdump: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build symbol table flags (objdump -t).
///
/// Returns the flags needed to display the symbol table from an object file.
pub fn build_symbol_table_flags() -> Vec<String> {
    vec!["-t".to_string()]
}

/// Get symbol table from an object file.
///
/// Extracts the symbol table showing all functions, variables, and their
/// addresses. Useful for debugging linking issues and understanding memory layout.
///
/// # Arguments
///
/// * `objdump_path` - Path to arm-none-eabi-objdump
/// * `object_file` - Object or ELF file to analyze
///
/// # Returns
///
/// Symbol table with names, addresses, types, and sections.
///
/// # Errors
///
/// Returns an error if objdump fails or the file is not a valid object file.
pub fn get_symbol_table(objdump_path: &Path, object_file: &Path) -> Result<String, String> {
    let args = build_symbol_table_flags();

    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run objdump: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build section headers flags (objdump -h).
///
/// Returns the flags needed to display section headers from an object file.
pub fn build_section_headers_flags() -> Vec<String> {
    vec!["-h".to_string()]
}

/// Get section headers from an object file.
///
/// Displays information about all sections in the object file, including
/// their sizes, addresses, and attributes. Useful for verifying memory
/// layout and linker script behavior.
///
/// # Arguments
///
/// * `objdump_path` - Path to arm-none-eabi-objdump
/// * `object_file` - Object or ELF file to analyze
///
/// # Returns
///
/// Section headers with names, sizes, VMA/LMA addresses, and flags.
///
/// # Errors
///
/// Returns an error if objdump fails or the file is not a valid object file.
pub fn get_section_headers(objdump_path: &Path, object_file: &Path) -> Result<String, String> {
    let args = build_section_headers_flags();

    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run objdump: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_preprocessor_flags_contains_e() {
        let flags = build_preprocessor_flags();
        assert!(flags.contains(&"-E".to_string()));
    }

    #[test]
    fn test_build_assembly_flags_contains_s() {
        let flags = build_assembly_flags();
        assert!(flags.contains(&"-S".to_string()));
    }

    #[test]
    fn test_build_disassembly_flags_contains_d() {
        let flags = build_disassembly_flags();
        assert!(flags.contains(&"-d".to_string()));
    }

    #[test]
    fn test_build_symbol_table_flags_contains_t() {
        let flags = build_symbol_table_flags();
        assert!(flags.contains(&"-t".to_string()));
    }

    #[test]
    fn test_build_section_headers_flags_contains_h() {
        let flags = build_section_headers_flags();
        assert!(flags.contains(&"-h".to_string()));
    }
}
