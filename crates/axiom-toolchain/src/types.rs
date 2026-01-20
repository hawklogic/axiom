// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Type definitions for toolchain detection and configuration.
//!
//! This module provides the core data structures used throughout the toolchain system,
//! including toolchain kinds, detection results, compilation requests, and ARM-specific
//! configurations.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Kind of toolchain.
///
/// Represents the different types of toolchains that can be detected and used
/// by the Axiom IDE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolchainKind {
    /// LLVM/Clang compiler for C/C++ development.
    Clang,
    /// GNU GCC compiler for native development.
    Gcc,
    /// ARM GCC (arm-none-eabi-gcc) for embedded ARM development.
    ArmGcc,
    /// Python interpreter for Python development.
    Python,
}

/// Source of a detected toolchain.
///
/// Indicates where a toolchain was found, which helps with prioritization
/// and troubleshooting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolchainSource {
    /// Installed via Homebrew package manager (macOS).
    Homebrew,
    /// Bundled with STM32CubeIDE.
    Stm32CubeIde,
    /// Found in system PATH directories.
    SystemPath,
    /// Manually specified by user in settings.
    Manual,
}

/// Completeness status of a toolchain suite.
///
/// For ARM toolchains, a complete suite includes all required tools
/// (gcc, g++, as, ld, objcopy, objdump, size, gdb).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolchainCompleteness {
    /// All required tools are present and functional.
    Complete,
    /// Some tools are missing from the suite.
    Incomplete {
        /// List of missing tool names (e.g., "gdb", "objcopy").
        missing: Vec<String>,
    },
}

/// Complete ARM toolchain suite with all required tools.
///
/// Represents a full ARM embedded development toolchain with paths to all
/// necessary binaries. This structure is returned by toolchain detection
/// and used throughout the compilation pipeline.
///
/// # Example
///
/// ```no_run
/// use axiom_toolchain::{detect_arm_toolchains, ToolchainCompleteness};
///
/// let suites = detect_arm_toolchains();
/// for suite in suites {
///     match suite.completeness {
///         ToolchainCompleteness::Complete => {
///             println!("Found complete toolchain: {}", suite.version);
///         }
///         ToolchainCompleteness::Incomplete { ref missing } => {
///             println!("Incomplete toolchain, missing: {:?}", missing);
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmToolchainSuite {
    /// Path to arm-none-eabi-gcc (C compiler).
    pub gcc: PathBuf,
    /// Path to arm-none-eabi-g++ (C++ compiler).
    pub gxx: PathBuf,
    /// Path to arm-none-eabi-as (assembler).
    pub as_: PathBuf,
    /// Path to arm-none-eabi-ld (linker).
    pub ld: PathBuf,
    /// Path to arm-none-eabi-objcopy (binary format converter).
    pub objcopy: PathBuf,
    /// Path to arm-none-eabi-objdump (object file analyzer).
    pub objdump: PathBuf,
    /// Path to arm-none-eabi-size (size reporting tool).
    pub size: PathBuf,
    /// Path to arm-none-eabi-gdb (debugger).
    pub gdb: PathBuf,
    /// Version string extracted from gcc (e.g., "10.3.1").
    pub version: String,
    /// Source where this toolchain was detected.
    pub source: ToolchainSource,
    /// Completeness status indicating if all tools are present.
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
///
/// Contains metadata about a toolchain that was found on the system,
/// including its type, location, version, and whether it's bundled with the IDE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectedToolchain {
    /// Kind of toolchain (Clang, GCC, ARM GCC, Python).
    pub kind: ToolchainKind,
    /// Path to the toolchain binary.
    pub path: PathBuf,
    /// Version string extracted from the toolchain.
    pub version: String,
    /// Whether this is a bundled version shipped with the IDE.
    pub bundled: bool,
}

impl DetectedToolchain {
    /// Create a new detected toolchain.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of toolchain
    /// * `path` - Path to the toolchain binary
    /// * `version` - Version string
    pub fn new(kind: ToolchainKind, path: PathBuf, version: String) -> Self {
        Self {
            kind,
            path,
            version,
            bundled: false,
        }
    }

    /// Mark this toolchain as bundled with the IDE.
    ///
    /// Returns a new instance with the bundled flag set to true.
    pub fn as_bundled(mut self) -> Self {
        self.bundled = true;
        self
    }
}

/// A request to compile source code.
///
/// Represents a compilation operation with all necessary parameters including
/// source file, output location, optimization level, and compiler flags.
///
/// # Example
///
/// ```no_run
/// use axiom_toolchain::CompileRequest;
/// use std::path::PathBuf;
///
/// let request = CompileRequest::new(
///     PathBuf::from("main.c"),
///     PathBuf::from("main.o"),
/// )
/// .with_optimization(2)
/// .with_flag("-Wall");
/// ```
#[derive(Debug, Clone)]
pub struct CompileRequest {
    /// Source file path to compile.
    pub source: PathBuf,
    /// Output object file path.
    pub output: PathBuf,
    /// Target architecture for cross-compilation (optional).
    pub target: Option<String>,
    /// Additional compiler flags to pass.
    pub flags: Vec<String>,
    /// Optimization level (0-3, where 0 is no optimization).
    pub optimization: u8,
    /// Include debug symbols in the output.
    pub debug: bool,
}

impl CompileRequest {
    /// Create a new compile request with default settings.
    ///
    /// Default settings: optimization level 0, debug symbols enabled.
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

