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
