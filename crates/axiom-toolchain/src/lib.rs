// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom Toolchain
//!
//! This crate provides comprehensive toolchain detection, configuration, and invocation
//! capabilities for the Axiom IDE. It supports multiple toolchain types including:
//!
//! - **Clang/LLVM**: Modern C/C++ compiler with excellent diagnostics
//! - **GCC**: GNU Compiler Collection for native development
//! - **ARM GCC**: ARM embedded toolchain (arm-none-eabi-gcc) for microcontroller development
//! - **Python**: Python interpreter detection
//!
//! # ARM Toolchain Support
//!
//! The crate provides extensive support for ARM embedded development workflows:
//!
//! - **Detection**: Automatically finds ARM toolchains from Homebrew, STM32CubeIDE, and system paths
//! - **Compilation**: MCU-specific compiler flag generation (CPU, FPU, ABI)
//! - **Linking**: Linker script support with memory map generation
//! - **Binary Generation**: ELF to HEX/BIN conversion with size reporting
//! - **Visualization**: Preprocessor, assembly, and disassembly output
//! - **Makefile Integration**: Support for existing Makefile-based projects
//!
//! # Example: Detecting ARM Toolchains
//!
//! ```no_run
//! use axiom_toolchain::detect_arm_toolchains;
//!
//! let toolchains = detect_arm_toolchains();
//! for suite in toolchains {
//!     println!("Found ARM toolchain: {} at {:?}", suite.version, suite.gcc);
//! }
//! ```
//!
//! # Example: Compiling for ARM
//!
//! ```no_run
//! use axiom_toolchain::{ArmMcuConfig, ArmCompileRequest, compile_arm};
//! use std::path::PathBuf;
//!
//! let mcu = ArmMcuConfig::cortex_m4()
//!     .with_define("STM32F407xx");
//!
//! let request = ArmCompileRequest::new(
//!     PathBuf::from("main.c"),
//!     PathBuf::from("main.o"),
//!     mcu,
//! );
//!
//! // Compile (requires arm-none-eabi-gcc in PATH)
//! // let result = compile_arm(Path::new("arm-none-eabi-gcc"), &request);
//! ```

mod arm_mcu;
mod binary_gen;
mod detection;
mod error;
mod invocation;
mod makefile;
mod types;
mod visualizer;

pub use arm_mcu::*;
pub use binary_gen::*;
pub use detection::*;
pub use error::*;
pub use invocation::*;
pub use makefile::*;
pub use types::*;
pub use visualizer::*;
