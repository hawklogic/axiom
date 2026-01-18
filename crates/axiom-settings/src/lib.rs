// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Axiom Settings
//!
//! TOML-based settings management with versioned schema and migrations.

mod schema;
mod migration;
mod persistence;

pub use schema::*;
pub use migration::*;
pub use persistence::*;

/// Current settings schema version.
pub const SCHEMA_VERSION: u32 = 1;
