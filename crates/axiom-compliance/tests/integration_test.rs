// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Integration tests for axiom-compliance crate

use axiom_compliance::*;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_generate_matrix_from_reference_project_traced_module() {
    let traced_path = Path::new("tests/fixtures/arm-reference-project/compliance/traced_module.c");
    
    if !traced_path.exists() {
        // Skip test if reference project doesn't exist
        return;
    }
    
    // Parse the traced module
    let links = parse_requirement_annotations(traced_path).unwrap();
    
    // The traced_module.c file has multiple REQ annotations
    assert!(!links.is_empty(), "Should find requirement annotations in traced_module.c");
    
    // Check that we found some specific requirements
    let req_ids: Vec<String> = links.iter().map(|l| l.requirement_id.clone()).collect();
    assert!(req_ids.iter().any(|id| id.starts_with("REQ-SENSOR")), 
        "Should find REQ-SENSOR requirements");
}

#[test]
fn test_find_untraceable_code_in_untraced_module() {
    let untraced_path = Path::new("tests/fixtures/arm-reference-project/compliance/untraced_module.c");
    
    if !untraced_path.exists() {
        // Skip test if reference project doesn't exist
        return;
    }
    
    let mut untraceable = Vec::new();
    axiom_compliance::traceability::analyze_file_for_untraceable(untraced_path, &mut untraceable).unwrap();
    
    // The untraced_module.c file has several functions without REQ annotations
    assert!(!untraceable.is_empty(), "Should find untraceable functions in untraced_module.c");
    
    // Check for specific function names
    let func_names: Vec<String> = untraceable.iter().map(|f| f.name.clone()).collect();
    println!("Found untraceable functions: {:?}", func_names);
}

#[test]
fn test_export_matrix_to_csv() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("traceability.csv");
    
    // Create a simple matrix
    let mut matrix = TraceabilityMatrix::new();
    matrix.add_link(TraceabilityLink::new(
        "REQ-001".to_string(),
        Path::new("test.c").to_path_buf(),
        10,
        LinkType::Implementation,
    ));
    matrix.add_link(TraceabilityLink::new(
        "REQ-001".to_string(),
        Path::new("test_test.c").to_path_buf(),
        5,
        LinkType::Test,
    ));
    
    // Export to CSV
    export_matrix_csv(&matrix, &csv_path).unwrap();
    
    // Verify the CSV file was created and contains expected content
    assert!(csv_path.exists(), "CSV file should be created");
    
    let content = std::fs::read_to_string(&csv_path).unwrap();
    assert!(content.contains("Requirement ID"), "CSV should have header");
    assert!(content.contains("REQ-001"), "CSV should contain requirement ID");
    assert!(content.contains("test.c"), "CSV should contain source file");
    assert!(content.contains("Implementation"), "CSV should contain link type");
    assert!(content.contains("Test"), "CSV should contain test link type");
}

#[test]
fn test_full_traceability_workflow() {
    let project_path = Path::new("tests/fixtures/arm-reference-project/compliance");
    
    if !project_path.exists() {
        // Skip test if reference project doesn't exist
        return;
    }
    
    // Generate traceability matrix from the compliance directory
    let matrix = generate_traceability_matrix(project_path).unwrap();
    
    // Should find requirements from traced_module.c
    assert!(!matrix.links.is_empty(), "Should find traceability links");
    
    let all_reqs = matrix.get_all_requirements();
    assert!(!all_reqs.is_empty(), "Should have requirements");
    
    // Find untested requirements
    let untested = find_untested_requirements(&matrix);
    println!("Untested requirements: {:?}", untested);
    
    // Find untraceable functions
    let untraceable = find_untraceable_functions(project_path).unwrap();
    println!("Untraceable functions: {} found", untraceable.len());
    
    // Export to CSV
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("matrix.csv");
    export_matrix_csv(&matrix, &csv_path).unwrap();
    assert!(csv_path.exists(), "CSV export should succeed");
}
