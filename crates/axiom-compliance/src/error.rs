// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Error types for compliance operations.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during compliance operations.
#[derive(Debug, Error)]
pub enum ComplianceError {
    /// Traceability annotation parsing error.
    #[error("Failed to parse traceability annotation in {file}:{line}: {message}")]
    AnnotationParseError {
        /// File where the error occurred.
        file: PathBuf,
        /// Line number.
        line: u32,
        /// Error message.
        message: String,
    },

    /// Requirement not found in traceability matrix.
    #[error("Requirement '{requirement_id}' not found in traceability matrix")]
    RequirementNotFound {
        /// Requirement identifier.
        requirement_id: String,
    },

    /// Coverage data file not found.
    #[error("Coverage data file not found: {path}")]
    CoverageDataNotFound {
        /// Path to the missing coverage data file.
        path: PathBuf,
    },

    /// Invalid coverage data format.
    #[error("Invalid coverage data format in {file}: {message}")]
    InvalidCoverageData {
        /// File with invalid data.
        file: PathBuf,
        /// Error message.
        message: String,
    },

    /// Tool qualification log error.
    #[error("Tool qualification log error: {message}")]
    ToolQualificationError {
        /// Error message.
        message: String,
    },

    /// Checksum mismatch detected.
    #[error("Checksum mismatch for {file}: expected {expected}, found {found}")]
    ChecksumMismatch {
        /// File with checksum mismatch.
        file: PathBuf,
        /// Expected checksum.
        expected: String,
        /// Found checksum.
        found: String,
    },

    /// Compliance mode not enabled.
    #[error("Compliance mode '{mode}' is not enabled")]
    ModeNotEnabled {
        /// Compliance mode name.
        mode: String,
    },

    /// Export format not supported.
    #[error("Export format '{format}' is not supported")]
    UnsupportedExportFormat {
        /// Format name.
        format: String,
    },

    /// Deviation detected during mode re-enablement.
    #[error("Deviations detected: {deviation_count} deviation(s) found")]
    DeviationsDetected {
        /// Number of deviations.
        deviation_count: usize,
        /// Deviation details.
        details: Vec<String>,
    },

    /// I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// CSV error occurred.
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    /// JSON serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Other error.
    #[error("{0}")]
    Other(String),
}

impl ComplianceError {
    /// Create an AnnotationParseError.
    pub fn annotation_parse_error(file: PathBuf, line: u32, message: String) -> Self {
        Self::AnnotationParseError {
            file,
            line,
            message,
        }
    }

    /// Create a RequirementNotFound error.
    pub fn requirement_not_found(requirement_id: String) -> Self {
        Self::RequirementNotFound { requirement_id }
    }

    /// Create a CoverageDataNotFound error.
    pub fn coverage_data_not_found(path: PathBuf) -> Self {
        Self::CoverageDataNotFound { path }
    }

    /// Create an InvalidCoverageData error.
    pub fn invalid_coverage_data(file: PathBuf, message: String) -> Self {
        Self::InvalidCoverageData { file, message }
    }

    /// Create a ToolQualificationError.
    pub fn tool_qualification_error(message: String) -> Self {
        Self::ToolQualificationError { message }
    }

    /// Create a ChecksumMismatch error.
    pub fn checksum_mismatch(file: PathBuf, expected: String, found: String) -> Self {
        Self::ChecksumMismatch {
            file,
            expected,
            found,
        }
    }

    /// Create a ModeNotEnabled error.
    pub fn mode_not_enabled(mode: String) -> Self {
        Self::ModeNotEnabled { mode }
    }

    /// Create an UnsupportedExportFormat error.
    pub fn unsupported_export_format(format: String) -> Self {
        Self::UnsupportedExportFormat { format }
    }

    /// Create a DeviationsDetected error.
    pub fn deviations_detected(deviation_count: usize, details: Vec<String>) -> Self {
        Self::DeviationsDetected {
            deviation_count,
            details,
        }
    }
}

/// Result type for compliance operations.
pub type Result<T> = std::result::Result<T, ComplianceError>;
