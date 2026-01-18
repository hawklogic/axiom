// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom Parser
//!
//! C/C++ parsing using tree-sitter.

mod ast;
mod parser;

pub use ast::*;
pub use parser::*;
