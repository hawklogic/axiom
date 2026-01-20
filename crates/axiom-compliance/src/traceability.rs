// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! # Traceability System
//!
//! Provides bidirectional traceability between requirements, source code, and test cases
//! for DO-178C compliance.
//!
//! ## Safety-Critical Context
//!
//! This module supports traceability requirements for safety-critical avionics software.
//! Incorrect traceability could mask safety-critical defects and lead to catastrophic
//! failures. All traceability data is human-authored and verified.
//!
//! **CRITICAL**: No AI-generated content is used in any traceability functionality.

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during traceability operations.
///
/// These errors represent various failure modes when parsing traceability
/// annotations, generating matrices, or exporting reports.
#[derive(Debug, Error)]
pub enum TraceabilityError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse CSV: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Invalid requirement ID format: {0}")]
    InvalidRequirementId(String),
}

/// Result type for traceability operations.
///
/// A convenience type alias for `Result<T, TraceabilityError>`.
pub type Result<T> = std::result::Result<T, TraceabilityError>;

/// Type of traceability link.
///
/// Defines the relationship between a requirement and an artifact:
/// - `Implementation`: Links requirement to source code that implements it
/// - `Test`: Links requirement to test cases that verify it
/// - `Derived`: Links derived requirements to parent requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkType {
    /// Link from requirement to implementation code
    Implementation,
    /// Link from requirement to test case
    Test,
    /// Link for derived requirements
    Derived,
}

/// A traceability link between a requirement and an artifact.
///
/// Represents a single bidirectional link in the traceability matrix,
/// connecting a requirement identifier to a specific location in source code.
///
/// # Examples
///
/// ```
/// use axiom_compliance::{TraceabilityLink, LinkType};
/// use std::path::PathBuf;
///
/// let link = TraceabilityLink::new(
///     "REQ-001".to_string(),
///     PathBuf::from("src/main.c"),
///     42,
///     LinkType::Implementation,
/// );
///
/// assert_eq!(link.requirement_id, "REQ-001");
/// assert_eq!(link.line_number, 42);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceabilityLink {
    /// Requirement identifier (e.g., "REQ-001", "REQ-SENSOR-001.1")
    pub requirement_id: String,

    /// Source file path containing the link
    pub source_file: PathBuf,

    /// Line number in the source file
    pub line_number: u32,

    /// Type of link (implementation, test, or derived)
    pub link_type: LinkType,

    /// Timestamp when the link was created
    pub created_at: DateTime<Utc>,
}

impl TraceabilityLink {
    /// Create a new traceability link.
    ///
    /// # Arguments
    ///
    /// * `requirement_id` - The requirement identifier (e.g., "REQ-001")
    /// * `source_file` - Path to the file containing the link
    /// * `line_number` - Line number in the file (1-based)
    /// * `link_type` - Type of link (implementation, test, or derived)
    ///
    /// # Returns
    ///
    /// A new `TraceabilityLink` with the current timestamp
    pub fn new(
        requirement_id: String,
        source_file: PathBuf,
        line_number: u32,
        link_type: LinkType,
    ) -> Self {
        Self {
            requirement_id,
            source_file,
            line_number,
            link_type,
            created_at: Utc::now(),
        }
    }
}

/// A traceability matrix containing all links for a project.
///
/// The matrix maintains bidirectional traceability between requirements and
/// artifacts, with efficient indexing by requirement ID and source file.
///
/// # Examples
///
/// ```
/// use axiom_compliance::{TraceabilityMatrix, TraceabilityLink, LinkType};
/// use std::path::PathBuf;
///
/// let mut matrix = TraceabilityMatrix::new();
///
/// let link = TraceabilityLink::new(
///     "REQ-001".to_string(),
///     PathBuf::from("main.c"),
///     10,
///     LinkType::Implementation,
/// );
///
/// matrix.add_link(link);
/// assert_eq!(matrix.links.len(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityMatrix {
    /// All traceability links in the project
    pub links: Vec<TraceabilityLink>,

    /// Map from requirement ID to all links for that requirement
    requirement_index: HashMap<String, Vec<usize>>,

    /// Map from source file to all links in that file
    file_index: HashMap<PathBuf, Vec<usize>>,

    /// Timestamp when the matrix was generated
    pub generated_at: DateTime<Utc>,
}

