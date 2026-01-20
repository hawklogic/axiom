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

/// Source of a detected toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolchainSource {
    /// Installed via Homebrew (macOS).
    Homebrew,
    /// Bundled with STM32CubeIDE.
    Stm32CubeIde,
    /// Found in system PATH.
    SystemPath,
    /// Manually specified by user.
    Manual,
}

/// Completeness status of a toolchain suite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolchainCompleteness {
    /// All required tools are present.
    Complete,
    /// Some tools are missing.
    Incomplete {
        /// List of missing tool names.
        missing: Vec<String>,
    },
}

/// Complete ARM toolchain suite with all required tools.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmToolchainSuite {
    /// Path to arm-none-eabi-gcc.
    pub gcc: PathBuf,
    /// Path to arm-none-eabi-g++.
    pub gxx: PathBuf,
    /// Path to arm-none-eabi-as (assembler).
    pub as_: PathBuf,
    /// Path to arm-none-eabi-ld (linker).
    pub ld: PathBuf,
    /// Path to arm-none-eabi-objcopy.
    pub objcopy: PathBuf,
    /// Path to arm-none-eabi-objdump.
    pub objdump: PathBuf,
    /// Path to arm-none-eabi-size.
    pub size: PathBuf,
    /// Path to arm-none-eabi-gdb.
    pub gdb: PathBuf,
    /// Version string from gcc.
    pub version: String,
    /// Source of this toolchain.
    pub source: ToolchainSource,
    /// Completeness status.
    pub completeness: ToolchainCompleteness,
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

/// A request to compile ARM source code.
#[derive(Debug, Clone)]
pub struct ArmCompileRequest {
    /// Source file path.
    pub source: PathBuf,
    /// Output file path.
    pub output: PathBuf,
    /// MCU configuration.
    pub mcu: crate::ArmMcuConfig,
    /// Include paths.
    pub include_paths: Vec<PathBuf>,
    /// Optimization level (0-3).
    pub optimization: u8,
    /// Include debug symbols.
    pub debug: bool,
}

impl ArmCompileRequest {
    /// Create a new ARM compile request.
    pub fn new(source: PathBuf, output: PathBuf, mcu: crate::ArmMcuConfig) -> Self {
        Self {
            source,
            output,
            mcu,
            include_paths: Vec::new(),
            optimization: 0,
            debug: true,
        }
    }
    
    /// Add an include path.
    pub fn with_include_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.include_paths.push(path.into());
        self
    }
    
    /// Add a preprocessor define.
    pub fn with_define(mut self, define: impl Into<String>) -> Self {
        self.mcu = self.mcu.with_define(define);
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

/// A request to link ARM object files.
#[derive(Debug, Clone)]
pub struct ArmLinkRequest {
    /// Object files to link.
    pub objects: Vec<PathBuf>,
    /// Output ELF file path.
    pub output: PathBuf,
    /// Linker configuration.
    pub linker: crate::LinkerConfig,
    /// MCU configuration.
    pub mcu: crate::ArmMcuConfig,
}

impl ArmLinkRequest {
    /// Create a new ARM link request.
    pub fn new(
        objects: Vec<PathBuf>,
        output: PathBuf,
        linker: crate::LinkerConfig,
        mcu: crate::ArmMcuConfig,
    ) -> Self {
        Self {
            objects,
            output,
            linker,
            mcu,
        }
    }
}

/// Result of a link operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkResult {
    /// Exit code of the linker.
    pub exit_code: i32,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
    /// Parsed diagnostics.
    pub diagnostics: Vec<axiom_core::Diagnostic>,
}

impl LinkResult {
    /// Check if linking succeeded.
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
    
    /// Check if there was a memory overflow error.
    pub fn has_memory_overflow(&self) -> bool {
        self.stderr.contains("will not fit") || 
        self.stderr.contains("region") && self.stderr.contains("overflow")
    }
}
