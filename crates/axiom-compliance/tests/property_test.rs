// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Property-based tests for axiom-compliance crate

use axiom_compliance::*;
use proptest::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

// Arbitrary generator for LinkType
fn arb_link_type() -> impl Strategy<Value = LinkType> {
    prop_oneof![
        Just(LinkType::Implementation),
        Just(LinkType::Test),
        Just(LinkType::Derived),
    ]
}

// Arbitrary generator for TraceabilityLink
fn arb_traceability_link() -> impl Strategy<Value = TraceabilityLink> {
    (
        "REQ-[A-Z]{3}-[0-9]{3}", // requirement_id
        "[a-z]{3,10}\\.c",       // source_file
        1u32..1000,              // line_number
        arb_link_type(),         // link_type
    )
        .prop_map(|(req_id, file, line, link_type)| {
            TraceabilityLink::new(req_id, PathBuf::from(file), line, link_type)
        })
}

// Property P6: All annotated requirements appear in generated matrix
proptest! {
    #[test]
    fn prop_all_annotated_requirements_appear_in_matrix(
        req_suffixes in prop::collection::vec("[A-Z]{3}-[0-9]{3}", 1..10)
    ) {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.c");

        // Generate requirement IDs with REQ- prefix to match the regex pattern
        let req_ids: Vec<String> = req_suffixes.iter()
            .map(|suffix| format!("REQ-{}", suffix))
            .collect();

        // Generate a C file with the given requirement IDs
        let mut content = String::new();
        content.push_str("#include <stdint.h>\n\n");

        for (i, req_id) in req_ids.iter().enumerate() {
            content.push_str(&format!("// {}: Function {}\n", req_id, i));
            content.push_str(&format!("void function_{}(void) {{\n", i));
            content.push_str("    // Implementation\n");
            content.push_str("}\n\n");
        }

        std::fs::write(&test_file, content).unwrap();

        // Parse the file
        let links = parse_requirement_annotations(&test_file).unwrap();

        // Extract requirement IDs from links
        let found_reqs: std::collections::HashSet<String> =
            links.iter().map(|l| l.requirement_id.clone()).collect();

        // Property: All requirement IDs we wrote should be found
        for req_id in &req_ids {
            prop_assert!(
                found_reqs.contains(req_id),
                "Requirement {} should be found in parsed links",
                req_id
            );
        }

        // Property: We shouldn't find more requirements than we wrote
        prop_assert_eq!(
            found_reqs.len(),
            req_ids.len(),
            "Should find exactly the requirements we wrote"
        );
    }
}

proptest! {
    #[test]
    fn prop_matrix_preserves_all_added_links(
        num_links in 1usize..20
    ) {
        let mut matrix = TraceabilityMatrix::new();
        let mut added_req_ids = Vec::new();

        // Add links to the matrix
        for i in 0..num_links {
            let req_id = format!("REQ-{:03}", i);
            added_req_ids.push(req_id.clone());

            matrix.add_link(TraceabilityLink::new(
                req_id,
                PathBuf::from(format!("file{}.c", i)),
                (i as u32) + 1,
                LinkType::Implementation,
            ));
        }

        // Property: All added requirements should be retrievable
        let all_reqs = matrix.get_all_requirements();
        prop_assert_eq!(
            all_reqs.len(),
            num_links,
            "Matrix should contain all added requirements"
        );

        for req_id in &added_req_ids {
            let links = matrix.get_links_for_requirement(req_id);
            prop_assert!(
                !links.is_empty(),
                "Should be able to retrieve links for requirement {}",
                req_id
            );
        }
    }
}

proptest! {
    #[test]
    fn prop_untested_requirements_detection_is_consistent(
        num_impl in 1usize..10,
        num_test in 0usize..10
    ) {
        let mut matrix = TraceabilityMatrix::new();

        // Add implementation links
        for i in 0..num_impl {
            matrix.add_link(TraceabilityLink::new(
                format!("REQ-{:03}", i),
                PathBuf::from("impl.c"),
                (i as u32) + 1,
                LinkType::Implementation,
            ));
        }

        // Add test links for some requirements
        for i in 0..num_test.min(num_impl) {
            matrix.add_link(TraceabilityLink::new(
                format!("REQ-{:03}", i),
                PathBuf::from("test.c"),
                (i as u32) + 1,
                LinkType::Test,
            ));
        }

        let untested = find_untested_requirements(&matrix);

        // Property: Number of untested should be implementation count minus test count
        let expected_untested = if num_test < num_impl {
            num_impl - num_test
        } else {
            0
        };

        prop_assert_eq!(
            untested.len(),
            expected_untested,
            "Should find correct number of untested requirements"
        );

        // Property: All untested requirements should have implementation but no test
        for req_id in &untested {
            let links = matrix.get_links_for_requirement(req_id);
            let has_impl = links.iter().any(|l| l.link_type == LinkType::Implementation);
            let has_test = links.iter().any(|l| l.link_type == LinkType::Test);

            prop_assert!(has_impl, "Untested requirement {} should have implementation", req_id);
            prop_assert!(!has_test, "Untested requirement {} should not have test", req_id);
        }
    }
}