impl TraceabilityMatrix {
    /// Create a new empty traceability matrix.
    ///
    /// # Returns
    ///
    /// An empty matrix with the current timestamp
    pub fn new() -> Self {
        Self {
            links: Vec::new(),
            requirement_index: HashMap::new(),
            file_index: HashMap::new(),
            generated_at: Utc::now(),
        }
    }

    /// Add a link to the matrix.
    ///
    /// Updates the internal indices for efficient lookup by requirement ID and file path.
    ///
    /// # Arguments
    ///
    /// * `link` - The traceability link to add
    pub fn add_link(&mut self, link: TraceabilityLink) {
        let index = self.links.len();

        // Add to requirement index
        self.requirement_index
            .entry(link.requirement_id.clone())
            .or_default()
            .push(index);

        // Add to file index
        self.file_index
            .entry(link.source_file.clone())
            .or_default()
            .push(index);

        self.links.push(link);
    }

    /// Get all links for a specific requirement.
    ///
    /// # Arguments
    ///
    /// * `requirement_id` - The requirement identifier to search for
    ///
    /// # Returns
    ///
    /// A vector of references to all links for the specified requirement
    pub fn get_links_for_requirement(&self, requirement_id: &str) -> Vec<&TraceabilityLink> {
        self.requirement_index
            .get(requirement_id)
            .map(|indices| indices.iter().filter_map(|&i| self.links.get(i)).collect())
            .unwrap_or_default()
    }

    /// Get all links in a specific file.
    ///
    /// # Arguments
    ///
    /// * `file` - The file path to search for
    ///
    /// # Returns
    ///
    /// A vector of references to all links in the specified file
    pub fn get_links_in_file(&self, file: &Path) -> Vec<&TraceabilityLink> {
        self.file_index
            .get(file)
            .map(|indices| indices.iter().filter_map(|&i| self.links.get(i)).collect())
            .unwrap_or_default()
    }

    /// Get all unique requirement IDs in the matrix.
    ///
    /// # Returns
    ///
    /// A sorted vector of all requirement identifiers
    pub fn get_all_requirements(&self) -> Vec<String> {
        let mut reqs: Vec<String> = self.requirement_index.keys().cloned().collect();
        reqs.sort();
        reqs
    }

    /// Get all unique source files in the matrix.
    ///
    /// # Returns
    ///
    /// A sorted vector of all file paths
    pub fn get_all_files(&self) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = self.file_index.keys().cloned().collect();
        files.sort();
        files
    }
}

impl Default for TraceabilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse requirement annotations from source code.
///
/// Searches for requirement identifiers in source code comments using patterns like:
/// - `// REQ-001`
/// - `/* REQ-SENSOR-001 */`
/// - `REQ-001.1` (sub-requirements)
///
/// # Arguments
///
/// * `file_path` - Path to the source file to parse
///
/// # Returns
///
/// A vector of traceability links found in the file
///
/// # Errors
///
/// Returns `TraceabilityError` if the file cannot be read
///
/// # Examples
///
/// ```no_run
/// use axiom_compliance::parse_requirement_annotations;
/// use std::path::Path;
///
/// let links = parse_requirement_annotations(Path::new("src/main.c")).unwrap();
/// for link in links {
///     println!("Found requirement: {} at line {}", link.requirement_id, link.line_number);
/// }
/// ```
pub fn parse_requirement_annotations(file_path: &Path) -> Result<Vec<TraceabilityLink>> {
    let content = fs::read_to_string(file_path)?;

    // Regex pattern to match requirement IDs
    // Matches: REQ-XXX, REQ-XXX-YYY, REQ-XXX.Y, etc.
    let re = Regex::new(r"REQ-[A-Z0-9]+(?:-[A-Z0-9]+)*(?:\.[0-9]+)?").unwrap();

    let mut links = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        // Find all requirement IDs in this line
        for cap in re.find_iter(line) {
            let requirement_id = cap.as_str().to_string();

            let link = TraceabilityLink::new(
                requirement_id,
                file_path.to_path_buf(),
                (line_num + 1) as u32, // Line numbers are 1-based
                LinkType::Implementation,
            );

            links.push(link);
        }
    }

    Ok(links)
}

