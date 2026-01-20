// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! # Tool Qualification Logger
//!
//! Provides DO-330 tool qualification support by logging all tool invocations
//! with complete traceability of inputs, outputs, and execution context.
//!
//! ## Safety-Critical Context
//!
//! This module supports DO-330 tool qualification activities for safety-critical
//! avionics software development. Tool qualification is essential for demonstrating
//! that development and verification tools do not introduce errors into the software.
//!
//! **CRITICAL**: All tool usage must be logged with complete fidelity. No AI-generated
//! content or approximations are permitted in qualification data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during tool qualification logging
#[derive(Debug, Error)]
pub enum ToolQualificationError {
    #[error("Failed to compute checksum for file {path}: {source}")]
    ChecksumError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to write to log file: {0}")]
    LogWriteError(#[from] std::io::Error),

    #[error("Failed to serialize record: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Log file not found: {0}")]
    LogFileNotFound(PathBuf),
}

/// Result type for tool qualification operations
pub type Result<T> = std::result::Result<T, ToolQualificationError>;

/// Record of a single tool invocation for DO-330 qualification
///
/// This structure captures all information necessary to demonstrate that a tool
/// was used correctly and produced deterministic, verifiable outputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolUsageRecord {
    /// Name of the tool (e.g., "arm-none-eabi-gcc")
    pub tool: String,

    /// Tool version string
    pub version: String,

    /// Complete command-line arguments
    pub arguments: Vec<String>,

    /// SHA-256 checksums of all input files
    pub input_checksums: HashMap<PathBuf, String>,

    /// SHA-256 checksums of all output files
    pub output_checksums: HashMap<PathBuf, String>,

    /// Timestamp when the tool was invoked
    pub timestamp: DateTime<Utc>,

    /// Exit code returned by the tool
    pub exit_code: i32,

    /// Any warnings, errors, or diagnostic messages
    pub diagnostics: Vec<String>,
}

impl ToolUsageRecord {
    /// Create a new tool usage record
    pub fn new(
        tool: String,
        version: String,
        arguments: Vec<String>,
        exit_code: i32,
    ) -> Self {
        Self {
            tool,
            version,
            arguments,
            input_checksums: HashMap::new(),
            output_checksums: HashMap::new(),
            timestamp: Utc::now(),
            exit_code,
            diagnostics: Vec::new(),
        }
    }

    /// Add an input file checksum
    pub fn add_input_checksum(&mut self, path: PathBuf, checksum: String) {
        self.input_checksums.insert(path, checksum);
    }

    /// Add an output file checksum
    pub fn add_output_checksum(&mut self, path: PathBuf, checksum: String) {
        self.output_checksums.insert(path, checksum);
    }

    /// Add a diagnostic message
    pub fn add_diagnostic(&mut self, message: String) {
        self.diagnostics.push(message);
    }
}

/// Tool qualification logger for DO-330 compliance
///
/// Maintains an append-only log of all tool invocations with complete
/// traceability information for certification purposes.
pub struct ToolQualificationLogger {
    /// Path to the log file
    log_path: PathBuf,
}

impl ToolQualificationLogger {
    /// Create a new tool qualification logger
    ///
    /// # Arguments
    ///
    /// * `log_path` - Path where the log file will be stored
    ///
    /// # Example
    ///
    /// ```
    /// use axiom_compliance::tool_qualification::ToolQualificationLogger;
    /// use std::path::PathBuf;
    ///
    /// let logger = ToolQualificationLogger::new(PathBuf::from("tool_usage.log"));
    /// ```
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    /// Log a tool usage record
    ///
    /// Appends the record to the log file in JSON format, one record per line.
    /// The log file is append-only to maintain an immutable audit trail.
    ///
    /// # Arguments
    ///
    /// * `record` - The tool usage record to log
    ///
    /// # Errors
    ///
    /// Returns an error if the log file cannot be opened or written to,
    /// or if the record cannot be serialized.
    pub fn log(&self, record: &ToolUsageRecord) -> Result<()> {
        // Open file in append mode, create if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        // Serialize record to JSON
        let json = serde_json::to_string(record)?;

        // Write record as a single line
        writeln!(file, "{}", json)?;

        Ok(())
    }

