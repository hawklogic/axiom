// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Error types for ARM toolchain operations.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during ARM toolchain operations.
#[derive(Debug, Error)]
pub enum ArmToolchainError {
    /// ARM toolchain not found on the system.
    #[error("ARM toolchain not found. {suggestion}")]
    NotFound {
        /// Platform-specific installation suggestion.
        suggestion: String,
    },

    /// Toolchain is incomplete (missing required tools).
    #[error("ARM toolchain is incomplete. Missing tools: {}", missing.join(", "))]
    Incomplete {
        /// List of missing tool names.
        missing: Vec<String>,
    },

    /// Toolchain version is too old.
    #[error("ARM toolchain version {found} is too old. Minimum required version: {required}")]
    VersionTooOld {
        /// Version that was found.
        found: String,
        /// Minimum required version.
        required: String,
    },

    /// Linker script file not found.
    #[error("Linker script not found: {path}")]
    LinkerScriptNotFound {
        /// Path to the missing linker script.
        path: PathBuf,
    },

    /// Memory overflow during linking.
    #[error("Memory overflow in region '{region}': {details}")]
    MemoryOverflow {
        /// Memory region that overflowed.
        region: String,
        /// Additional details about the overflow.
        details: String,
    },

    /// Compilation failed with errors.
    #[error("Compilation failed with {error_count} error(s)")]
    CompilationFailed {
        /// Number of errors.
        error_count: usize,
        /// Diagnostic messages.
        diagnostics: Vec<String>,
    },

    /// I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Other error.
    #[error("{0}")]
    Other(String),
}

impl ArmToolchainError {
    /// Create a NotFound error with platform-specific installation suggestions.
    pub fn not_found() -> Self {
        Self::NotFound {
            suggestion: get_installation_suggestion(),
        }
    }

    /// Create an Incomplete error with a list of missing tools.
    pub fn incomplete(missing: Vec<String>) -> Self {
        Self::Incomplete { missing }
    }

    /// Create a VersionTooOld error.
    pub fn version_too_old(found: String, required: String) -> Self {
        Self::VersionTooOld { found, required }
    }

    /// Create a LinkerScriptNotFound error.
    pub fn linker_script_not_found(path: PathBuf) -> Self {
        Self::LinkerScriptNotFound { path }
    }

    /// Create a MemoryOverflow error.
    pub fn memory_overflow(region: String, details: String) -> Self {
        Self::MemoryOverflow { region, details }
    }

    /// Create a CompilationFailed error.
    pub fn compilation_failed(error_count: usize, diagnostics: Vec<String>) -> Self {
        Self::CompilationFailed {
            error_count,
            diagnostics,
        }
    }
}

/// Get platform-specific installation suggestion for ARM toolchain.
fn get_installation_suggestion() -> String {
    #[cfg(target_os = "macos")]
    {
        "Install via Homebrew: brew install --cask gcc-arm-embedded\n\
         Or download from: https://developer.arm.com/downloads/-/gnu-rm"
            .to_string()
    }

    #[cfg(target_os = "linux")]
    {
        "Install via package manager:\n\
         - Ubuntu/Debian: sudo apt-get install gcc-arm-none-eabi\n\
         - Fedora/RHEL: sudo dnf install arm-none-eabi-gcc-cs\n\
         Or download from: https://developer.arm.com/downloads/-/gnu-rm"
            .to_string()
    }

    #[cfg(target_os = "windows")]
    {
        "Download and install from ARM Developer:\n\
         https://developer.arm.com/downloads/-/gnu-rm\n\
         Or install via Chocolatey: choco install gcc-arm-embedded"
            .to_string()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        "Download from: https://developer.arm.com/downloads/-/gnu-rm".to_string()
    }
}

/// Result type for ARM toolchain operations.
pub type Result<T> = std::result::Result<T, ArmToolchainError>;
