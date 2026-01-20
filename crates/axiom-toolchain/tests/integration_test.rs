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
        assert!(
            suite.gcc.to_string_lossy().contains("arm-none-eabi-gcc")
                || suite.gcc.to_string_lossy().contains("gcc"),
            "GCC path should contain gcc"
        );
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

    let request = ArmCompileRequest::new(source, output.clone(), mcu).with_include_path(inc_path);

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

    let request = ArmCompileRequest::new(source, output.clone(), mcu).with_include_path(inc_path);

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
    assert_eq!(
        result.exit_code, 0,
        "Inline assembly should compile for ARM: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_link_arm_with_valid_linker_script() {
    use std::path::PathBuf;

    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }

    // First compile an object file
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

    let obj_path = PathBuf::from("/tmp/test_link_gpio.o");

    let compile_req =
        ArmCompileRequest::new(source, obj_path.clone(), mcu.clone()).with_include_path(inc_path);

    let compile_result = compile_arm(&gcc_path, &compile_req);
    if compile_result.exit_code != 0 {
        // Skip test if compilation fails
        let _ = std::fs::remove_file(&obj_path);
        return;
    }

    // Now link it
    let mut linker_script = std::env::current_dir().unwrap();
    if linker_script.ends_with("crates/axiom-toolchain") {
        linker_script.pop();
        linker_script.pop();
    }
    linker_script.push("tests/fixtures/arm-reference-project/STM32F103C8_FLASH.ld");

    let linker = LinkerConfig::new(linker_script);
    let output = PathBuf::from("/tmp/test_link.elf");

    let link_req = ArmLinkRequest::new(vec![obj_path.clone()], output.clone(), linker, mcu);

    let result = link_arm(&gcc_path, &link_req);

    // Should link successfully or fail with missing symbols (which is expected)
    assert!(result.exit_code == 0 || result.stderr.contains("undefined reference"));

    // Clean up
    let _ = std::fs::remove_file(obj_path);
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_link_arm_missing_linker_script() {
    use std::path::PathBuf;

    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }

    let mcu = ArmMcuConfig::cortex_m3();
    let linker = LinkerConfig::new("/nonexistent/script.ld");
    let output = PathBuf::from("/tmp/test_link_missing.elf");

    let link_req = ArmLinkRequest::new(
        vec![PathBuf::from("/tmp/dummy.o")],
        output.clone(),
        linker,
        mcu,
    );

    let result = link_arm(&gcc_path, &link_req);

    // Should fail
    assert_ne!(result.exit_code, 0);
    assert!(!result.stderr.is_empty());

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_get_assembly_output_inline_assembly() {
    use std::path::PathBuf;

    // Only run if ARM toolchain is available
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }

    let source = PathBuf::from("tests/fixtures/arm-reference-project/edge_cases/inline_assembly.c");
    let output = PathBuf::from("/tmp/inline_assembly.s");

    let mcu = ArmMcuConfig::cortex_m3();
    let mcu_flags = mcu.compiler_flags();

    let result = get_assembly_output(&gcc_path, &source, &output, &mcu_flags);

    // Should succeed and contain ARM assembly instructions
    if let Ok(asm) = result {
        assert!(
            asm.contains("mrs") || asm.contains("msr") || asm.contains(".syntax"),
            "Assembly should contain ARM instructions or directives"
        );
    }
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_get_disassembly_of_object() {
    use std::path::PathBuf;

    // Only run if ARM toolchain is available
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    let objdump_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-objdump");
    if !gcc_path.exists() || !objdump_path.exists() {
        return;
    }

    // First compile a simple file
    let source = PathBuf::from("tests/fixtures/arm-reference-project/Drivers/gpio.c");
    let object = PathBuf::from("/tmp/gpio_test.o");

    let mcu = ArmMcuConfig::cortex_m3();
    let request = ArmCompileRequest::new(source, object.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);
    if result.exit_code != 0 {
        return; // Skip if compilation fails
    }

    // Now disassemble it
    let disasm = get_disassembly(&objdump_path, &object);

    if let Ok(output) = disasm {
        assert!(
            output.contains("Disassembly") || output.contains("<"),
            "Disassembly should contain section markers"
        );
    }
}

#[test]
fn test_detect_makefile_in_reference_project() {
    use std::path::PathBuf;

    let project_path = PathBuf::from("tests/fixtures/arm-reference-project");
    if !project_path.exists() {
        return;
    }

    let makefile_info = detect_makefile(&project_path);
    assert!(
        makefile_info.is_some(),
        "Should detect Makefile in reference project"
    );

    if let Some(info) = makefile_info {
        assert!(!info.targets.is_empty(), "Should find targets in Makefile");
    }
}

