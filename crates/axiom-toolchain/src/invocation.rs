// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compiler invocation.

use crate::{CompileRequest, CompileResult, DetectedToolchain, ToolchainKind};
use axiom_core::Diagnostic;
use std::process::Command;
use std::time::Instant;

/// Build command arguments for a compile request.
pub fn build_command(toolchain: &DetectedToolchain, request: &CompileRequest) -> Vec<String> {
    let mut args = Vec::new();

    // Source file
    args.push("-c".to_string());
    args.push(request.source.display().to_string());

    // Output file
    args.push("-o".to_string());
    args.push(request.output.display().to_string());

    // Optimization level
    args.push(format!("-O{}", request.optimization));

    // Debug symbols
    if request.debug {
        args.push("-g".to_string());
    }

    // Target (for cross-compilation)
    if let Some(ref target) = request.target {
        match toolchain.kind {
            ToolchainKind::Clang => {
                args.push(format!("--target={}", target));
            }
            ToolchainKind::Gcc | ToolchainKind::ArmGcc => {
                // GCC uses different binaries for cross-compilation
                // The target is implicit in the binary name (arm-none-eabi-gcc)
            }
            ToolchainKind::Python => {
                // Not applicable
            }
        }
    }

    // Additional flags
    args.extend(request.flags.iter().cloned());

    args
}

/// Get the command that would be executed (dry run).
pub fn dry_run(toolchain: &DetectedToolchain, request: &CompileRequest) -> String {
    let args = build_command(toolchain, request);
    format!("{} {}", toolchain.path.display(), args.join(" "))
}

/// Execute compilation.
pub fn compile(toolchain: &DetectedToolchain, request: &CompileRequest) -> CompileResult {
    let args = build_command(toolchain, request);
    let start = Instant::now();

    let output = Command::new(&toolchain.path)
        .args(&args)
        .output();

    let duration_ms = start.elapsed().as_millis() as u64;

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let diagnostics = parse_diagnostics(&stderr, toolchain.kind);

            CompileResult {
                exit_code: output.status.code().unwrap_or(-1),
                stdout,
                stderr,
                duration_ms,
                diagnostics,
            }
        }
        Err(e) => CompileResult {
            exit_code: -1,
            stdout: String::new(),
            stderr: e.to_string(),
            duration_ms,
            diagnostics: vec![Diagnostic::error(e.to_string())],
        },
    }
}

/// Parse diagnostics from compiler stderr.
fn parse_diagnostics(stderr: &str, _kind: ToolchainKind) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for line in stderr.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Simple heuristic: lines containing "error:" or "warning:"
        if line.contains("error:") {
            diagnostics.push(Diagnostic::error(line.to_string()));
        } else if line.contains("warning:") {
            diagnostics.push(Diagnostic::warning(line.to_string()));
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_toolchain() -> DetectedToolchain {
        DetectedToolchain::new(
            ToolchainKind::Clang,
            PathBuf::from("/usr/bin/clang"),
            "15.0.0".to_string(),
        )
    }

    #[test]
    fn test_build_command_basic() {
        let tc = test_toolchain();
        let request = CompileRequest::new(
            PathBuf::from("main.c"),
            PathBuf::from("main.o"),
        );

        let args = build_command(&tc, &request);
        assert!(args.contains(&"-c".to_string()));
        assert!(args.contains(&"main.c".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"main.o".to_string()));
        assert!(args.contains(&"-O0".to_string()));
        assert!(args.contains(&"-g".to_string()));
    }

    #[test]
    fn test_build_command_with_optimization() {
        let tc = test_toolchain();
        let request = CompileRequest::new(
            PathBuf::from("main.c"),
            PathBuf::from("main.o"),
        )
        .with_optimization(2)
        .with_debug(false);

        let args = build_command(&tc, &request);
        assert!(args.contains(&"-O2".to_string()));
        assert!(!args.contains(&"-g".to_string()));
    }

    #[test]
    fn test_dry_run() {
        let tc = test_toolchain();
        let request = CompileRequest::new(
            PathBuf::from("main.c"),
            PathBuf::from("main.o"),
        );

        let cmd = dry_run(&tc, &request);
        assert!(cmd.starts_with("/usr/bin/clang"));
        assert!(cmd.contains("main.c"));
    }

    #[test]
    fn test_parse_diagnostics() {
        let stderr = r#"
main.c:10:5: error: use of undeclared identifier 'x'
main.c:15:10: warning: unused variable 'y' [-Wunused-variable]
        "#;

        let diags = parse_diagnostics(stderr, ToolchainKind::Clang);
        assert_eq!(diags.len(), 2);
    }
}
