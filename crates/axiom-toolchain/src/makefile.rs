// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Makefile support for ARM projects.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Information about a detected Makefile.
#[derive(Debug, Clone)]
pub struct MakefileInfo {
    /// Path to the Makefile
    pub path: PathBuf,
    /// Detected targets
    pub targets: Vec<String>,
}

/// Result of running make.
#[derive(Debug, Clone)]
pub struct MakeResult {
    /// Exit code
    pub exit_code: i32,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
}

/// Detect Makefile in a directory.
pub fn detect_makefile(dir: &Path) -> Option<MakefileInfo> {
    let makefile_path = dir.join("Makefile");
    if makefile_path.exists() {
        let targets = parse_makefile_targets(&makefile_path).unwrap_or_default();
        Some(MakefileInfo {
            path: makefile_path,
            targets,
        })
    } else {
        None
    }
}

/// Parse targets from a Makefile.
pub fn parse_makefile_targets(makefile_path: &Path) -> Result<Vec<String>, String> {
    let content = std::fs::read_to_string(makefile_path)
        .map_err(|e| format!("Failed to read Makefile: {}", e))?;
    
    let mut targets = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // Look for .PHONY declarations
        if line.starts_with(".PHONY:") {
            let phony_targets = line.strip_prefix(".PHONY:").unwrap().trim();
            for target in phony_targets.split_whitespace() {
                if !targets.contains(&target.to_string()) {
                    targets.push(target.to_string());
                }
            }
        }
        
        // Look for target definitions (lines ending with :)
        if line.contains(':') && !line.starts_with('#') && !line.starts_with('\t') {
            if let Some(target_name) = line.split(':').next() {
                let target_name = target_name.trim();
                // Skip special targets and variables
                if !target_name.is_empty() 
                    && !target_name.starts_with('.') 
                    && !target_name.contains('$')
                    && !target_name.contains('=')
                    && !targets.contains(&target_name.to_string()) {
                    targets.push(target_name.to_string());
                }
            }
        }
    }
    
    Ok(targets)
}

/// Run make with a specific target.
pub fn run_make(
    dir: &Path,
    target: &str,
    toolchain_prefix: Option<&Path>,
) -> MakeResult {
    let mut cmd = Command::new("make");
    cmd.current_dir(dir);
    cmd.arg(target);
    
    // Pass toolchain path as PREFIX environment variable
    if let Some(prefix) = toolchain_prefix {
        if let Some(parent) = prefix.parent() {
            cmd.env("PREFIX", parent);
        }
    }
    
    let output = cmd.output();
    
    match output {
        Ok(output) => MakeResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        },
        Err(e) => MakeResult {
            exit_code: -1,
            stdout: String::new(),
            stderr: format!("Failed to run make: {}", e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_detect_makefile_exists() {
        let temp_dir = std::env::temp_dir().join("test_makefile_detect");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let makefile_path = temp_dir.join("Makefile");
        let mut file = fs::File::create(&makefile_path).unwrap();
        writeln!(file, "all:\n\techo hello").unwrap();
        
        let result = detect_makefile(&temp_dir);
        assert!(result.is_some());
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_detect_makefile_not_exists() {
        let temp_dir = std::env::temp_dir().join("test_makefile_not_exists");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let result = detect_makefile(&temp_dir);
        assert!(result.is_none());
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_parse_makefile_targets() {
        let temp_dir = std::env::temp_dir().join("test_makefile_parse");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let makefile_path = temp_dir.join("Makefile");
        let mut file = fs::File::create(&makefile_path).unwrap();
        writeln!(file, ".PHONY: all clean flash").unwrap();
        writeln!(file, "all:").unwrap();
        writeln!(file, "\techo building").unwrap();
        writeln!(file, "clean:").unwrap();
        writeln!(file, "\trm -f *.o").unwrap();
        
        let targets = parse_makefile_targets(&makefile_path).unwrap();
        assert!(targets.contains(&"all".to_string()));
        assert!(targets.contains(&"clean".to_string()));
        assert!(targets.contains(&"flash".to_string()));
        
        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_parse_makefile_phony_declaration() {
        let temp_dir = std::env::temp_dir().join("test_makefile_phony");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let makefile_path = temp_dir.join("Makefile");
        let mut file = fs::File::create(&makefile_path).unwrap();
        writeln!(file, ".PHONY: test debug").unwrap();
        
        let targets = parse_makefile_targets(&makefile_path).unwrap();
        assert!(targets.contains(&"test".to_string()));
        assert!(targets.contains(&"debug".to_string()));
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}