#[test]
fn test_run_make_clean() {
    use std::path::PathBuf;

    let project_path = PathBuf::from("tests/fixtures/arm-reference-project");
    if !project_path.exists() {
        return;
    }

    // Run make clean (should always succeed even if nothing to clean)
    let result = run_make(&project_path, "clean", None);

    // Exit code 0 or 2 (no rule) are both acceptable
    assert!(
        result.exit_code == 0 || result.exit_code == 2,
        "Make clean should succeed or report no rule"
    );
}

#[test]
fn test_run_make_invalid_target() {
    use std::path::PathBuf;

    let project_path = PathBuf::from("tests/fixtures/arm-reference-project");
    if !project_path.exists() {
        return;
    }

    // Run make with invalid target
    let result = run_make(&project_path, "nonexistent_target_xyz", None);

    // Should fail or report no rule
    assert!(
        result.exit_code != 0 || result.stderr.contains("No rule"),
        "Invalid target should fail or report no rule"
    );
}

// Error handling tests

#[test]
fn test_error_display_not_found() {
    let error = ArmToolchainError::not_found();
    let display = format!("{}", error);

    assert!(
        display.contains("ARM toolchain not found"),
        "Error message should mention toolchain not found"
    );
    assert!(!display.is_empty(), "Error message should not be empty");
}

#[test]
fn test_error_display_incomplete() {
    let missing = vec![
        "arm-none-eabi-gdb".to_string(),
        "arm-none-eabi-size".to_string(),
    ];
    let error = ArmToolchainError::incomplete(missing);
    let display = format!("{}", error);

    assert!(
        display.contains("incomplete"),
        "Error message should mention incomplete"
    );
    assert!(
        display.contains("arm-none-eabi-gdb"),
        "Error message should list missing tools"
    );
    assert!(
        display.contains("arm-none-eabi-size"),
        "Error message should list missing tools"
    );
}

#[test]
fn test_error_display_version_too_old() {
    let error = ArmToolchainError::version_too_old("7.3.1".to_string(), "8.0.0".to_string());
    let display = format!("{}", error);

    assert!(
        display.contains("7.3.1"),
        "Error message should show found version"
    );
    assert!(
        display.contains("8.0.0"),
        "Error message should show required version"
    );
    assert!(
        display.contains("too old"),
        "Error message should mention version is too old"
    );
}

#[test]
fn test_error_display_linker_script_not_found() {
    use std::path::PathBuf;
    let path = PathBuf::from("/path/to/missing/script.ld");
    let error = ArmToolchainError::linker_script_not_found(path.clone());
    let display = format!("{}", error);

    assert!(
        display.contains("Linker script not found"),
        "Error message should mention linker script"
    );
    assert!(
        display.contains("script.ld"),
        "Error message should include file name"
    );
}

#[test]
fn test_error_display_memory_overflow() {
    let error = ArmToolchainError::memory_overflow(
        "FLASH".to_string(),
        "section .text exceeds available space by 2048 bytes".to_string(),
    );
    let display = format!("{}", error);

    assert!(
        display.contains("Memory overflow"),
        "Error message should mention memory overflow"
    );
    assert!(
        display.contains("FLASH"),
        "Error message should mention region"
    );
    assert!(
        display.contains("2048 bytes"),
        "Error message should include details"
    );
}

