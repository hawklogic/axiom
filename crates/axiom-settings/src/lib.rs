// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom Settings
//!
//! TOML-based settings management with versioned schema and migrations.

mod migration;
mod persistence;
mod schema;

pub use migration::*;
pub use persistence::*;
pub use schema::*;

/// Current settings schema version.
pub const SCHEMA_VERSION: u32 = 1;
