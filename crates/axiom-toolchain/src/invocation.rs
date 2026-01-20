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

/// Build command arguments for ARM compilation.
pub fn build_arm_compile_command(
    _gcc_path: &std::path::Path,
    request: &crate::ArmCompileRequest,
) -> Vec<String> {
    let mut args = Vec::new();
    
    // Compile only (don't link)
    args.push("-c".to_string());
    
    // MCU-specific flags
    args.extend(request.mcu.compiler_flags());
    
    // Include paths (in order)
    for path in &request.include_paths {
        args.push(format!("-I{}", path.display()));
    }
    
    // Optimization
    args.push(format!("-O{}", request.optimization));
    
    // Debug symbols
    if request.debug {
        args.push("-g3".to_string());
    }
    
    // Source file
    args.push(request.source.display().to_string());
    
    // Output file
    args.push("-o".to_string());
    args.push(request.output.display().to_string());
    
    args
}

/// Compile ARM source code.
pub fn compile_arm(
    gcc_path: &std::path::Path,
    request: &crate::ArmCompileRequest,
) -> CompileResult {
    let args = build_arm_compile_command(gcc_path, request);
    let start = Instant::now();
    
    let output = Command::new(gcc_path)
        .args(&args)
        .output();
    
    let duration_ms = start.elapsed().as_millis() as u64;
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let diagnostics = parse_diagnostics(&stderr, ToolchainKind::ArmGcc);
            
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

/// Build command arguments for ARM linking.
pub fn build_arm_link_command(
    gcc_path: &std::path::Path,
    request: &crate::ArmLinkRequest,
) -> Vec<String> {
    let mut args = Vec::new();
    
    // MCU-specific flags
    args.extend(request.mcu.linker_flags(&request.linker));
    
    // Nano specs for smaller code size
    args.push("--specs=nano.specs".to_string());
    
    // No startup files (we provide our own)
    args.push("-nostartfiles".to_string());
    
    // Object files
    for obj in &request.objects {
        args.push(obj.display().to_string());
    }
    
    // Output file
    args.push("-o".to_string());
    args.push(request.output.display().to_string());
    
    args
}

/// Link ARM object files.
pub fn link_arm(
    gcc_path: &std::path::Path,
    request: &crate::ArmLinkRequest,
) -> crate::LinkResult {
    let args = build_arm_link_command(gcc_path, request);
    
    let output = Command::new(gcc_path)
        .args(&args)
        .output();
    
    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let diagnostics = parse_diagnostics(&stderr, ToolchainKind::ArmGcc);
            
            crate::LinkResult {
                exit_code: output.status.code().unwrap_or(-1),
                stdout,
                stderr,
                diagnostics,
            }
        }
        Err(e) => crate::LinkResult {
            exit_code: -1,
            stdout: String::new(),
            stderr: e.to_string(),
            diagnostics: vec![Diagnostic::error(e.to_string())],
        },
    }
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
    
    #[test]
    fn test_build_arm_compile_command_includes_c_flag() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let request = crate::ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        );
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"-c".to_string()));
    }
    
    #[test]
    fn test_build_arm_compile_command_includes_source_and_output() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let request = crate::ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        );
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"test.c".to_string()));
        assert!(args.contains(&"-o".to_string()));
        assert!(args.contains(&"test.o".to_string()));
    }
    
    #[test]
    fn test_build_arm_compile_command_includes_mcu_flags() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let request = crate::ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        );
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"-mcpu=cortex-m3".to_string()));
        assert!(args.contains(&"-mthumb".to_string()));
    }
    
    #[test]
    fn test_build_arm_compile_command_include_paths_in_order() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let request = crate::ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        )
        .with_include_path("inc1")
        .with_include_path("inc2");
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        let inc1_pos = args.iter().position(|a| a == "-Iinc1");
        let inc2_pos = args.iter().position(|a| a == "-Iinc2");
        
        assert!(inc1_pos.is_some());
        assert!(inc2_pos.is_some());
        assert!(inc1_pos.unwrap() < inc2_pos.unwrap());
    }
    
    #[test]
    fn test_build_arm_compile_command_optimization_levels() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        
        for level in 0..=3 {
            let request = crate::ArmCompileRequest::new(
                PathBuf::from("test.c"),
                PathBuf::from("test.o"),
                mcu.clone(),
            )
            .with_optimization(level);
            
            let args = build_arm_compile_command(
                std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
                &request,
            );
            
            assert!(args.contains(&format!("-O{}", level)));
        }
    }
    
    #[test]
    fn test_build_arm_compile_command_debug_flag() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let request = crate::ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        )
        .with_debug(true);
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"-g3".to_string()));
    }
    
    #[test]
    fn test_build_arm_link_command_includes_linker_script() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let linker = crate::LinkerConfig::new("test.ld");
        let request = crate::ArmLinkRequest::new(
            vec![PathBuf::from("test.o")],
            PathBuf::from("test.elf"),
            linker,
            mcu,
        );
        
        let args = build_arm_link_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.iter().any(|a| a.starts_with("-Ttest.ld")));
    }
    
    #[test]
    fn test_build_arm_link_command_includes_all_objects() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let linker = crate::LinkerConfig::new("test.ld");
        let request = crate::ArmLinkRequest::new(
            vec![
                PathBuf::from("main.o"),
                PathBuf::from("gpio.o"),
                PathBuf::from("uart.o"),
            ],
            PathBuf::from("test.elf"),
            linker,
            mcu,
        );
        
        let args = build_arm_link_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"main.o".to_string()));
        assert!(args.contains(&"gpio.o".to_string()));
        assert!(args.contains(&"uart.o".to_string()));
    }
    
    #[test]
    fn test_build_arm_link_command_includes_mcu_flags() {
        let mcu = crate::ArmMcuConfig::cortex_m3();
        let linker = crate::LinkerConfig::new("test.ld");
        let request = crate::ArmLinkRequest::new(
            vec![PathBuf::from("test.o")],
            PathBuf::from("test.elf"),
            linker,
            mcu,
        );
        
        let args = build_arm_link_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        assert!(args.contains(&"-mcpu=cortex-m3".to_string()));
        assert!(args.contains(&"-mthumb".to_string()));
    }
    
    #[test]
    fn test_link_result_memory_overflow_detection_will_not_fit() {
        let result = crate::LinkResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: "section `.text' will not fit in region `FLASH'".to_string(),
            diagnostics: vec![],
        };
        
        assert!(result.has_memory_overflow());
    }
    
    #[test]
    fn test_link_result_memory_overflow_detection_region_overflow() {
        let result = crate::LinkResult {
            exit_code: 1,
            stdout: String::new(),
            stderr: "region `RAM' overflowed by 1024 bytes".to_string(),
            diagnostics: vec![],
        };
        
        assert!(result.has_memory_overflow());
    }
}
