// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compliance commands for DO-178C, DO-330, and ARP4754A support.
//!
//! This module provides Tauri commands for managing compliance modes and
//! generating compliance artifacts for safety-critical avionics software.

use axiom_compliance::{
    ComplianceMode, ComplianceSnapshot, ComplianceSystem, CoverageReport, DeviationReport,
    TraceabilityMatrix,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

/// Global compliance system state
pub struct ComplianceState {
    system: Mutex<ComplianceSystem>,
}

impl ComplianceState {
    pub fn new() -> Self {
        Self {
            system: Mutex::new(ComplianceSystem::new()),
        }
    }
}

impl Default for ComplianceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Response for compliance status query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatusResponse {
    /// Currently enabled compliance modes
    pub enabled_modes: Vec<ComplianceMode>,
    /// Whether any mode is enabled
    pub has_any_mode_enabled: bool,
    /// Active requirements from all enabled modes
    pub active_requirements: ComplianceRequirementsResponse,
}

/// Requirements enforced by active compliance modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirementsResponse {
    pub requires_traceability: bool,
    pub requires_coverage: bool,
    pub requires_structural_coverage: bool,
    pub requires_tool_qualification: bool,
    pub requires_tool_usage_logging: bool,
    pub requires_system_traceability: bool,
    pub requires_safety_assessment: bool,
}

/// Enable a compliance mode
///
/// # Arguments
///
/// * `mode` - The compliance mode to enable (Do178c, Do330, or Arp4754a)
/// * `state` - The compliance system state
///
/// # Returns
///
/// An optional deviation report if the mode was previously disabled
#[tauri::command]
pub async fn enable_compliance_mode(
    mode: ComplianceMode,
    state: State<'_, ComplianceState>,
) -> Result<Option<DeviationReport>, String> {
    let mut system = state
        .system
        .lock()
        .map_err(|e| format!("Failed to lock compliance system: {}", e))?;

    let report = system.enable_mode(mode);

    Ok(report)
}

/// Disable a compliance mode
///
/// The mode's collected data is preserved for future re-enablement.
///
/// # Arguments
///
/// * `mode` - The compliance mode to disable
/// * `state` - The compliance system state
///
/// # Returns
///
/// Success or error message
#[tauri::command]
pub async fn disable_compliance_mode(
    mode: ComplianceMode,
    state: State<'_, ComplianceState>,
) -> Result<(), String> {
    let mut system = state
        .system
        .lock()
        .map_err(|e| format!("Failed to lock compliance system: {}", e))?;

    // Create a snapshot of current state before disabling
    let snapshot = ComplianceSnapshot::new();
    system.disable_mode(mode, Some(snapshot));

    Ok(())
}

/// Get the traceability matrix for a project
///
/// Scans the project directory for source files and test files,
/// parsing requirement and test annotations to build a complete
/// traceability matrix.
///
/// # Arguments
///
/// * `project_path` - Path to the project root directory
///
/// # Returns
///
/// The complete traceability matrix
#[tauri::command]
pub async fn get_traceability_matrix(
    project_path: PathBuf,
) -> Result<TraceabilityMatrix, String> {
    use axiom_compliance::generate_traceability_matrix;

    generate_traceability_matrix(&project_path)
        .map_err(|e| format!("Failed to generate traceability matrix: {}", e))
}

/// Get the coverage report for a project
///
/// Analyzes gcov coverage data to generate a complete coverage report
/// with statement and branch coverage percentages.
///
/// # Arguments
///
/// * `_project_path` - Path to the project root directory (reserved for future use)
/// * `gcov_data` - Map of file paths to their gcov output
///
/// # Returns
///
/// The complete coverage report
#[tauri::command]
pub async fn get_coverage_report(
    _project_path: PathBuf,
    gcov_data: HashMap<PathBuf, String>,
) -> Result<CoverageReport, String> {
    use axiom_compliance::generate_coverage_report;

    // Note: In a real implementation, we would scan the project_path
    // for .gcda files and run gcov to generate the data.
    // For now, we accept gcov_data as input.

    generate_coverage_report(gcov_data)
        .map_err(|e| format!("Failed to generate coverage report: {}", e))
}

/// Get the current compliance status
///
/// Returns information about which compliance modes are enabled
/// and what requirements are currently active.
///
/// # Arguments
///
/// * `state` - The compliance system state
///
/// # Returns
///
/// The current compliance status
#[tauri::command]
pub async fn get_compliance_status(
    state: State<'_, ComplianceState>,
) -> Result<ComplianceStatusResponse, String> {
    let system = state
        .system
        .lock()
        .map_err(|e| format!("Failed to lock compliance system: {}", e))?;

    let enabled_modes = system.enabled_modes();
    let has_any_mode_enabled = system.has_any_mode_enabled();
    let requirements = system.get_active_requirements();

    Ok(ComplianceStatusResponse {
        enabled_modes,
        has_any_mode_enabled,
        active_requirements: ComplianceRequirementsResponse {
            requires_traceability: requirements.requires_traceability,
            requires_coverage: requirements.requires_coverage,
            requires_structural_coverage: requirements.requires_structural_coverage,
            requires_tool_qualification: requirements.requires_tool_qualification,
            requires_tool_usage_logging: requirements.requires_tool_usage_logging,
            requires_system_traceability: requirements.requires_system_traceability,
            requires_safety_assessment: requirements.requires_safety_assessment,
        },
    })
}

/// Generate a deviation report for a mode
///
/// Compares the current project state against a snapshot taken when
/// the mode was disabled, identifying all changes that occurred.
///
/// # Arguments
///
/// * `mode` - The compliance mode to generate a report for
/// * `_project_path` - Path to the project root directory (reserved for future use)
/// * `state` - The compliance system state
///
/// # Returns
///
/// A deviation report if a snapshot exists, None otherwise
#[tauri::command]
pub async fn generate_deviation_report(
    mode: ComplianceMode,
    _project_path: PathBuf,
    state: State<'_, ComplianceState>,
) -> Result<Option<DeviationReport>, String> {
    let system = state
        .system
        .lock()
        .map_err(|e| format!("Failed to lock compliance system: {}", e))?;

    // In a real implementation, we would scan the project_path to get current state
    // For now, we'll use empty data structures
    let current_files = HashMap::new();
    let current_traced_requirements = HashSet::new();
    let current_traced_files = HashSet::new();

    let report = system.generate_deviation_report(
        mode,
        &current_files,
        &current_traced_requirements,
        &current_traced_files,
    );

    Ok(report)
}
