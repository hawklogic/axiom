// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Common types used across Axiom crates.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a position in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-indexed).
    pub line: u32,
    /// Column number (0-indexed).
    pub column: u32,
}

impl Position {
    /// Create a new position.
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Represents a range in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    /// Start position.
    pub start: Position,
    /// End position.
    pub end: Position,
}

impl Range {
    /// Create a new range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Represents a location in a source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// Path to the file.
    pub path: PathBuf,
    /// Range within the file.
    pub range: Range,
}

impl Location {
    /// Create a new location.
    pub fn new(path: PathBuf, range: Range) -> Self {
        Self { path, range }
    }
}

/// Severity level for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Error - compilation cannot proceed.
    Error,
    /// Warning - potential issue.
    Warning,
    /// Note - informational.
    Note,
}

/// A diagnostic message from a tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity level.
    pub severity: Severity,
    /// Diagnostic message.
    pub message: String,
    /// Location in source (if available).
    pub location: Option<Location>,
}

impl Diagnostic {
    /// Create an error diagnostic.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            location: None,
        }
    }

    /// Create a warning diagnostic.
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            location: None,
        }
    }

    /// Attach a location to this diagnostic.
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(10, 5);
        assert_eq!(pos.line, 10);
        assert_eq!(pos.column, 5);
    }

    #[test]
    fn test_range() {
        let start = Position::new(0, 0);
        let end = Position::new(10, 20);
        let range = Range::new(start, end);
        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
    }

    #[test]
    fn test_diagnostic() {
        let diag = Diagnostic::error("test error");
        assert_eq!(diag.severity, Severity::Error);
        assert_eq!(diag.message, "test error");
        assert!(diag.location.is_none());
    }
}
