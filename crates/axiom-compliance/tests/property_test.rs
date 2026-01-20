// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Property-based tests for axiom-compliance crate

use axiom_compliance::*;
use proptest::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

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
