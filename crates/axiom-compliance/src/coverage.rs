// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! # Coverage Analysis
//!
//! Structural coverage analysis for DO-178C compliance.
//!
//! This module provides functionality for measuring statement, branch, and decision coverage
//! using GCC coverage instrumentation (gcov).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during coverage analysis operations.
///
/// These errors represent various failure modes when analyzing code coverage,
/// including parsing errors, I/O failures, and invalid data.
#[derive(Debug, Error)]
pub enum CoverageError {
    #[error("Failed to parse gcov output: {0}")]
    ParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid coverage data: {0}")]
    InvalidData(String),
}

/// Coverage data for a single source file.
///
/// Contains statement and branch coverage percentages along with
/// a list of uncovered line numbers for detailed analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    /// Source file path
    pub file: PathBuf,

    /// Statement coverage percentage (0.0 - 100.0)
    pub statement_coverage: f64,

    /// Branch coverage percentage (0.0 - 100.0)
    pub branch_coverage: f64,

    /// List of uncovered line numbers
    pub uncovered_lines: Vec<u32>,
}

/// Complete coverage report for a project.
///
/// Aggregates coverage data from all source files and provides
/// overall coverage statistics for the entire project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageReport {
    /// Coverage data for each file
    pub files: Vec<FileCoverage>,

    /// Total statement coverage percentage across all files
    pub total_statement: f64,

    /// Total branch coverage percentage across all files
    pub total_branch: f64,
}

/// Line execution data from gcov.
///
/// Represents a single line in a source file with its execution count.
/// Non-executable lines (comments, blank lines) have `None` for execution_count.
#[derive(Debug, Clone)]
pub struct LineData {
    /// Line number in source file
    pub line_number: u32,
    /// Execution count (None means non-executable like comments)
    pub execution_count: Option<u32>,
}

/// Branch execution data from gcov.
///
/// Represents a branch point in the code and whether it was taken during execution.
#[derive(Debug, Clone)]
pub struct BranchData {
    /// Line number where branch occurs
    pub line_number: u32,
    /// Whether the branch was taken
    pub taken: bool,
}

/// Build compiler flags for coverage instrumentation.
///
/// Returns the flags needed for GCC coverage instrumentation:
/// - `--coverage`: Enable coverage data generation
/// - `-fprofile-arcs`: Generate arc profiling information
/// - `-ftest-coverage`: Generate coverage notes
///
/// # Returns
///
/// A vector of flag strings to pass to the compiler
///
/// # Example
///
/// ```
/// use axiom_compliance::build_coverage_flags;
///
/// let flags = build_coverage_flags();
/// assert!(flags.contains(&"--coverage".to_string()));
/// ```
pub fn build_coverage_flags() -> Vec<String> {
    vec![
        "--coverage".to_string(),
        "-fprofile-arcs".to_string(),
        "-ftest-coverage".to_string(),
    ]
}

/// Parse gcov output to extract line execution counts.
///
/// Parses the gcov output format to extract execution data for each line.
///
/// # Gcov Output Format
///
/// ```text
///         -:    0:Source:file.c
///         5:    1:int main() {
///         5:    2:    int x = 0;
///     #####:    3:    if (false) {
///     #####:    4:        x = 1;
///         -:    5:    }
///         5:    6:    return x;
///         -:    7:}
/// ```
///
/// Lines starting with:
/// - `-`: Non-executable (comments, blank lines)
/// - `#####`: Unexecuted code
/// - Number: Execution count
///
/// # Arguments
///
/// * `gcov_output` - The raw gcov output text
///
/// # Returns
///
/// A tuple containing:
/// - Vector of line execution data
/// - Vector of branch execution data
///
/// # Errors
///
/// Returns `CoverageError` if the gcov output cannot be parsed
pub fn parse_gcov_output(
    gcov_output: &str,
) -> Result<(Vec<LineData>, Vec<BranchData>), CoverageError> {
    let mut lines = Vec::new();
    let mut branches = Vec::new();

    for line in gcov_output.lines() {
        // Skip header lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse line format: "execution_count:line_number:source_code"
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() < 3 {
            continue;
        }

        let execution_str = parts[0].trim();
        let line_num_str = parts[1].trim();

        // Parse line number
        let line_number: u32 = match line_num_str.parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip non-numeric line numbers (headers)
        };

        // Parse execution count
        let execution_count = if execution_str == "-" {
            // Non-executable line
            None
        } else if execution_str == "#####" {
            // Unexecuted line
            Some(0)
        } else {
            // Executed line with count
            execution_str.parse::<u32>().ok()
        };

        lines.push(LineData {
            line_number,
            execution_count,
        });

        // Check for branch information (appears as "branch  0 taken 0" in some gcov formats)
        // For now, we'll extract basic branch info from the line content
        if parts[2].contains("branch") {
            let taken = !parts[2].contains("never executed");
            branches.push(BranchData { line_number, taken });
        }
    }

    Ok((lines, branches))
}