/// Parse test annotations from test code.
///
/// Searches for test annotations that link test cases to requirements using patterns like:
/// - `// TEST: REQ-001`
/// - `/* TEST: REQ-SENSOR-001, REQ-SENSOR-002 */`
///
/// # Arguments
///
/// * `file_path` - Path to the test file to parse
///
/// # Returns
///
/// A vector of traceability links found in the file
///
/// # Errors
///
/// Returns `TraceabilityError` if the file cannot be read
///
/// # Examples
///
/// ```no_run
/// use axiom_compliance::parse_test_annotations;
/// use std::path::Path;
///
/// let links = parse_test_annotations(Path::new("tests/test_main.c")).unwrap();
/// for link in links {
///     println!("Test covers requirement: {}", link.requirement_id);
/// }
/// ```
pub fn parse_test_annotations(file_path: &Path) -> Result<Vec<TraceabilityLink>> {
    let content = fs::read_to_string(file_path)?;

    // Regex pattern to match TEST: followed by requirement IDs
    let test_re = Regex::new(r"TEST:\s*(.+)").unwrap();
    let req_re = Regex::new(r"REQ-[A-Z0-9]+(?:-[A-Z0-9]+)*(?:\.[0-9]+)?").unwrap();

    let mut links = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        // Check if line contains TEST: annotation
        if let Some(test_cap) = test_re.captures(line) {
            let req_list = test_cap.get(1).unwrap().as_str();

            // Find all requirement IDs in the TEST: annotation
            for req_cap in req_re.find_iter(req_list) {
                let requirement_id = req_cap.as_str().to_string();

                let link = TraceabilityLink::new(
                    requirement_id,
                    file_path.to_path_buf(),
                    (line_num + 1) as u32, // Line numbers are 1-based
                    LinkType::Test,
                );

                links.push(link);
            }
        }
    }

    Ok(links)
}

/// Generate a traceability matrix from source files in a directory.
///
/// Recursively scans the directory for source files (.c, .h, .cpp, .hpp)
/// and test files, parsing requirement and test annotations to build a
/// complete traceability matrix.
///
/// # Arguments
///
/// * `project_path` - Root directory of the project to scan
///
/// # Returns
///
/// A complete traceability matrix with all discovered links
///
/// # Errors
///
/// Returns `TraceabilityError` if any files cannot be read or parsed
///
/// # Examples
///
/// ```no_run
/// use axiom_compliance::generate_traceability_matrix;
/// use std::path::Path;
///
/// let matrix = generate_traceability_matrix(Path::new("./src")).unwrap();
/// println!("Found {} traceability links", matrix.links.len());
/// ```
pub fn generate_traceability_matrix(project_path: &Path) -> Result<TraceabilityMatrix> {
    let mut matrix = TraceabilityMatrix::new();

    // Walk the directory tree
    visit_dirs(project_path, &mut matrix)?;

    Ok(matrix)
}

