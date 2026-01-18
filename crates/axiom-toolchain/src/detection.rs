// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Toolchain detection from known paths.

use crate::{DetectedToolchain, ToolchainKind};
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
];

/// Known paths for Python.
const PYTHON_PATHS: &[&str] = &[
    "/usr/bin/python3",
    "/opt/homebrew/bin/python3",
    "/usr/local/bin/python3",
];

/// Detect all available toolchains.
pub fn detect_all() -> Vec<DetectedToolchain> {
    let mut toolchains = Vec::new();

    // Detect Clang
    for path in CLANG_PATHS {
        if let Some(tc) = detect_at_path(Path::new(path), ToolchainKind::Clang) {
            toolchains.push(tc);
            break; // Take first found
        }
    }

    // Detect GCC
    for path in GCC_PATHS {
        if let Some(tc) = detect_at_path(Path::new(path), ToolchainKind::Gcc) {
            toolchains.push(tc);
            break;
        }
    }

    // Detect ARM GCC
    for path in ARM_GCC_PATHS {
        if let Some(tc) = detect_at_path(Path::new(path), ToolchainKind::ArmGcc) {
            toolchains.push(tc);
            break;
        }
    }

    // Detect Python
    for path in PYTHON_PATHS {
        if let Some(tc) = detect_at_path(Path::new(path), ToolchainKind::Python) {
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

    for path in paths {
        if let Some(tc) = detect_at_path(Path::new(path), kind) {
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
            if let Some(idx) = first_line.rfind(')') {
                let rest = first_line[idx + 1..].trim();
                if !rest.is_empty() {
                    return Some(rest.to_string());
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
            if first_line.starts_with("Python ") {
                return Some(first_line[7..].trim().to_string());
            }
        }
    }

    // Fallback: return first line
    Some(first_line.to_string())
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
}
