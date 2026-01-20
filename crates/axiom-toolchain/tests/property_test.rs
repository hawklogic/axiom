// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Property-based tests for ARM toolchain.

use axiom_toolchain::*;
use proptest::prelude::*;

// Arbitrary generator for FloatAbi
fn arb_float_abi() -> impl Strategy<Value = FloatAbi> {
    prop_oneof![
        Just(FloatAbi::Soft),
        Just(FloatAbi::SoftFp),
        Just(FloatAbi::Hard),
    ]
}

// Arbitrary generator for ArmMcuConfig
fn arb_arm_mcu_config() -> impl Strategy<Value = ArmMcuConfig> {
    (
        "[a-z0-9-]{3,20}",  // cpu
        any::<bool>(),       // thumb
        prop::option::of("[a-z0-9-]{3,15}"),  // fpu (optional)
        arb_float_abi(),     // float_abi
        prop::collection::vec("[A-Z0-9_]{3,20}", 0..5),  // defines
    )
        .prop_map(|(cpu, thumb, fpu, float_abi, defines)| {
            let mut config = ArmMcuConfig::new(cpu);
            config.thumb = thumb;
            config.fpu = fpu.unwrap_or_default();
            config.float_abi = float_abi;
            config.defines = defines;
            config
        })
}

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


proptest! {
    /// P3: Include paths in flags match order in request
    #[test]
    fn prop_include_paths_preserve_order(
        paths in proptest::collection::vec("[a-z]{3,10}", 1..5)
    ) {
        use std::path::PathBuf;
        
        let mcu = ArmMcuConfig::cortex_m3();
        let mut request = ArmCompileRequest::new(
            PathBuf::from("test.c"),
            PathBuf::from("test.o"),
            mcu,
        );
        
        // Add paths in order
        for path in &paths {
            request = request.with_include_path(path);
        }
        
        let args = build_arm_compile_command(
            std::path::Path::new("/usr/bin/arm-none-eabi-gcc"),
            &request,
        );
        
        // Find positions of -I flags
        let mut positions = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            if arg.starts_with("-I") {
                positions.push((i, &arg[2..]));
            }
        }
        
        // Verify order matches
        for (i, path) in paths.iter().enumerate() {
            if i < positions.len() {
                prop_assert_eq!(positions[i].1, path.as_str(),
                    "Include path order should be preserved");
            }
        }
    }
}

proptest! {
    /// P5: Version parsing extracts valid semver components
    /// **Validates: Requirements 1, 2, 11, 12**
    #[test]
    fn prop_version_parsing_extracts_valid_semver(
        major in 0u32..100,
        minor in 0u32..100,
        patch in 0u32..100,
    ) {
        // Generate a version string in semver format
        let version_str = format!("{}.{}.{}", major, minor, patch);
        
        // Create a mock GCC output with the version
        let gcc_output = format!(
            "arm-none-eabi-gcc (GNU Arm Embedded Toolchain 10.3-2021.10) {}\nCopyright...",
            version_str
        );
        
        // Parse the version
        let parsed = parse_arm_gcc_version(&gcc_output);
        
        // Property: Should successfully parse the version
        prop_assert!(parsed.is_some(), "Should parse version from output");
        
        let parsed_version = parsed.unwrap();
        
        // Property: Parsed version should contain digits
        prop_assert!(
            parsed_version.chars().any(|c| c.is_ascii_digit()),
            "Parsed version should contain digits"
        );
        
        // Property: Parsed version should contain dots (for multi-part versions)
        if major > 0 || minor > 0 || patch > 0 {
            prop_assert!(
                parsed_version.contains('.'),
                "Multi-part version should contain dots"
            );
        }
        
        // Property: Should be able to extract version components
        let parts: Vec<&str> = parsed_version.split('.').collect();
        prop_assert!(
            !parts.is_empty(),
            "Should have at least one version component"
        );
        
        // Property: Each component should be parseable as a number
        for part in &parts {
            let trimmed = part.trim_end_matches(|c: char| !c.is_ascii_digit());
            if !trimmed.is_empty() {
                prop_assert!(
                    trimmed.parse::<u32>().is_ok(),
                    "Version component '{}' should be parseable as number",
                    trimmed
                );
            }
        }
        
        // Property: Parsed version should start with the major version
        prop_assert!(
            parsed_version.starts_with(&major.to_string()),
            "Parsed version should start with major version {}", major
        );
    }
}
