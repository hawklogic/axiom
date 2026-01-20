// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Toolchain detection from known paths.

use crate::{DetectedToolchain, ToolchainKind, ArmToolchainSuite, ToolchainSource, ToolchainCompleteness};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Known paths for Clang on macOS.
const CLANG_PATHS: &[&str] = &[
    "/usr/bin/clang",
    "/opt/homebrew/bin/clang",
    "/usr/local/bin/clang",
    "/Library/Developer/CommandLineTools/usr/bin/clang",
];

/// Known paths for GCC on macOS.
const GCC_PATHS: &[&str] = &[
    "/usr/bin/gcc",
    "/opt/homebrew/bin/gcc-13",
    "/opt/homebrew/bin/gcc-12",
    "/opt/homebrew/bin/gcc-11",
    "/usr/local/bin/gcc-13",
    "/usr/local/bin/gcc-12",
    "/usr/local/bin/gcc-11",
];

/// Known paths for ARM GCC.
const ARM_GCC_PATHS: &[&str] = &[
    "/Applications/ARM/bin/arm-none-eabi-gcc",
    "/opt/homebrew/bin/arm-none-eabi-gcc",
    "/usr/local/bin/arm-none-eabi-gcc",
    // STM32CubeIDE macOS paths (versioned)
    "/Applications/STM32CubeIDE.app/Contents/Eclipse/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc",
    // STM32CubeIDE Linux paths (versioned)
    "/opt/st/stm32cubeide_*/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc",
    "/usr/local/STM32CubeIDE/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc",
    // STM32CubeIDE Windows paths (versioned)
    "C:/ST/STM32CubeIDE_*/STM32CubeIDE/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc.exe",
    "C:/Program Files/STMicroelectronics/STM32CubeIDE_*/STM32CubeIDE/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc.exe",
];

/// Known paths for Python.
const PYTHON_PATHS: &[&str] = &[
    "/usr/bin/python3",
    "/opt/homebrew/bin/python3",
    "/usr/local/bin/python3",
];

/// Expand glob patterns in paths and return all matching paths.
fn expand_glob_paths(patterns: &[&str]) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    for pattern in patterns {
        // Check if pattern contains glob characters
        if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
            // Try to expand glob pattern
            if let Ok(entries) = glob::glob(pattern) {
                for entry in entries.flatten() {
                    if entry.exists() {
                        paths.push(entry);
                    }
                }
            }
        } else {
            // Not a glob pattern, just check if it exists
            let path = PathBuf::from(pattern);
            if path.exists() {
                paths.push(path);
            }
        }
    }
    
    paths
}

/// Detect all available toolchains.
pub fn detect_all() -> Vec<DetectedToolchain> {
    let mut toolchains = Vec::new();

    // Detect Clang
    for path in expand_glob_paths(CLANG_PATHS) {
        if let Some(tc) = detect_at_path(&path, ToolchainKind::Clang) {
            toolchains.push(tc);
            break; // Take first found
        }
    }

    // Detect GCC
    for path in expand_glob_paths(GCC_PATHS) {
        if let Some(tc) = detect_at_path(&path, ToolchainKind::Gcc) {
            toolchains.push(tc);
            break;
        }
    }

    // Detect ARM GCC
    for path in expand_glob_paths(ARM_GCC_PATHS) {
        if let Some(tc) = detect_at_path(&path, ToolchainKind::ArmGcc) {
            toolchains.push(tc);
            break;
        }
    }

    // Detect Python
    for path in expand_glob_paths(PYTHON_PATHS) {
        if let Some(tc) = detect_at_path(&path, ToolchainKind::Python) {
            toolchains.push(tc);
            break;
        }
    }

    toolchains
}

/// Detect a specific toolchain kind.
pub fn detect(kind: ToolchainKind) -> Option<DetectedToolchain> {
    let paths = match kind {
        ToolchainKind::Clang => CLANG_PATHS,
        ToolchainKind::Gcc => GCC_PATHS,
        ToolchainKind::ArmGcc => ARM_GCC_PATHS,
        ToolchainKind::Python => PYTHON_PATHS,
    };

    for path in expand_glob_paths(paths) {
        if let Some(tc) = detect_at_path(&path, kind) {
            return Some(tc);
        }
    }

    None
}

/// Detect a toolchain at a specific path.
pub fn detect_at_path(path: &Path, kind: ToolchainKind) -> Option<DetectedToolchain> {
    if !path.exists() {
        return None;
    }

    let version = get_version(path, kind)?;
    Some(DetectedToolchain::new(kind, path.to_path_buf(), version))
}

/// Get version string for a toolchain binary.
fn get_version(path: &Path, kind: ToolchainKind) -> Option<String> {
    let output = Command::new(path).arg("--version").output().ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_version(&stdout, kind)
}