    /// Set the target architecture for cross-compilation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use axiom_toolchain::CompileRequest;
    /// # use std::path::PathBuf;
    /// let request = CompileRequest::new(PathBuf::from("main.c"), PathBuf::from("main.o"))
    ///     .with_target("arm64-apple-darwin");
    /// ```
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Add a compiler flag.
    ///
    /// Flags are passed directly to the compiler in the order they are added.
    pub fn with_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Set optimization level (0-3).
    ///
    /// Values above 3 are clamped to 3.
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

/// Result of a compilation operation.
///
/// Contains the exit code, output streams, execution time, and parsed diagnostics
/// from the compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    /// Exit code of the compiler process.
    pub exit_code: i32,
    /// Standard output from the compiler.
    pub stdout: String,
    /// Standard error from the compiler.
    pub stderr: String,
    /// Compilation duration in milliseconds.
    pub duration_ms: u64,
    /// Parsed diagnostic messages (errors and warnings).
    pub diagnostics: Vec<axiom_core::Diagnostic>,
}

impl CompileResult {
    /// Check if compilation succeeded (exit code 0).
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

/// A request to compile ARM source code.
///
/// Extends the basic compile request with ARM-specific MCU configuration,
/// including CPU type, FPU settings, and preprocessor defines.
///
/// # Example
///
/// ```no_run
/// use axiom_toolchain::{ArmCompileRequest, ArmMcuConfig};
/// use std::path::PathBuf;
///
/// let mcu = ArmMcuConfig::cortex_m4()
///     .with_define("STM32F407xx");
///
/// let request = ArmCompileRequest::new(
///     PathBuf::from("main.c"),
///     PathBuf::from("main.o"),
///     mcu,
/// )
/// .with_include_path("Core/Inc")
/// .with_optimization(2);
/// ```
#[derive(Debug, Clone)]
pub struct ArmCompileRequest {
    /// Source file path to compile.
    pub source: PathBuf,
    /// Output object file path.
    pub output: PathBuf,
    /// MCU-specific configuration (CPU, FPU, defines).
    pub mcu: crate::ArmMcuConfig,
    /// Include directories for header files.
    pub include_paths: Vec<PathBuf>,
    /// Optimization level (0-3).
    pub optimization: u8,
    /// Include debug symbols.
    pub debug: bool,
}

impl ArmCompileRequest {
    /// Create a new ARM compile request.
    ///
    /// # Arguments
    ///
    /// * `source` - Source file to compile
    /// * `output` - Output object file path
    /// * `mcu` - MCU configuration with CPU type and flags
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

    /// Add an include directory path.
    ///
    /// Include paths are added in order and passed to the compiler with `-I` flags.
    pub fn with_include_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.include_paths.push(path.into());
        self
    }

    /// Add a preprocessor define.
    ///
    /// Defines are passed to the compiler with `-D` flags.
    pub fn with_define(mut self, define: impl Into<String>) -> Self {
        self.mcu = self.mcu.with_define(define);
        self
    }

    /// Set optimization level (0-3).
    ///
    /// Values above 3 are clamped to 3.
    pub fn with_optimization(mut self, level: u8) -> Self {
        self.optimization = level.min(3);
        self
    }

    /// Enable or disable debug symbols (-g3 flag).
    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// A request to link ARM object files into an executable.
///
/// Combines multiple object files with a linker script to produce an ELF executable
/// suitable for flashing to ARM microcontrollers.
///
/// # Example
///
/// ```no_run
/// use axiom_toolchain::{ArmLinkRequest, ArmMcuConfig, LinkerConfig};
/// use std::path::PathBuf;
///
/// let mcu = ArmMcuConfig::cortex_m4();
/// let linker = LinkerConfig::new("STM32F407.ld")
///     .with_map("output.map");
///
/// let request = ArmLinkRequest::new(
///     vec![PathBuf::from("main.o"), PathBuf::from("gpio.o")],
///     PathBuf::from("firmware.elf"),
///     linker,
///     mcu,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ArmLinkRequest {
    /// Object files to link together.
    pub objects: Vec<PathBuf>,
    /// Output ELF executable path.
    pub output: PathBuf,
    /// Linker configuration (script, map file, flags).
    pub linker: crate::LinkerConfig,
    /// MCU configuration for linker flags.
    pub mcu: crate::ArmMcuConfig,
}

impl ArmLinkRequest {
    /// Create a new ARM link request.
    ///
    /// # Arguments
    ///
    /// * `objects` - Object files to link
    /// * `output` - Output ELF file path
    /// * `linker` - Linker configuration with script and options
    /// * `mcu` - MCU configuration for architecture-specific flags
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
///
/// Contains the exit code, output streams, and parsed diagnostics from the linker.
/// Includes helper methods to check for common linker errors like memory overflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkResult {
    /// Exit code of the linker process.
    pub exit_code: i32,
    /// Standard output from the linker.
    pub stdout: String,
    /// Standard error from the linker.
    pub stderr: String,
    /// Parsed diagnostic messages.
    pub diagnostics: Vec<axiom_core::Diagnostic>,
}

impl LinkResult {
    /// Check if linking succeeded (exit code 0).
    pub fn success(&self) -> bool {
        self.exit_code == 0
    }

    /// Check if there was a memory overflow error.
    ///
    /// Detects common linker error messages indicating that the program
    /// is too large for the target MCU's memory regions.
    pub fn has_memory_overflow(&self) -> bool {
        self.stderr.contains("will not fit")
            || self.stderr.contains("region") && self.stderr.contains("overflow")
    }
}
