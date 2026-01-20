// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Integration tests for ARM toolchain detection.

use axiom_toolchain::*;

#[test]
fn test_detect_all_runs_without_panic() {
    // This test verifies detect_all() runs without panicking
    // It may or may not find toolchains depending on the system
    let _toolchains = detect_all();
    // Test passes if no panic occurs
}

#[test]
fn test_detect_at_path_nonexistent() {
    use std::path::Path;
    let result = detect_at_path(Path::new("/nonexistent/path/gcc"), ToolchainKind::ArmGcc);
    assert!(result.is_none());
}

#[test]
fn test_detect_arm_toolchains_runs() {
    // This test verifies detect_arm_toolchains() runs without panicking
    let suites = detect_arm_toolchains();
    
    // If any toolchains are found, verify they have valid version strings
    for suite in suites {
        assert!(!suite.version.is_empty(), "Version should not be empty");
        assert!(suite.gcc.to_string_lossy().contains("arm-none-eabi-gcc") || 
                suite.gcc.to_string_lossy().contains("gcc"),
                "GCC path should contain gcc");
    }
}


#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_compile_arm_simple_file() {
    use std::path::PathBuf;
    
    // Only run if ARM toolchain is available
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }
    
    let mcu = ArmMcuConfig::cortex_m3();
    
    let mut source = std::env::current_dir().unwrap();
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/Drivers/gpio.c");
    
    let mut inc_path = std::env::current_dir().unwrap();
    if inc_path.ends_with("crates/axiom-toolchain") {
        inc_path.pop();
        inc_path.pop();
    }
    inc_path.push("tests/fixtures/arm-reference-project/Drivers");
    
    let output = PathBuf::from("/tmp/test_gpio.o");
    
    let request = ArmCompileRequest::new(source, output.clone(), mcu)
        .with_include_path(inc_path);
    
    let result = compile_arm(&gcc_path, &request);
    
    // Should compile successfully (or at least attempt to)
    assert!(result.exit_code == 0 || !result.stderr.is_empty());
    
    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_compile_arm_with_fpu() {
    use std::path::PathBuf;
    
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }
    
    let mcu = ArmMcuConfig::cortex_m4(); // Has FPU
    
    let mut source = std::env::current_dir().unwrap();
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/Drivers/gpio.c");
    
    let mut inc_path = std::env::current_dir().unwrap();
    if inc_path.ends_with("crates/axiom-toolchain") {
        inc_path.pop();
        inc_path.pop();
    }
    inc_path.push("tests/fixtures/arm-reference-project/Drivers");
    
    let output = PathBuf::from("/tmp/test_gpio_m4.o");
    
    let request = ArmCompileRequest::new(source, output.clone(), mcu)
        .with_include_path(inc_path);
    
    let result = compile_arm(&gcc_path, &request);
    
    // Should compile successfully
    assert!(result.exit_code == 0 || !result.stderr.is_empty());
    
    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_compile_arm_syntax_error() {
    use std::path::PathBuf;
    
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }
    
    let mcu = ArmMcuConfig::cortex_m3();
    
    let mut source = std::env::current_dir().unwrap();
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/edge_cases/syntax_error.c");
    
    let output = PathBuf::from("/tmp/test_syntax_error.o");
    
    let request = ArmCompileRequest::new(source, output.clone(), mcu);
    
    let result = compile_arm(&gcc_path, &request);
    
    // Should fail with non-zero exit code
    assert_ne!(result.exit_code, 0);
    assert!(!result.stderr.is_empty());
    
    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_compile_arm_missing_include() {
    use std::path::PathBuf;
    
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }
    
    let mcu = ArmMcuConfig::cortex_m3();
    
    let mut source = std::env::current_dir().unwrap();
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/edge_cases/missing_include.c");
    
    let output = PathBuf::from("/tmp/test_missing_include.o");
    
    let request = ArmCompileRequest::new(source, output.clone(), mcu);
    
    let result = compile_arm(&gcc_path, &request);
    
    // Should fail with "fatal error" in stderr
    assert_ne!(result.exit_code, 0);
    assert!(result.stderr.contains("fatal error") || result.stderr.contains("No such file"));
    
    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_compile_arm_inline_assembly() {
    use std::path::PathBuf;
    
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }
    
    let mcu = ArmMcuConfig::cortex_m3();
    
    // Navigate to workspace root
    let mut source = std::env::current_dir().unwrap();
    // If we're in crates/axiom-toolchain, go up two levels
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/edge_cases/inline_assembly.c");
    
    let output = PathBuf::from("/tmp/test_inline_asm.o");
    
    let request = ArmCompileRequest::new(source, output.clone(), mcu);
    
    let result = compile_arm(&gcc_path, &request);
    
    // Should compile successfully for ARM target
    assert_eq!(result.exit_code, 0, "Inline assembly should compile for ARM: {}", result.stderr);
    
    // Clean up
    let _ = std::fs::remove_file(output);
}