/// Parse version from --version output.
fn parse_version(output: &str, kind: ToolchainKind) -> Option<String> {
    let first_line = output.lines().next()?;

    match kind {
        ToolchainKind::Clang => {
            // "Apple clang version 15.0.0 (clang-1500.0.40.1)"
            // "clang version 17.0.6"
            if let Some(idx) = first_line.find("version") {
                let rest = &first_line[idx + 8..];
                let version: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect();
                if !version.is_empty() {
                    return Some(version);
                }
            }
        }
        ToolchainKind::Gcc | ToolchainKind::ArmGcc => {
            // "gcc (Homebrew GCC 13.2.0) 13.2.0"
            // "arm-none-eabi-gcc (GNU Arm Embedded Toolchain 10.3-2021.10) 10.3.1"
            // "arm-none-eabi-gcc (xPack GNU Arm Embedded GCC x86_64) 12.2.1 20221205"
            if let Some(idx) = first_line.rfind(')') {
                let rest = first_line[idx + 1..].trim();
                // Extract just the version number (first word that looks like a version)
                for word in rest.split_whitespace() {
                    if word.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                        && word.contains('.')
                    {
                        return Some(word.to_string());
                    }
                }
            }
            // Fallback: find version pattern
            for word in first_line.split_whitespace() {
                if word.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                    && word.contains('.')
                {
                    return Some(word.to_string());
                }
            }
        }
        ToolchainKind::Python => {
            // "Python 3.11.6"
            if let Some(stripped) = first_line.strip_prefix("Python ") {
                return Some(stripped.trim().to_string());
            }
        }
    }

    // Fallback: return first line
    Some(first_line.to_string())
}

/// Parse ARM GCC version string (public for testing).
pub fn parse_arm_gcc_version(output: &str) -> Option<String> {
    parse_version(output, ToolchainKind::ArmGcc)
}

/// Check if a version is compatible (meets minimum requirement).
/// Returns true if actual_version >= min_version.
pub fn is_version_compatible(actual: &str, minimum: &str) -> bool {
    let actual_parts: Vec<u32> = actual
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let min_parts: Vec<u32> = minimum
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    for i in 0..actual_parts.len().max(min_parts.len()) {
        let a = actual_parts.get(i).copied().unwrap_or(0);
        let m = min_parts.get(i).copied().unwrap_or(0);
        
        if a > m {
            return true;
        } else if a < m {
            return false;
        }
    }
    
    true // Equal versions are compatible
}

/// Get bundled Python path.
pub fn bundled_python_path() -> PathBuf {
    // Relative to the application bundle
    PathBuf::from("vendor/python/bin/python3")
}

/// Check if bundled Python exists.
pub fn has_bundled_python() -> bool {
    bundled_python_path().exists()
}

/// Validate that all tools in an ARM toolchain suite exist.
pub fn validate_toolchain_suite(suite: &ArmToolchainSuite) -> ToolchainCompleteness {
    let mut missing = Vec::new();
    
    let tools = vec![
        ("gcc", &suite.gcc),
        ("g++", &suite.gxx),
        ("as", &suite.as_),
        ("ld", &suite.ld),
        ("objcopy", &suite.objcopy),
        ("objdump", &suite.objdump),
        ("size", &suite.size),
        ("gdb", &suite.gdb),
    ];
    
    for (name, path) in tools {
        if !path.exists() {
            missing.push(name.to_string());
        }
    }
    
    if missing.is_empty() {
        ToolchainCompleteness::Complete
    } else {
        ToolchainCompleteness::Incomplete { missing }
    }
}

/// Detect the source of a toolchain from its path.
pub fn detect_source(path: &Path) -> ToolchainSource {
    let path_str = path.to_string_lossy();
    
    if path_str.contains("homebrew") || path_str.contains("/opt/homebrew/") {
        ToolchainSource::Homebrew
    } else if path_str.contains("STM32CubeIDE") || path_str.contains("stm32cubeide") {
        ToolchainSource::Stm32CubeIde
    } else if path_str.starts_with("/usr/bin/") || path_str.starts_with("/usr/local/bin/") {
        ToolchainSource::SystemPath
    } else {
        ToolchainSource::Manual
    }
}

