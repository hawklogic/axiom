// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! # Axiom Compliance
//!
//! Compliance and certification support for safety-critical avionics software.
//!
//! This crate provides functionality for:
//! - DO-178C software airworthiness compliance
//! - DO-330 tool qualification support
//! - ARP4754A system-level integration
//!
//! ## Safety-Critical Context
//!
//! This module supports development of safety-critical avionics software where human lives
//! depend on the correctness and reliability of the compiled code and compliance artifacts.
//! All components operate with the highest levels of integrity, accuracy, and traceability.
//!
//! **CRITICAL**: No AI-generated content, approximations, or unverified automation may be
//! used in any compliance-related functionality.

pub mod coverage;
pub mod error;
pub mod modes;
pub mod tool_qualification;
pub mod traceability;

pub use coverage::{
    build_coverage_flags, calculate_branch_coverage, calculate_statement_coverage,
    generate_coverage_report, parse_gcov_output, CoverageReport, FileCoverage,
};
pub use error::{ComplianceError, Result};
pub use modes::{
    detect_deviations, ComplianceMode, ComplianceRequirements, ComplianceSnapshot,
    ComplianceSystem, Deviation, DeviationReport,
};
pub use tool_qualification::{
    compute_sha256, ToolQualificationError, ToolQualificationLogger, ToolUsageRecord,
};
pub use traceability::{
    export_matrix_csv, find_untested_requirements, find_untraceable_functions,
    generate_traceability_matrix, parse_requirement_annotations, parse_test_annotations, LinkType,
    TraceabilityLink, TraceabilityMatrix, UntraceableFunction,
};
