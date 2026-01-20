// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compiler stage visualization for ARM toolchain.

use std::path::Path;
use std::process::Command;

/// Build preprocessor flags (-E).
pub fn build_preprocessor_flags() -> Vec<String> {
    vec!["-E".to_string()]
}

/// Get preprocessor output for a source file.
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
pub fn build_assembly_flags() -> Vec<String> {
    vec!["-S".to_string()]
}

/// Get assembly output for a source file.
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
pub fn build_disassembly_flags() -> Vec<String> {
    vec!["-d".to_string()]
}

/// Get disassembly of an object file.
pub fn get_disassembly(
    objdump_path: &Path,
    object_file: &Path,
) -> Result<String, String> {
    let args = build_disassembly_flags();
    
    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to run objdump: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build symbol table flags (objdump -t).
pub fn build_symbol_table_flags() -> Vec<String> {
    vec!["-t".to_string()]
}

/// Get symbol table from an object file.
pub fn get_symbol_table(
    objdump_path: &Path,
    object_file: &Path,
) -> Result<String, String> {
    let args = build_symbol_table_flags();
    
    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());
    
    let output = cmd.output()
        .map_err(|e| format!("Failed to run objdump: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build section headers flags (objdump -h).
pub fn build_section_headers_flags() -> Vec<String> {
    vec!["-h".to_string()]
}

/// Get section headers from an object file.
pub fn get_section_headers(
    objdump_path: &Path,
    object_file: &Path,
) -> Result<String, String> {
    let args = build_section_headers_flags();
    
    let mut cmd = Command::new(objdump_path);
    cmd.args(&args);
    cmd.arg(object_file.display().to_string());
    
    let output = cmd.output()
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