/// Detect all ARM toolchain suites.
pub fn detect_arm_toolchains() -> Vec<ArmToolchainSuite> {
    let mut suites = Vec::new();
    
    for gcc_path in expand_glob_paths(ARM_GCC_PATHS) {
        if !gcc_path.exists() {
            continue;
        }
        
        // Get version
        let version = match get_version(&gcc_path, ToolchainKind::ArmGcc) {
            Some(v) => v,
            None => continue,
        };
        
        // Derive other tool paths from gcc path
        let bin_dir = gcc_path.parent().unwrap();
        let gxx = bin_dir.join("arm-none-eabi-g++");
        let as_ = bin_dir.join("arm-none-eabi-as");
        let ld = bin_dir.join("arm-none-eabi-ld");
        let objcopy = bin_dir.join("arm-none-eabi-objcopy");
        let objdump = bin_dir.join("arm-none-eabi-objdump");
        let size = bin_dir.join("arm-none-eabi-size");
        let gdb = bin_dir.join("arm-none-eabi-gdb");
        
        let source = detect_source(&gcc_path);
        
        let suite = ArmToolchainSuite {
            gcc: gcc_path.clone(),
            gxx,
            as_,
            ld,
            objcopy,
            objdump,
            size,
            gdb,
            version,
            source,
            completeness: ToolchainCompleteness::Complete, // Will be validated
        };
        
        // Validate completeness
        let completeness = validate_toolchain_suite(&suite);
        let suite = ArmToolchainSuite {
            completeness,
            ..suite
        };
        
        suites.push(suite);
    }
    
    suites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_clang_version() {
        let output = "Apple clang version 15.0.0 (clang-1500.0.40.1)\nTarget: arm64-apple-darwin23.0.0";
        let version = parse_version(output, ToolchainKind::Clang);
        assert_eq!(version, Some("15.0.0".to_string()));
    }

    #[test]
    fn test_parse_gcc_version() {
        let output = "gcc (Homebrew GCC 13.2.0) 13.2.0\nCopyright...";
        let version = parse_version(output, ToolchainKind::Gcc);
        assert_eq!(version, Some("13.2.0".to_string()));
    }

    #[test]
    fn test_parse_python_version() {
        let output = "Python 3.11.6";
        let version = parse_version(output, ToolchainKind::Python);
        assert_eq!(version, Some("3.11.6".to_string()));
    }
    
    #[test]
    fn test_parse_arm_gcc_version_standard() {
        let output = "arm-none-eabi-gcc (GNU Arm Embedded Toolchain 10.3-2021.10) 10.3.1\nCopyright...";
        let version = parse_arm_gcc_version(output);
        assert_eq!(version, Some("10.3.1".to_string()));
    }
    
    #[test]
    fn test_parse_arm_gcc_version_stm32cubeide() {
        let output = "arm-none-eabi-gcc (xPack GNU Arm Embedded GCC x86_64) 12.2.1 20221205\nCopyright...";
        let version = parse_arm_gcc_version(output);
        // Parser extracts first version-like string after closing paren
        assert_eq!(version, Some("12.2.1".to_string()));
    }
    
    #[test]
    fn test_detect_source_homebrew() {
        let path = Path::new("/opt/homebrew/bin/arm-none-eabi-gcc");
        assert_eq!(detect_source(path), ToolchainSource::Homebrew);
    }
    
    #[test]
    fn test_detect_source_stm32cubeide() {
        let path = Path::new("/Applications/STM32CubeIDE.app/Contents/Eclipse/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.11.3.rel1.202309141235/tools/bin/arm-none-eabi-gcc");
        assert_eq!(detect_source(path), ToolchainSource::Stm32CubeIde);
    }
    
    #[test]
    fn test_validate_toolchain_suite_complete() {
        let suite = ArmToolchainSuite {
            gcc: PathBuf::from("/usr/bin/true"),  // Use existing file for test
            gxx: PathBuf::from("/usr/bin/true"),
            as_: PathBuf::from("/usr/bin/true"),
            ld: PathBuf::from("/usr/bin/true"),
            objcopy: PathBuf::from("/usr/bin/true"),
            objdump: PathBuf::from("/usr/bin/true"),
            size: PathBuf::from("/usr/bin/true"),
            gdb: PathBuf::from("/usr/bin/true"),
            version: "10.3.1".to_string(),
            source: ToolchainSource::SystemPath,
            completeness: ToolchainCompleteness::Complete,
        };
        
        let result = validate_toolchain_suite(&suite);
        assert_eq!(result, ToolchainCompleteness::Complete);
    }
    
    #[test]
    fn test_validate_toolchain_suite_incomplete() {
        let suite = ArmToolchainSuite {
            gcc: PathBuf::from("/usr/bin/true"),
            gxx: PathBuf::from("/nonexistent/g++"),
            as_: PathBuf::from("/usr/bin/true"),
            ld: PathBuf::from("/nonexistent/ld"),
            objcopy: PathBuf::from("/usr/bin/true"),
            objdump: PathBuf::from("/usr/bin/true"),
            size: PathBuf::from("/usr/bin/true"),
            gdb: PathBuf::from("/usr/bin/true"),
            version: "10.3.1".to_string(),
            source: ToolchainSource::SystemPath,
            completeness: ToolchainCompleteness::Complete,
        };
        
        let result = validate_toolchain_suite(&suite);
        match result {
            ToolchainCompleteness::Incomplete { missing } => {
                assert!(missing.contains(&"g++".to_string()));
                assert!(missing.contains(&"ld".to_string()));
                assert_eq!(missing.len(), 2);
            }
            _ => panic!("Expected Incomplete"),
        }
    }
    
    #[test]
    fn test_version_comparison_compatible() {
        assert!(is_version_compatible("14.3.1", "8.0.0"));
    }
    
    #[test]
    fn test_version_comparison_incompatible() {
        assert!(!is_version_compatible("7.9.9", "8.0.0"));
    }
}