/// Calculate statement coverage percentage.
///
/// Computes the percentage of executable lines that were executed at least once.
///
/// # Formula
///
/// Statement coverage = (executed lines / executable lines) × 100
///
/// # Arguments
///
/// * `lines` - Vector of line execution data
///
/// # Returns
///
/// Coverage percentage as a floating-point value between 0.0 and 100.0
///
/// # Example
///
/// ```
/// use axiom_compliance::coverage::{LineData, calculate_statement_coverage};
///
/// let lines = vec![
///     LineData { line_number: 1, execution_count: Some(5) },
///     LineData { line_number: 2, execution_count: Some(0) },
/// ];
///
/// let coverage = calculate_statement_coverage(&lines);
/// assert_eq!(coverage, 50.0);
/// ```
pub fn calculate_statement_coverage(lines: &[LineData]) -> f64 {
    let executable_lines: Vec<_> = lines
        .iter()
        .filter(|l| l.execution_count.is_some())
        .collect();

    if executable_lines.is_empty() {
        return 0.0;
    }

    let executed_lines = executable_lines
        .iter()
        .filter(|l| l.execution_count.unwrap_or(0) > 0)
        .count();

    (executed_lines as f64 / executable_lines.len() as f64) * 100.0
}

/// Calculate branch coverage percentage.
///
/// Computes the percentage of branches that were taken during execution.
///
/// # Formula
///
/// Branch coverage = (taken branches / total branches) × 100
///
/// # Arguments
///
/// * `branches` - Vector of branch execution data
///
/// # Returns
///
/// Coverage percentage as a floating-point value between 0.0 and 100.0
///
/// # Example
///
/// ```
/// use axiom_compliance::coverage::{BranchData, calculate_branch_coverage};
///
/// let branches = vec![
///     BranchData { line_number: 1, taken: true },
///     BranchData { line_number: 2, taken: false },
/// ];
///
/// let coverage = calculate_branch_coverage(&branches);
/// assert_eq!(coverage, 50.0);
/// ```
pub fn calculate_branch_coverage(branches: &[BranchData]) -> f64 {
    if branches.is_empty() {
        return 0.0;
    }

    let taken_branches = branches.iter().filter(|b| b.taken).count();

    (taken_branches as f64 / branches.len() as f64) * 100.0
}