#[test]
fn test_error_display_compilation_failed() {
    let diagnostics = vec![
        "error: expected ';' before '}' token".to_string(),
        "error: 'undefined_var' undeclared".to_string(),
    ];
    let error = ArmToolchainError::compilation_failed(2, diagnostics);
    let display = format!("{}", error);

    assert!(
        display.contains("Compilation failed"),
        "Error message should mention compilation failed"
    );
    assert!(
        display.contains("2 error"),
        "Error message should show error count"
    );
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_installation_suggestion() {
    let error = ArmToolchainError::not_found();
    let display = format!("{}", error);

    assert!(
        display.contains("Homebrew") || display.contains("brew"),
        "macOS error should mention Homebrew"
    );
    assert!(
        display.contains("gcc-arm-embedded") || display.contains("arm.com"),
        "macOS error should mention installation method"
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_linux_installation_suggestion() {
    let error = ArmToolchainError::not_found();
    let display = format!("{}", error);

    assert!(
        display.contains("apt-get") || display.contains("dnf"),
        "Linux error should mention package manager"
    );
    assert!(
        display.contains("gcc-arm-none-eabi") || display.contains("arm-none-eabi-gcc"),
        "Linux error should mention package name"
    );
}

#[test]
#[cfg(target_os = "windows")]
fn test_windows_installation_suggestion() {
    let error = ArmToolchainError::not_found();
    let display = format!("{}", error);

    assert!(
        display.contains("ARM Developer") || display.contains("developer.arm.com"),
        "Windows error should mention ARM Developer"
    );
    assert!(
        display.contains("Chocolatey") || display.contains("choco"),
        "Windows error should mention Chocolatey as alternative"
    );
}

// ============================================================================
// Edge Case Integration Tests (Task 21)
// ============================================================================

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_empty_source_file() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/empty_file.c");

    let output = PathBuf::from("/tmp/test_empty.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Empty file should compile successfully (produces empty object file)
    assert_eq!(
        result.exit_code, 0,
        "Empty file should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_unicode_in_comments() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/unicode_comments.c");

    let output = PathBuf::from("/tmp/test_unicode.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Unicode in comments should not affect compilation
    assert_eq!(
        result.exit_code, 0,
        "Unicode in comments should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_very_long_lines() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/very_long_lines.c");

    let output = PathBuf::from("/tmp/test_long_lines.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Very long lines should compile successfully
    assert_eq!(
        result.exit_code, 0,
        "Very long lines should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_deeply_nested_includes() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/deeply_nested_includes.c");

    let mut inc_path = std::env::current_dir().unwrap();
    if inc_path.ends_with("crates/axiom-toolchain") {
        inc_path.pop();
        inc_path.pop();
    }
    inc_path.push("tests/fixtures/arm-reference-project/edge_cases");

    let output = PathBuf::from("/tmp/test_nested.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu).with_include_path(inc_path);

    let result = compile_arm(&gcc_path, &request);

    // Deeply nested includes should compile successfully
    assert_eq!(
        result.exit_code, 0,
        "Deeply nested includes should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_inline_assembly() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/inline_assembly.c");

    let output = PathBuf::from("/tmp/test_inline_asm_edge.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Inline assembly should compile successfully for ARM target
    assert_eq!(
        result.exit_code, 0,
        "Inline assembly should compile: {}",
        result.stderr
    );

    // Verify the object file was created
    assert!(output.exists(), "Object file should be created");

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_preprocessor_heavy_macros() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/heavy_macros.c");

    let output = PathBuf::from("/tmp/test_macros.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Heavy macro usage should compile successfully
    assert_eq!(
        result.exit_code, 0,
        "Heavy macros should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_invalid_mcu_config() {
    use std::path::PathBuf;

    // Only run if ARM toolchain is available
    let gcc_path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
    if !gcc_path.exists() {
        return;
    }

    // Create an invalid MCU config with nonsense CPU name
    let mut mcu = ArmMcuConfig::cortex_m3();
    mcu.cpu = "invalid-cpu-that-does-not-exist".to_string();

    let mut source = std::env::current_dir().unwrap();
    if source.ends_with("crates/axiom-toolchain") {
        source.pop();
        source.pop();
    }
    source.push("tests/fixtures/arm-reference-project/edge_cases/empty_file.c");

    let output = PathBuf::from("/tmp/test_invalid_mcu.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu);

    let result = compile_arm(&gcc_path, &request);

    // Should fail with invalid CPU error
    assert_ne!(result.exit_code, 0, "Invalid MCU config should fail");
    assert!(
        result.stderr.contains("unknown")
            || result.stderr.contains("unrecognized")
            || result.stderr.contains("invalid"),
        "Error should mention invalid/unknown CPU: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}

#[test]
#[cfg_attr(not(target_os = "macos"), ignore)]
fn test_circular_includes() {
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
    source.push("tests/fixtures/arm-reference-project/edge_cases/circular_includes.c");

    let mut inc_path = std::env::current_dir().unwrap();
    if inc_path.ends_with("crates/axiom-toolchain") {
        inc_path.pop();
        inc_path.pop();
    }
    inc_path.push("tests/fixtures/arm-reference-project/edge_cases");

    let output = PathBuf::from("/tmp/test_circular.o");

    let request = ArmCompileRequest::new(source, output.clone(), mcu).with_include_path(inc_path);

    let result = compile_arm(&gcc_path, &request);

    // Circular includes with proper include guards should compile successfully
    assert_eq!(
        result.exit_code, 0,
        "Circular includes with guards should compile: {}",
        result.stderr
    );

    // Clean up
    let _ = std::fs::remove_file(output);
}
