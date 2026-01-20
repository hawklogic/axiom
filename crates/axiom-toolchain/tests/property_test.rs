// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Property-based tests for ARM toolchain.

use axiom_toolchain::*;
use proptest::prelude::*;

proptest! {
    /// P1: For any returned toolchain, gcc path exists and version is non-empty
    #[test]
    fn prop_detected_toolchains_have_valid_gcc_and_version(
        _dummy in 0..1u8  // Dummy input since we're testing detect_all()
    ) {
        let toolchains = detect_arm_toolchains();
        
        for suite in toolchains {
            // Property 1: GCC path must exist
            prop_assert!(suite.gcc.exists(), 
                "GCC path {:?} should exist", suite.gcc);
            
            // Property 2: Version must be non-empty
            prop_assert!(!suite.version.is_empty(), 
                "Version should not be empty");
            
            // Property 3: Version should contain at least one digit
            prop_assert!(suite.version.chars().any(|c| c.is_ascii_digit()),
                "Version {:?} should contain digits", suite.version);
        }
    }
}


proptest! {
    /// P2: For any ArmMcuConfig, flags contain -mcpu with specified CPU
    #[test]
    fn prop_arm_mcu_config_contains_cpu(
        cpu in "[a-z0-9-]{3,20}"
    ) {
        let config = ArmMcuConfig::new(cpu.clone());
        let flags = config.compiler_flags();
        
        let expected_flag = format!("-mcpu={}", cpu);
        prop_assert!(flags.contains(&expected_flag),
            "Flags should contain -mcpu={}", cpu);
    }
}