    /// Retrieve all logged records
    ///
    /// Reads the entire log file and deserializes all records.
    ///
    /// # Errors
    ///
    /// Returns an error if the log file cannot be read or if any record
    /// cannot be deserialized.
    pub fn get_all_records(&self) -> Result<Vec<ToolUsageRecord>> {
        // Check if log file exists
        if !self.log_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let record: ToolUsageRecord = serde_json::from_str(&line)?;
                records.push(record);
            }
        }

        Ok(records)
    }

    /// Get the path to the log file
    pub fn log_path(&self) -> &Path {
        &self.log_path
    }
}

/// Compute SHA-256 checksum of a file
///
/// # Arguments
///
/// * `path` - Path to the file to checksum
///
/// # Returns
///
/// A 64-character hexadecimal string representing the SHA-256 hash
///
/// # Errors
///
/// Returns an error if the file cannot be read
///
/// # Example
///
/// ```no_run
/// use axiom_compliance::tool_qualification::compute_sha256;
/// use std::path::Path;
///
/// let checksum = compute_sha256(Path::new("input.c")).unwrap();
/// assert_eq!(checksum.len(), 64);
/// ```
pub fn compute_sha256(path: &Path) -> Result<String> {
    let contents = std::fs::read(path).map_err(|e| ToolQualificationError::ChecksumError {
        path: path.to_path_buf(),
        source: e,
    })?;

    let mut hasher = Sha256::new();
    hasher.update(&contents);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_tool_usage_record_creation() {
        let record = ToolUsageRecord::new(
            "arm-none-eabi-gcc".to_string(),
            "14.3.1".to_string(),
            vec!["-c".to_string(), "main.c".to_string()],
            0,
        );

        assert_eq!(record.tool, "arm-none-eabi-gcc");
        assert_eq!(record.version, "14.3.1");
        assert_eq!(record.arguments.len(), 2);
        assert_eq!(record.exit_code, 0);
        assert!(record.input_checksums.is_empty());
        assert!(record.output_checksums.is_empty());
        assert!(record.diagnostics.is_empty());
    }

    #[test]
    fn test_tool_usage_record_add_checksums() {
        let mut record = ToolUsageRecord::new(
            "gcc".to_string(),
            "1.0".to_string(),
            vec![],
            0,
        );

        record.add_input_checksum(PathBuf::from("input.c"), "abc123".to_string());
        record.add_output_checksum(PathBuf::from("output.o"), "def456".to_string());

        assert_eq!(record.input_checksums.len(), 1);
        assert_eq!(record.output_checksums.len(), 1);
        assert_eq!(
            record.input_checksums.get(&PathBuf::from("input.c")),
            Some(&"abc123".to_string())
        );
    }

    #[test]
    fn test_tool_usage_record_add_diagnostic() {
        let mut record = ToolUsageRecord::new(
            "gcc".to_string(),
            "1.0".to_string(),
            vec![],
            0,
        );

        record.add_diagnostic("warning: unused variable".to_string());
        record.add_diagnostic("error: undefined reference".to_string());

        assert_eq!(record.diagnostics.len(), 2);
        assert_eq!(record.diagnostics[0], "warning: unused variable");
    }

    // 13.2.1: Test compute_sha256() returns 64-char hex string
    #[test]
    fn test_compute_sha256_returns_64_char_hex() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, b"Hello, World!").unwrap();

        let checksum = compute_sha256(&file_path).unwrap();

        // SHA-256 produces 32 bytes = 64 hex characters
        assert_eq!(checksum.len(), 64);
        // Verify it's all hex characters
        assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // 13.2.2: Test compute_sha256() same content produces same hash
    #[test]
    fn test_compute_sha256_deterministic() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        let content = b"This is test content for checksumming";
        fs::write(&file1, content).unwrap();
        fs::write(&file2, content).unwrap();

        let checksum1 = compute_sha256(&file1).unwrap();
        let checksum2 = compute_sha256(&file2).unwrap();

        assert_eq!(checksum1, checksum2);
    }

    // 13.2.3: Test ToolUsageRecord serialization roundtrip
    #[test]
    fn test_tool_usage_record_serialization_roundtrip() {
        let mut record = ToolUsageRecord::new(
            "arm-none-eabi-gcc".to_string(),
            "14.3.1".to_string(),
            vec!["-c".to_string(), "-O2".to_string(), "main.c".to_string()],
            0,
        );

        record.add_input_checksum(
            PathBuf::from("main.c"),
            "abc123def456".to_string(),
        );
        record.add_output_checksum(
            PathBuf::from("main.o"),
            "789ghi012jkl".to_string(),
        );
        record.add_diagnostic("warning: unused variable 'x'".to_string());

        // Serialize to JSON
        let json = serde_json::to_string(&record).unwrap();

        // Deserialize back
        let deserialized: ToolUsageRecord = serde_json::from_str(&json).unwrap();

        // Verify all fields match
        assert_eq!(record.tool, deserialized.tool);
        assert_eq!(record.version, deserialized.version);
        assert_eq!(record.arguments, deserialized.arguments);
        assert_eq!(record.input_checksums, deserialized.input_checksums);
        assert_eq!(record.output_checksums, deserialized.output_checksums);
        assert_eq!(record.exit_code, deserialized.exit_code);
        assert_eq!(record.diagnostics, deserialized.diagnostics);
    }

    // 13.2.4: Test log() appends to file without overwriting
    #[test]
    fn test_log_appends_without_overwriting() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("tool_usage.log");
        let logger = ToolQualificationLogger::new(log_path.clone());

        // Log first record
        let record1 = ToolUsageRecord::new(
            "gcc".to_string(),
            "1.0".to_string(),
            vec!["-c".to_string(), "file1.c".to_string()],
            0,
        );
        logger.log(&record1).unwrap();

        // Log second record
        let record2 = ToolUsageRecord::new(
            "gcc".to_string(),
            "1.0".to_string(),
            vec!["-c".to_string(), "file2.c".to_string()],
            0,
        );
        logger.log(&record2).unwrap();

        // Read log file and verify both records are present
        let records = logger.get_all_records().unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].arguments[1], "file1.c");
        assert_eq!(records[1].arguments[1], "file2.c");
    }

    // 13.2.5: Test get_all_records() returns all logged records
    #[test]
    fn test_get_all_records_returns_all() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("tool_usage.log");
        let logger = ToolQualificationLogger::new(log_path.clone());

        // Log multiple records
        for i in 0..5 {
            let record = ToolUsageRecord::new(
                "tool".to_string(),
                "1.0".to_string(),
                vec![format!("arg{}", i)],
                i,
            );
            logger.log(&record).unwrap();
        }

        // Retrieve all records
        let records = logger.get_all_records().unwrap();

        assert_eq!(records.len(), 5);
        for (i, record) in records.iter().enumerate() {
            assert_eq!(record.exit_code, i as i32);
            assert_eq!(record.arguments[0], format!("arg{}", i));
        }
    }

    #[test]
    fn test_get_all_records_empty_log() {
        let temp_dir = TempDir::new().unwrap();
        let log_path = temp_dir.path().join("nonexistent.log");
        let logger = ToolQualificationLogger::new(log_path);

        let records = logger.get_all_records().unwrap();
        assert_eq!(records.len(), 0);
    }

    #[test]
    fn test_compute_sha256_nonexistent_file() {
        let result = compute_sha256(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }
}