/// Recursively visit directories and parse source files.
///
/// Helper function for `generate_traceability_matrix` that walks the directory
/// tree and parses all source and test files.
fn visit_dirs(dir: &Path, matrix: &mut TraceabilityMatrix) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip common non-source directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "build" || name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }
            visit_dirs(&path, matrix)?;
        } else if path.is_file() {
            // Check if it's a source or test file
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if ext_str == "c" || ext_str == "h" || ext_str == "cpp" || ext_str == "hpp" {
                    // Determine if it's a test file
                    let is_test = path.to_string_lossy().to_lowercase().contains("test");

                    if is_test {
                        // Parse test annotations
                        let links = parse_test_annotations(&path)?;
                        for link in links {
                            matrix.add_link(link);
                        }
                    } else {
                        // Parse requirement annotations
                        let links = parse_requirement_annotations(&path)?;
                        for link in links {
                            matrix.add_link(link);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Information about an untraceable function.
///
/// Represents a function that lacks requirement annotations, indicating
/// a gap in traceability that should be addressed for compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UntraceableFunction {
    /// Function name
    pub name: String,

    /// Source file containing the function
    pub file: PathBuf,

    /// Line number where the function is defined
    pub line_number: u32,
}

/// Find functions without requirement annotations.
///
/// Scans source files for function definitions and checks if they have
/// associated requirement annotations nearby (within 10 lines before the function).
///
/// # Arguments
///
/// * `project_path` - Root directory of the project to scan
///
/// # Returns
///
/// A list of functions that lack traceability annotations
///
/// # Errors
///
/// Returns `TraceabilityError` if any files cannot be read
///
/// # Examples
///
/// ```no_run
/// use axiom_compliance::find_untraceable_functions;
/// use std::path::Path;
///
/// let untraceable = find_untraceable_functions(Path::new("./src")).unwrap();
/// for func in untraceable {
///     println!("Function '{}' in {} lacks traceability", func.name, func.file.display());
/// }
/// ```
pub fn find_untraceable_functions(project_path: &Path) -> Result<Vec<UntraceableFunction>> {
    let mut untraceable = Vec::new();

    // Walk the directory tree
    find_untraceable_in_dir(project_path, &mut untraceable)?;

    Ok(untraceable)
}

/// Recursively find untraceable functions in a directory.
///
/// Helper function for `find_untraceable_functions` that walks the directory
/// tree and analyzes all source files.
fn find_untraceable_in_dir(dir: &Path, untraceable: &mut Vec<UntraceableFunction>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip common non-source directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "build" || name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }
            find_untraceable_in_dir(&path, untraceable)?;
        } else if path.is_file() {
            // Check if it's a source file (not test)
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if (ext_str == "c" || ext_str == "cpp")
                    && !path.to_string_lossy().to_lowercase().contains("test")
                {
                    analyze_file_for_untraceable(&path, untraceable)?;
                }
            }
        }
    }

    Ok(())
}

/// Analyze a single file for untraceable functions.
///
/// Parses a source file to identify function definitions and checks if they
/// have requirement annotations within the preceding 10 lines.
///
/// # Arguments
///
/// * `file_path` - Path to the source file to analyze
/// * `untraceable` - Vector to append untraceable functions to
///
/// # Errors
///
/// Returns `TraceabilityError` if the file cannot be read
pub fn analyze_file_for_untraceable(
    file_path: &Path,
    untraceable: &mut Vec<UntraceableFunction>,
) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Regex to match function definitions - looking for pattern: type name(...)
    // This pattern looks for: identifier followed by ( and )
    let func_re = Regex::new(r"\b([a-zA-Z_]\w*)\s*\([^)]*\)").unwrap();
    let req_re = Regex::new(r"REQ-[A-Z0-9]+(?:-[A-Z0-9]+)*(?:\.[0-9]+)?").unwrap();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines and preprocessor directives
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Skip lines that are just comments
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
            continue;
        }

        // Check if this line contains a function definition pattern
        // Must contain ( and ) and not be a function call (no semicolon at end typically)
        if !trimmed.contains('(') || !trimmed.contains(')') {
            continue;
        }

        // Skip lines that look like function calls (end with semicolon)
        if trimmed.ends_with(';') {
            continue;
        }

        // Check if this line contains a function definition
        if let Some(cap) = func_re.captures(trimmed) {
            let func_name = cap.get(1).unwrap().as_str().to_string();

            // Skip common keywords that aren't functions
            if func_name == "if"
                || func_name == "while"
                || func_name == "for"
                || func_name == "switch"
                || func_name == "return"
                || func_name == "sizeof"
            {
                continue;
            }

            // Look back up to 10 lines for a REQ annotation
            let start = line_num.saturating_sub(10);
            let mut has_req = false;

            for prev_line in &lines[start..=line_num] {
                if req_re.is_match(prev_line) {
                    has_req = true;
                    break;
                }
            }

            if !has_req {
                untraceable.push(UntraceableFunction {
                    name: func_name,
                    file: file_path.to_path_buf(),
                    line_number: (line_num + 1) as u32,
                });
            }
        }
    }

    Ok(())
}