// Property P7: All logged invocations are retrievable with correct data
proptest! {
    #[test]
    fn prop_all_logged_invocations_retrievable(
        num_invocations in 1usize..20,
        tool_name in "[a-z-]{5,20}",
        version in "[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}"
    ) {
        use axiom_compliance::tool_qualification::{ToolQualificationLogger, ToolUsageRecord};

        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("tool_usage.log");
        let logger = ToolQualificationLogger::new(log_path.clone());

        let mut expected_records = Vec::new();

        // Log multiple invocations
        for i in 0..num_invocations {
            let mut record = ToolUsageRecord::new(
                tool_name.clone(),
                version.clone(),
                vec![format!("arg{}", i), format!("file{}.c", i)],
                i as i32,
            );

            // Add some checksums
            record.add_input_checksum(
                PathBuf::from(format!("input{}.c", i)),
                format!("checksum{:064x}", i),
            );
            record.add_output_checksum(
                PathBuf::from(format!("output{}.o", i)),
                format!("checksum{:064x}", i + 1000),
            );

            // Add diagnostics
            if i % 2 == 0 {
                record.add_diagnostic(format!("diagnostic message {}", i));
            }

            expected_records.push(record.clone());
            logger.log(&record).unwrap();
        }

        // Retrieve all records
        let retrieved_records = logger.get_all_records().unwrap();

        // Property: Should retrieve exactly the number of records we logged
        prop_assert_eq!(
            retrieved_records.len(),
            num_invocations,
            "Should retrieve all logged records"
        );

        // Property: Each retrieved record should match the corresponding expected record
        for (i, (expected, retrieved)) in expected_records.iter().zip(retrieved_records.iter()).enumerate() {
            prop_assert_eq!(
                &retrieved.tool,
                &expected.tool,
                "Record {} tool name should match", i
            );
            prop_assert_eq!(
                &retrieved.version,
                &expected.version,
                "Record {} version should match", i
            );
            prop_assert_eq!(
                &retrieved.arguments,
                &expected.arguments,
                "Record {} arguments should match", i
            );
            prop_assert_eq!(
                retrieved.exit_code,
                expected.exit_code,
                "Record {} exit code should match", i
            );
            prop_assert_eq!(
                &retrieved.input_checksums,
                &expected.input_checksums,
                "Record {} input checksums should match", i
            );
            prop_assert_eq!(
                &retrieved.output_checksums,
                &expected.output_checksums,
                "Record {} output checksums should match", i
            );
            prop_assert_eq!(
                &retrieved.diagnostics,
                &expected.diagnostics,
                "Record {} diagnostics should match", i
            );
        }
    }
}

proptest! {
    #[test]
    fn prop_logger_preserves_data_integrity(
        tools in prop::collection::vec(
            ("[a-z-]{3,10}", "[0-9]\\.[0-9]", prop::collection::vec("[a-z]{2,5}", 1..5)),
            1..10
        )
    ) {
        use axiom_compliance::tool_qualification::{ToolQualificationLogger, ToolUsageRecord};

        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("tool_usage.log");
        let logger = ToolQualificationLogger::new(log_path.clone());

        // Log records
        for (i, (tool, version, args)) in tools.iter().enumerate() {
            let record = ToolUsageRecord::new(
                tool.clone(),
                version.clone(),
                args.clone(),
                i as i32,
            );
            logger.log(&record).unwrap();
        }

        // Retrieve records
        let records = logger.get_all_records().unwrap();

        // Property: Data integrity - all fields preserved correctly
        prop_assert_eq!(records.len(), tools.len());

        for (i, ((tool, version, args), record)) in tools.iter().zip(records.iter()).enumerate() {
            prop_assert_eq!(&record.tool, tool);
            prop_assert_eq!(&record.version, version);
            prop_assert_eq!(&record.arguments, args);
            prop_assert_eq!(record.exit_code, i as i32);
        }
    }
}

