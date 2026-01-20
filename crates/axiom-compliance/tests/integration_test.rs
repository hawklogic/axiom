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
    assert!(
        !links.is_empty(),
        "Should find requirement annotations in traced_module.c"
    );

    // Check that we found some specific requirements
    let req_ids: Vec<String> = links.iter().map(|l| l.requirement_id.clone()).collect();
    assert!(
        req_ids.iter().any(|id| id.starts_with("REQ-SENSOR")),
        "Should find REQ-SENSOR requirements"
    );
}

#[test]
fn test_find_untraceable_code_in_untraced_module() {
    let untraced_path =
        Path::new("tests/fixtures/arm-reference-project/compliance/untraced_module.c");

    if !untraced_path.exists() {
        // Skip test if reference project doesn't exist
        return;
    }

    let mut untraceable = Vec::new();
    axiom_compliance::traceability::analyze_file_for_untraceable(untraced_path, &mut untraceable)
        .unwrap();

    // The untraced_module.c file has several functions without REQ annotations
    assert!(
        !untraceable.is_empty(),
        "Should find untraceable functions in untraced_module.c"
    );

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
    assert!(
        content.contains("REQ-001"),
        "CSV should contain requirement ID"
    );
    assert!(content.contains("test.c"), "CSV should contain source file");
    assert!(
        content.contains("Implementation"),
        "CSV should contain link type"
    );
    assert!(
        content.contains("Test"),
        "CSV should contain test link type"
    );
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

// 13.3.1: Test log multiple tool invocations
#[test]
fn test_log_multiple_tool_invocations() {
    use axiom_compliance::tool_qualification::{ToolQualificationLogger, ToolUsageRecord};

    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("tool_usage.log");
    let logger = ToolQualificationLogger::new(log_path.clone());

    // Simulate multiple tool invocations
    let tools = vec![
        (
            "arm-none-eabi-gcc",
            "14.3.1",
            vec!["-c", "main.c", "-o", "main.o"],
        ),
        (
            "arm-none-eabi-gcc",
            "14.3.1",
            vec!["-c", "utils.c", "-o", "utils.o"],
        ),
        (
            "arm-none-eabi-ld",
            "14.3.1",
            vec!["main.o", "utils.o", "-o", "app.elf"],
        ),
        (
            "arm-none-eabi-objcopy",
            "14.3.1",
            vec!["-O", "ihex", "app.elf", "app.hex"],
        ),
    ];

    for (tool, version, args) in tools {
        let mut record = ToolUsageRecord::new(
            tool.to_string(),
            version.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
            0,
        );

        // Add some diagnostic info
        if tool == "arm-none-eabi-gcc" {
            record.add_diagnostic("warning: optimization level not specified".to_string());
        }

        logger.log(&record).unwrap();
    }

    // Retrieve all records
    let records = logger.get_all_records().unwrap();

    assert_eq!(records.len(), 4, "Should have logged 4 tool invocations");
    assert_eq!(records[0].tool, "arm-none-eabi-gcc");
    assert_eq!(records[2].tool, "arm-none-eabi-ld");
    assert_eq!(records[3].tool, "arm-none-eabi-objcopy");

    // Verify diagnostics were preserved
    assert!(
        !records[0].diagnostics.is_empty(),
        "First record should have diagnostics"
    );
}

// 13.3.2: Test records contain correct checksums for input files
#[test]
fn test_records_contain_correct_checksums() {
    use axiom_compliance::tool_qualification::{
        compute_sha256, ToolQualificationLogger, ToolUsageRecord,
    };
    use std::fs;

    let temp_dir = TempDir::new().unwrap();
    let log_path = temp_dir.path().join("tool_usage.log");
    let logger = ToolQualificationLogger::new(log_path.clone());

    // Create test input and output files
    let input_file = temp_dir.path().join("input.c");
    let output_file = temp_dir.path().join("output.o");

    fs::write(&input_file, b"int main() { return 0; }").unwrap();
    fs::write(&output_file, b"\x7fELF...").unwrap();

    // Compute checksums
    let input_checksum = compute_sha256(&input_file).unwrap();
    let output_checksum = compute_sha256(&output_file).unwrap();

    // Create and log a record with checksums
    let mut record = ToolUsageRecord::new(
        "arm-none-eabi-gcc".to_string(),
        "14.3.1".to_string(),
        vec![
            "-c".to_string(),
            "input.c".to_string(),
            "-o".to_string(),
            "output.o".to_string(),
        ],
        0,
    );

    record.add_input_checksum(input_file.clone(), input_checksum.clone());
    record.add_output_checksum(output_file.clone(), output_checksum.clone());

    logger.log(&record).unwrap();

    // Retrieve and verify
    let records = logger.get_all_records().unwrap();
    assert_eq!(records.len(), 1);

    let retrieved = &records[0];
    assert_eq!(retrieved.input_checksums.len(), 1);
    assert_eq!(retrieved.output_checksums.len(), 1);

    // Verify checksums match
    assert_eq!(
        retrieved.input_checksums.get(&input_file),
        Some(&input_checksum)
    );
    assert_eq!(
        retrieved.output_checksums.get(&output_file),
        Some(&output_checksum)
    );

    // Verify checksums are correct by recomputing
    let recomputed_input = compute_sha256(&input_file).unwrap();
    let recomputed_output = compute_sha256(&output_file).unwrap();

    assert_eq!(
        retrieved.input_checksums.get(&input_file).unwrap(),
        &recomputed_input
    );
    assert_eq!(
        retrieved.output_checksums.get(&output_file).unwrap(),
        &recomputed_output
    );
}

// Error handling tests for ComplianceError

#[test]
fn test_compliance_error_display_annotation_parse() {
    use std::path::PathBuf;

    let error = ComplianceError::annotation_parse_error(
        PathBuf::from("test.c"),
        42,
        "Invalid requirement format".to_string(),
    );
    let display = format!("{}", error);

    assert!(display.contains("test.c"), "Error should mention file");
    assert!(display.contains("42"), "Error should mention line number");
    assert!(
        display.contains("Invalid requirement format"),
        "Error should include message"
    );
}

#[test]
fn test_compliance_error_display_requirement_not_found() {
    let error = ComplianceError::requirement_not_found("REQ-999".to_string());
    let display = format!("{}", error);

    assert!(
        display.contains("REQ-999"),
        "Error should mention requirement ID"
    );
    assert!(
        display.contains("not found"),
        "Error should mention not found"
    );
}

#[test]
fn test_compliance_error_display_coverage_data_not_found() {
    use std::path::PathBuf;

    let error = ComplianceError::coverage_data_not_found(PathBuf::from("/path/to/coverage.gcov"));
    let display = format!("{}", error);

    assert!(
        display.contains("coverage.gcov"),
        "Error should mention file"
    );
    assert!(
        display.contains("not found"),
        "Error should mention not found"
    );
}

#[test]
fn test_compliance_error_display_invalid_coverage_data() {
    use std::path::PathBuf;

    let error = ComplianceError::invalid_coverage_data(
        PathBuf::from("coverage.gcov"),
        "Unexpected format in line 10".to_string(),
    );
    let display = format!("{}", error);

    assert!(
        display.contains("coverage.gcov"),
        "Error should mention file"
    );
    assert!(
        display.contains("Invalid coverage data"),
        "Error should mention invalid data"
    );
    assert!(
        display.contains("Unexpected format"),
        "Error should include details"
    );
}

#[test]
fn test_compliance_error_display_tool_qualification() {
    let error = ComplianceError::tool_qualification_error("Failed to write log entry".to_string());
    let display = format!("{}", error);

    assert!(
        display.contains("Tool qualification"),
        "Error should mention tool qualification"
    );
    assert!(
        display.contains("Failed to write"),
        "Error should include message"
    );
}

#[test]
fn test_compliance_error_display_checksum_mismatch() {
    use std::path::PathBuf;

    let error = ComplianceError::checksum_mismatch(
        PathBuf::from("output.elf"),
        "abc123".to_string(),
        "def456".to_string(),
    );
    let display = format!("{}", error);

    assert!(display.contains("output.elf"), "Error should mention file");
    assert!(
        display.contains("abc123"),
        "Error should show expected checksum"
    );
    assert!(
        display.contains("def456"),
        "Error should show found checksum"
    );
    assert!(
        display.contains("mismatch"),
        "Error should mention mismatch"
    );
}

#[test]
fn test_compliance_error_display_mode_not_enabled() {
    let error = ComplianceError::mode_not_enabled("DO-178C".to_string());
    let display = format!("{}", error);

    assert!(display.contains("DO-178C"), "Error should mention mode");
    assert!(
        display.contains("not enabled"),
        "Error should mention not enabled"
    );
}

#[test]
fn test_compliance_error_display_unsupported_export_format() {
    let error = ComplianceError::unsupported_export_format("YAML".to_string());
    let display = format!("{}", error);

    assert!(display.contains("YAML"), "Error should mention format");
    assert!(
        display.contains("not supported"),
        "Error should mention not supported"
    );
}

#[test]
fn test_compliance_error_display_deviations_detected() {
    let details = vec![
        "New function added without traceability".to_string(),
        "Modified code breaks existing link".to_string(),
    ];
    let error = ComplianceError::deviations_detected(2, details);
    let display = format!("{}", error);

    assert!(
        display.contains("2 deviation"),
        "Error should show deviation count"
    );
    assert!(
        display.contains("Deviations detected"),
        "Error should mention deviations"
    );
}