/// Find requirements that lack test coverage.
///
/// Analyzes the traceability matrix to identify requirements that have
/// implementation links but no test links, indicating untested functionality.
///
/// # Arguments
///
/// * `matrix` - The traceability matrix to analyze
///
/// # Returns
///
/// A sorted list of requirement IDs without test coverage
///
/// # Examples
///
/// ```
/// use axiom_compliance::{TraceabilityMatrix, TraceabilityLink, LinkType, find_untested_requirements};
/// use std::path::PathBuf;
///
/// let mut matrix = TraceabilityMatrix::new();
///
/// // Add implementation link
/// matrix.add_link(TraceabilityLink::new(
///     "REQ-001".to_string(),
///     PathBuf::from("impl.c"),
///     10,
///     LinkType::Implementation,
/// ));
///
/// let untested = find_untested_requirements(&matrix);
/// assert_eq!(untested, vec!["REQ-001"]);
/// ```
pub fn find_untested_requirements(matrix: &TraceabilityMatrix) -> Vec<String> {
    let mut untested = Vec::new();

    // Get all unique requirements
    let all_reqs = matrix.get_all_requirements();

    for req_id in all_reqs {
        let links = matrix.get_links_for_requirement(&req_id);

        // Check if there's at least one implementation link
        let has_impl = links
            .iter()
            .any(|link| link.link_type == LinkType::Implementation);

        // Check if there's at least one test link
        let has_test = links.iter().any(|link| link.link_type == LinkType::Test);

        // If implemented but not tested, add to untested list
        if has_impl && !has_test {
            untested.push(req_id);
        }
    }

    untested.sort();
    untested
}