/// Generate coverage report from gcov data.
///
/// Takes a map of file paths to their gcov output and generates a complete
/// coverage report with per-file and aggregate statistics.
///
/// # Arguments
///
/// * `gcov_data` - Map from file paths to their gcov output text
///
/// # Returns
///
/// A complete coverage report with per-file and total coverage statistics
///
/// # Errors
///
/// Returns `CoverageError` if any gcov output cannot be parsed
///
/// # Example
///
/// ```no_run
/// use axiom_compliance::generate_coverage_report;
/// use std::collections::HashMap;
/// use std::path::PathBuf;
///
/// let mut gcov_data = HashMap::new();
/// gcov_data.insert(PathBuf::from("main.c"), "gcov output...".to_string());
///
/// let report = generate_coverage_report(gcov_data).unwrap();
/// println!("Total coverage: {:.2}%", report.total_statement);
/// ```
pub fn generate_coverage_report(
    gcov_data: HashMap<PathBuf, String>,
) -> Result<CoverageReport, CoverageError> {
    let mut files = Vec::new();
    let mut total_executable = 0;
    let mut total_executed = 0;
    let mut total_branches = 0;
    let mut total_taken = 0;

    for (file_path, gcov_output) in gcov_data {
        let (lines, branches) = parse_gcov_output(&gcov_output)?;

        // Calculate file-specific coverage
        let statement_coverage = calculate_statement_coverage(&lines);
        let branch_coverage = calculate_branch_coverage(&branches);

        // Collect uncovered lines
        let uncovered_lines: Vec<u32> = lines
            .iter()
            .filter(|l| l.execution_count == Some(0))
            .map(|l| l.line_number)
            .collect();

        // Update totals for overall coverage
        let executable_lines: Vec<_> = lines
            .iter()
            .filter(|l| l.execution_count.is_some())
            .collect();

        total_executable += executable_lines.len();
        total_executed += executable_lines
            .iter()
            .filter(|l| l.execution_count.unwrap_or(0) > 0)
            .count();

        total_branches += branches.len();
        total_taken += branches.iter().filter(|b| b.taken).count();

        files.push(FileCoverage {
            file: file_path,
            statement_coverage,
            branch_coverage,
            uncovered_lines,
        });
    }

    // Calculate total coverage percentages
    let total_statement = if total_executable > 0 {
        (total_executed as f64 / total_executable as f64) * 100.0
    } else {
        0.0
    };

    let total_branch = if total_branches > 0 {
        (total_taken as f64 / total_branches as f64) * 100.0
    } else {
        0.0
    };

    Ok(CoverageReport {
        files,
        total_statement,
        total_branch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_coverage_flags_contains_coverage() {
        let flags = build_coverage_flags();
        assert!(flags.contains(&"--coverage".to_string()));
        assert!(flags.contains(&"-fprofile-arcs".to_string()));
        assert!(flags.contains(&"-ftest-coverage".to_string()));
    }

    #[test]
    fn test_parse_gcov_output_identifies_executed_lines() {
        let gcov_output = r#"
        -:    0:Source:test.c
        5:    1:int main() {
        5:    2:    return 0;
        -:    3:}
"#;

        let (lines, _) = parse_gcov_output(gcov_output).unwrap();

        // Should have 2 executable lines (lines 1 and 2)
        let executed: Vec<_> = lines
            .iter()
            .filter(|l| l.execution_count.is_some() && l.execution_count.unwrap() > 0)
            .collect();

        assert_eq!(executed.len(), 2);
    }

    #[test]
    fn test_parse_gcov_output_identifies_unexecuted_lines() {
        let gcov_output = r#"
        -:    0:Source:test.c
        5:    1:int main() {
    #####:    2:    unreachable();
        5:    3:    return 0;
        -:    4:}
"#;

        let (lines, _) = parse_gcov_output(gcov_output).unwrap();

        // Should have 1 unexecuted line (line 2)
        let unexecuted: Vec<_> = lines
            .iter()
            .filter(|l| l.execution_count == Some(0))
            .collect();

        assert_eq!(unexecuted.len(), 1);
        assert_eq!(unexecuted[0].line_number, 2);
    }

    #[test]
    fn test_calculate_statement_coverage_returns_50_percent_for_5_of_10() {
        let lines = vec![
            LineData {
                line_number: 1,
                execution_count: Some(5),
            },
            LineData {
                line_number: 2,
                execution_count: Some(3),
            },
            LineData {
                line_number: 3,
                execution_count: Some(1),
            },
            LineData {
                line_number: 4,
                execution_count: Some(2),
            },
            LineData {
                line_number: 5,
                execution_count: Some(1),
            },
            LineData {
                line_number: 6,
                execution_count: Some(0),
            },
            LineData {
                line_number: 7,
                execution_count: Some(0),
            },
            LineData {
                line_number: 8,
                execution_count: Some(0),
            },
            LineData {
                line_number: 9,
                execution_count: Some(0),
            },
            LineData {
                line_number: 10,
                execution_count: Some(0),
            },
        ];

        let coverage = calculate_statement_coverage(&lines);
        assert_eq!(coverage, 50.0);
    }

    #[test]
    fn test_calculate_branch_coverage_returns_75_percent_for_3_of_4() {
        let branches = vec![
            BranchData {
                line_number: 1,
                taken: true,
            },
            BranchData {
                line_number: 2,
                taken: true,
            },
            BranchData {
                line_number: 3,
                taken: true,
            },
            BranchData {
                line_number: 4,
                taken: false,
            },
        ];

        let coverage = calculate_branch_coverage(&branches);
        assert_eq!(coverage, 75.0);
    }
}
