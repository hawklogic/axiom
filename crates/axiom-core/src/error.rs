// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Error types for Axiom.

use thiserror::Error;

/// Core error type for Axiom operations.
#[derive(Error, Debug)]
pub enum AxiomError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Toolchain error: {0}")]
    Toolchain(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("{0}")]
    Other(String),
}

/// Result type alias using AxiomError.
pub type Result<T> = std::result::Result<T, AxiomError>;