/// Export the traceability matrix to CSV format.
///
/// Creates a CSV file with columns for requirement ID, source file, line number,
/// link type, and creation timestamp. This format is suitable for certification
/// documentation and external analysis tools.
///
/// # Arguments
///
/// * `matrix` - The traceability matrix to export
/// * `output_path` - Path where the CSV file will be written
///
/// # Errors
///
/// Returns `TraceabilityError` if the file cannot be written
///
/// # Examples
///
/// ```no_run
/// use axiom_compliance::{TraceabilityMatrix, export_matrix_csv};
/// use std::path::Path;
///
/// let matrix = TraceabilityMatrix::new();
/// export_matrix_csv(&matrix, Path::new("traceability.csv")).unwrap();
/// ```
pub fn export_matrix_csv(matrix: &TraceabilityMatrix, output_path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(output_path)?;

    // Write header
    writer.write_record([
        "Requirement ID",
        "Source File",
        "Line Number",
        "Link Type",
        "Created At",
    ])?;

    // Write each link
    for link in &matrix.links {
        let link_type_str = match link.link_type {
            LinkType::Implementation => "Implementation",
            LinkType::Test => "Test",
            LinkType::Derived => "Derived",
        };

        writer.write_record([
            &link.requirement_id,
            &link.source_file.display().to_string(),
            &link.line_number.to_string(),
            link_type_str,
            &link.created_at.to_rfc3339(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_requirement_annotations_finds_req_001() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "// REQ-001: Initialize system").unwrap();
        writeln!(temp_file, "void init(void) {{").unwrap();
        writeln!(temp_file, "    // Implementation").unwrap();
        writeln!(temp_file, "}}").unwrap();
        temp_file.flush().unwrap();

        let links = parse_requirement_annotations(temp_file.path()).unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].requirement_id, "REQ-001");
        assert_eq!(links[0].line_number, 1);
        assert_eq!(links[0].link_type, LinkType::Implementation);
    }

    #[test]
    fn test_parse_requirement_annotations_finds_sub_requirements() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "/* REQ-001.1: Sub-requirement */").unwrap();
        writeln!(temp_file, "void sub_init(void) {{").unwrap();
        writeln!(temp_file, "    /* REQ-001.2: Another sub */").unwrap();
        writeln!(temp_file, "}}").unwrap();
        temp_file.flush().unwrap();

        let links = parse_requirement_annotations(temp_file.path()).unwrap();

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].requirement_id, "REQ-001.1");
        assert_eq!(links[0].line_number, 1);
        assert_eq!(links[1].requirement_id, "REQ-001.2");
        assert_eq!(links[1].line_number, 3);
    }

    #[test]
    fn test_parse_test_annotations_finds_single_requirement() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "// TEST: REQ-001").unwrap();
        writeln!(temp_file, "void test_init(void) {{").unwrap();
        writeln!(temp_file, "    assert(init() == 0);").unwrap();
        writeln!(temp_file, "}}").unwrap();
        temp_file.flush().unwrap();

        let links = parse_test_annotations(temp_file.path()).unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].requirement_id, "REQ-001");
        assert_eq!(links[0].line_number, 1);
        assert_eq!(links[0].link_type, LinkType::Test);
    }

    #[test]
    fn test_parse_test_annotations_finds_multiple_requirements() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "/* TEST: REQ-001, REQ-002 */").unwrap();
        writeln!(temp_file, "void test_combined(void) {{").unwrap();
        writeln!(temp_file, "    // Test both requirements").unwrap();
        writeln!(temp_file, "}}").unwrap();
        temp_file.flush().unwrap();

        let links = parse_test_annotations(temp_file.path()).unwrap();

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].requirement_id, "REQ-001");
        assert_eq!(links[1].requirement_id, "REQ-002");
        assert_eq!(links[0].line_number, 1);
        assert_eq!(links[1].line_number, 1);
    }

    #[test]
    fn test_find_untraceable_functions_identifies_functions_without_req() {
        // Test with the actual reference project file that we know has untraceable functions
        let untraced_path =
            Path::new("tests/fixtures/arm-reference-project/compliance/untraced_module.c");

        if !untraced_path.exists() {
            // Skip test if reference project doesn't exist
            return;
        }

        let mut untraceable = Vec::new();
        analyze_file_for_untraceable(untraced_path, &mut untraceable).unwrap();

        // The untraced_module.c file has several functions without REQ annotations
        // We should find at least some of them
        assert!(
            !untraceable.is_empty(),
            "Should find untraceable functions in untraced_module.c"
        );
    }

    #[test]
    fn test_matrix_contains_all_parsed_requirements() {
        let mut matrix = TraceabilityMatrix::new();

        let link1 = TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("test.c"),
            10,
            LinkType::Implementation,
        );

        let link2 = TraceabilityLink::new(
            "REQ-002".to_string(),
            PathBuf::from("test.c"),
            20,
            LinkType::Implementation,
        );

        let link3 = TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("test_test.c"),
            5,
            LinkType::Test,
        );

        matrix.add_link(link1);
        matrix.add_link(link2);
        matrix.add_link(link3);

        let all_reqs = matrix.get_all_requirements();
        assert_eq!(all_reqs.len(), 2);
        assert!(all_reqs.contains(&"REQ-001".to_string()));
        assert!(all_reqs.contains(&"REQ-002".to_string()));

        let req1_links = matrix.get_links_for_requirement("REQ-001");
        assert_eq!(req1_links.len(), 2);
    }

    #[test]
    fn test_find_untested_requirements() {
        let mut matrix = TraceabilityMatrix::new();

        // REQ-001: Has both implementation and test
        matrix.add_link(TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("impl.c"),
            10,
            LinkType::Implementation,
        ));
        matrix.add_link(TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("test.c"),
            5,
            LinkType::Test,
        ));

        // REQ-002: Has implementation but no test
        matrix.add_link(TraceabilityLink::new(
            "REQ-002".to_string(),
            PathBuf::from("impl.c"),
            20,
            LinkType::Implementation,
        ));

        let untested = find_untested_requirements(&matrix);

        assert_eq!(untested.len(), 1);
        assert_eq!(untested[0], "REQ-002");
    }

    #[test]
    fn test_export_matrix_csv() {
        let mut matrix = TraceabilityMatrix::new();

        matrix.add_link(TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("test.c"),
            10,
            LinkType::Implementation,
        ));

        matrix.add_link(TraceabilityLink::new(
            "REQ-001".to_string(),
            PathBuf::from("test_test.c"),
            5,
            LinkType::Test,
        ));

        let temp_file = NamedTempFile::new().unwrap();
        export_matrix_csv(&matrix, temp_file.path()).unwrap();

        let content = fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("Requirement ID"));
        assert!(content.contains("REQ-001"));
        assert!(content.contains("test.c"));
        assert!(content.contains("Implementation"));
        assert!(content.contains("Test"));
    }
}
