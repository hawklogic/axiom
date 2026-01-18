// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Toolchain types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Kind of toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolchainKind {
    /// LLVM/Clang compiler.
    Clang,
    /// GNU GCC compiler.
    Gcc,
    /// ARM GCC (arm-none-eabi-gcc).
    ArmGcc,
    /// Python interpreter.
    Python,
}

impl std::fmt::Display for ToolchainKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolchainKind::Clang => write!(f, "Clang"),
            ToolchainKind::Gcc => write!(f, "GCC"),
            ToolchainKind::ArmGcc => write!(f, "ARM GCC"),
            ToolchainKind::Python => write!(f, "Python"),
        }
    }
}

/// Detected toolchain information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedToolchain {
    /// Kind of toolchain.
    pub kind: ToolchainKind,
    /// Path to the binary.
    pub path: PathBuf,
    /// Version string.
    pub version: String,
    /// Whether this is the bundled version.
    pub bundled: bool,
}

impl DetectedToolchain {
    /// Create a new detected toolchain.
    pub fn new(kind: ToolchainKind, path: PathBuf, version: String) -> Self {
        Self {
            kind,
            path,
            version,
            bundled: false,
        }
    }

    /// Mark this toolchain as bundled.
    pub fn as_bundled(mut self) -> Self {
        self.bundled = true;
        self
    }
}

/// A request to compile source code.
#[derive(Debug, Clone)]
pub struct CompileRequest {
    /// Source file path.
    pub source: PathBuf,
    /// Output file path.
    pub output: PathBuf,
    /// Target architecture (optional).
    pub target: Option<String>,
    /// Additional compiler flags.
    pub flags: Vec<String>,
    /// Optimization level (0-3).
    pub optimization: u8,
    /// Include debug symbols.
    pub debug: bool,
}

impl CompileRequest {
    /// Create a new compile request.
    pub fn new(source: PathBuf, output: PathBuf) -> Self {
        Self {
            source,
            output,
            target: None,
            flags: Vec::new(),
            optimization: 0,
            debug: true,
        }
    }

    /// Set the target architecture.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Add a compiler flag.
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Set optimization level.
    pub fn with_optimization(mut self, level: u8) -> Self {
        self.optimization = level.min(3);
        self
    }

    /// Enable or disable debug symbols.
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// Result of a compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// Exit code of the compiler.
    pub exit_code: i32,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Parsed diagnostics.
    pub diagnostics: Vec<axiom_core::Diagnostic>,
}

impl CompileResult {
    /// Check if compilation succeeded.
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}