// Property P8: Data survives mode disable/re-enable cycle
proptest! {
    #[test]
    fn prop_data_survives_mode_disable_reenable_cycle(
        num_files in 1usize..10,
        num_requirements in 1usize..20
    ) {
        use axiom_compliance::{ComplianceSystem, ComplianceMode, ComplianceSnapshot};

        let mut system = ComplianceSystem::new();

        // Enable a mode
        system.enable_mode(ComplianceMode::Do178c);
        prop_assert!(system.is_mode_enabled(ComplianceMode::Do178c));

        // Create a snapshot with data
        let mut snapshot = ComplianceSnapshot::new();

        // Add file checksums
        for i in 0..num_files {
            snapshot.file_checksums.insert(
                PathBuf::from(format!("file{}.c", i)),
                format!("checksum{:064x}", i),
            );
            snapshot.traced_files.insert(PathBuf::from(format!("file{}.c", i)));
        }

        // Add requirement IDs
        for i in 0..num_requirements {
            snapshot.traced_requirements.insert(format!("REQ-{:03}", i));
        }

        let original_timestamp = snapshot.timestamp;
        let original_file_count = snapshot.file_checksums.len();
        let original_req_count = snapshot.traced_requirements.len();

        // Disable the mode with the snapshot
        system.disable_mode(ComplianceMode::Do178c, Some(snapshot.clone()));
        prop_assert!(!system.is_mode_enabled(ComplianceMode::Do178c));

        // Verify snapshot is preserved
        let preserved = system.get_snapshot(ComplianceMode::Do178c);
        prop_assert!(preserved.is_some(), "Snapshot should be preserved after disable");

        let preserved = preserved.unwrap();

        // Property: All data should survive the disable
        prop_assert_eq!(
            preserved.file_checksums.len(),
            original_file_count,
            "File checksums count should be preserved"
        );
        prop_assert_eq!(
            preserved.traced_requirements.len(),
            original_req_count,
            "Traced requirements count should be preserved"
        );
        prop_assert_eq!(
            preserved.timestamp,
            original_timestamp,
            "Timestamp should be preserved"
        );

        // Verify specific data
        for i in 0..num_files {
            let path = PathBuf::from(format!("file{}.c", i));
            let expected_checksum = format!("checksum{:064x}", i);

            prop_assert!(
                preserved.file_checksums.contains_key(&path),
                "File {} should be in preserved checksums", i
            );
            prop_assert_eq!(
                preserved.file_checksums.get(&path),
                Some(&expected_checksum),
                "Checksum for file {} should match", i
            );
            prop_assert!(
                preserved.traced_files.contains(&path),
                "File {} should be in traced files", i
            );
        }

        for i in 0..num_requirements {
            let req_id = format!("REQ-{:03}", i);
            prop_assert!(
                preserved.traced_requirements.contains(&req_id),
                "Requirement {} should be preserved", req_id
            );
        }

        // Re-enable the mode
        let report = system.enable_mode(ComplianceMode::Do178c);
        prop_assert!(system.is_mode_enabled(ComplianceMode::Do178c));

        // Property: Re-enabling should return a deviation report
        prop_assert!(report.is_some(), "Re-enabling should return a deviation report");

        let report = report.unwrap();
        prop_assert_eq!(report.mode, ComplianceMode::Do178c);
        prop_assert_eq!(report.disabled_at, original_timestamp);

        // After re-enable, snapshot should be consumed
        let snapshot_after = system.get_snapshot(ComplianceMode::Do178c);
        prop_assert!(
            snapshot_after.is_none(),
            "Snapshot should be consumed after re-enable"
        );
    }
}

proptest! {
    #[test]
    fn prop_multiple_modes_preserve_data_independently(
        modes_to_test in prop::sample::subsequence(
            vec![ComplianceMode::Do178c, ComplianceMode::Do330, ComplianceMode::Arp4754a],
            1..=3
        )
    ) {
        use axiom_compliance::{ComplianceSystem, ComplianceSnapshot};

        let mut system = ComplianceSystem::new();

        // Enable all selected modes and create snapshots
        let mut snapshots = std::collections::HashMap::new();

        for (i, mode) in modes_to_test.iter().enumerate() {
            system.enable_mode(*mode);

            let mut snapshot = ComplianceSnapshot::new();
            snapshot.file_checksums.insert(
                PathBuf::from(format!("mode_{}_file.c", i)),
                format!("checksum_{}", i),
            );
            snapshot.traced_requirements.insert(format!("REQ-MODE-{}", i));

            snapshots.insert(*mode, snapshot.clone());
            system.disable_mode(*mode, Some(snapshot));
        }

        // Property: Each mode should have its own independent snapshot
        for (mode, original_snapshot) in &snapshots {
            let preserved = system.get_snapshot(*mode);
            prop_assert!(
                preserved.is_some(),
                "Mode {:?} should have preserved snapshot", mode
            );

            let preserved = preserved.unwrap();
            prop_assert_eq!(
                &preserved.file_checksums,
                &original_snapshot.file_checksums,
                "Mode {:?} file checksums should match", mode
            );
            prop_assert_eq!(
                &preserved.traced_requirements,
                &original_snapshot.traced_requirements,
                "Mode {:?} traced requirements should match", mode
            );
        }
    }
}
