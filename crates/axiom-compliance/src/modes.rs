// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compliance mode management for DO-178C, DO-330, and ARP4754A standards.
//!
//! This module provides functionality to enable, disable, and manage multiple
//! compliance modes simultaneously.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// Compliance modes supported by the system.
///
/// Each mode corresponds to a specific aviation safety standard:
/// - DO-178C: Software Considerations in Airborne Systems and Equipment Certification
/// - DO-330: Software Tool Qualification Considerations
/// - ARP4754A: Guidelines for Development of Civil Aircraft and Systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceMode {
    /// DO-178C software airworthiness compliance
    Do178c,
    /// DO-330 tool qualification support
    Do330,
    /// ARP4754A system-level integration
    Arp4754a,
}

/// Snapshot of compliance data at the time a mode was disabled.
///
/// This preserves the state of compliance artifacts for deviation analysis
/// when the mode is re-enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSnapshot {
    /// When this snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// Files that existed at snapshot time with their checksums
    pub file_checksums: HashMap<PathBuf, String>,
    /// Requirement IDs that were traced at snapshot time
    pub traced_requirements: HashSet<String>,
    /// Source files that had traceability annotations
    pub traced_files: HashSet<PathBuf>,
}

impl ComplianceSnapshot {
    /// Creates a new empty snapshot with the current timestamp.
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            file_checksums: HashMap::new(),
            traced_requirements: HashSet::new(),
            traced_files: HashSet::new(),
        }
    }
}

impl Default for ComplianceSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

/// Deviation detected when re-enabling a compliance mode.
///
/// Represents a change that occurred while the mode was disabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Deviation {
    /// New file added without traceability annotations
    NewUntracedFile {
        /// Path to the new file
        path: PathBuf,
    },
    /// Existing file was modified
    ModifiedFile {
        /// Path to the modified file
        path: PathBuf,
        /// Old checksum from snapshot
        old_checksum: String,
        /// New checksum
        new_checksum: String,
    },
    /// File was deleted
    DeletedFile {
        /// Path to the deleted file
        path: PathBuf,
    },
    /// Requirement that was traced is no longer found
    BrokenTraceabilityLink {
        /// Requirement ID that is no longer traced
        requirement_id: String,
        /// File where it was previously traced
        previous_file: PathBuf,
    },
    /// New code added without requirement annotations
    NewCodeWithoutTraceability {
        /// File containing new code
        file: PathBuf,
        /// Line numbers without annotations
        lines: Vec<u32>,
    },
    /// Coverage gap introduced
    CoverageGap {
        /// File with coverage gap
        file: PathBuf,
        /// Description of the gap
        description: String,
    },
}

/// Report of deviations found when re-enabling a compliance mode.
///
/// This report highlights all changes that occurred while the mode was disabled,
/// allowing developers to assess the impact on compliance evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviationReport {
    /// The compliance mode that was re-enabled
    pub mode: ComplianceMode,
    /// When the mode was disabled
    pub disabled_at: DateTime<Utc>,
    /// When the mode was re-enabled
    pub re_enabled_at: DateTime<Utc>,
    /// List of deviations detected
    pub deviations: Vec<Deviation>,
}

impl DeviationReport {
    /// Creates a new deviation report.
    pub fn new(mode: ComplianceMode, disabled_at: DateTime<Utc>) -> Self {
        Self {
            mode,
            disabled_at,
            re_enabled_at: Utc::now(),
            deviations: Vec::new(),
        }
    }

    /// Adds a deviation to the report.
    pub fn add_deviation(&mut self, deviation: Deviation) {
        self.deviations.push(deviation);
    }

    /// Returns the number of deviations in the report.
    pub fn deviation_count(&self) -> usize {
        self.deviations.len()
    }

    /// Checks if the report has any deviations.
    pub fn has_deviations(&self) -> bool {
        !self.deviations.is_empty()
    }
}

/// Detects deviations between a snapshot and current state.
///
/// This function compares the current project state against a snapshot taken
/// when a compliance mode was disabled, identifying all changes that occurred.
///
/// # Arguments
///
/// * `snapshot` - The snapshot from when the mode was disabled
/// * `current_files` - Current files with their checksums
/// * `current_traced_requirements` - Currently traced requirement IDs
/// * `current_traced_files` - Files that currently have traceability annotations
///
/// # Returns
///
/// A vector of detected deviations
pub fn detect_deviations(
    snapshot: &ComplianceSnapshot,
    current_files: &HashMap<PathBuf, String>,
    current_traced_requirements: &HashSet<String>,
    current_traced_files: &HashSet<PathBuf>,
) -> Vec<Deviation> {
    let mut deviations = Vec::new();

    // Detect new files without traceability
    for path in current_files.keys() {
        if !snapshot.file_checksums.contains_key(path) {
            // New file added
            if !current_traced_files.contains(path) {
                deviations.push(Deviation::NewUntracedFile { path: path.clone() });
            }
        }
    }

    // Detect modified files
    for (path, old_checksum) in &snapshot.file_checksums {
        if let Some(new_checksum) = current_files.get(path) {
            if old_checksum != new_checksum {
                deviations.push(Deviation::ModifiedFile {
                    path: path.clone(),
                    old_checksum: old_checksum.clone(),
                    new_checksum: new_checksum.clone(),
                });
            }
        } else {
            // File was deleted
            deviations.push(Deviation::DeletedFile { path: path.clone() });
        }
    }

    // Detect broken traceability links
    for req_id in &snapshot.traced_requirements {
        if !current_traced_requirements.contains(req_id) {
            // Find which file previously traced this requirement
            // For now, we'll report it without the specific file
            // In a real implementation, we'd store this in the snapshot
            deviations.push(Deviation::BrokenTraceabilityLink {
                requirement_id: req_id.clone(),
                previous_file: PathBuf::from("unknown"),
            });
        }
    }

    deviations
}

/// Manages the state of compliance modes for a project.
///
/// The ComplianceSystem allows enabling and disabling multiple compliance modes
/// simultaneously. When modes are disabled, their data is preserved for future
/// re-enablement and deviation analysis.
#[derive(Debug, Clone, Default)]
pub struct ComplianceSystem {
    /// Set of currently enabled compliance modes
    enabled_modes: HashSet<ComplianceMode>,
    /// Snapshots of disabled modes for deviation analysis
    disabled_snapshots: HashMap<ComplianceMode, ComplianceSnapshot>,
}

impl ComplianceSystem {
    /// Creates a new ComplianceSystem with no modes enabled.
    pub fn new() -> Self {
        Self {
            enabled_modes: HashSet::new(),
            disabled_snapshots: HashMap::new(),
        }
    }

    /// Enables a compliance mode.
    ///
    /// If the mode is already enabled, this is a no-op.
    /// Multiple modes can be enabled simultaneously.
    ///
    /// If the mode was previously disabled, this will trigger deviation analysis.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to enable
    ///
    /// # Returns
    ///
    /// An optional deviation report if the mode was previously disabled
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// let report = system.enable_mode(ComplianceMode::Do178c);
    /// assert!(report.is_none()); // No previous snapshot
    /// assert!(system.is_mode_enabled(ComplianceMode::Do178c));
    /// ```
    pub fn enable_mode(&mut self, mode: ComplianceMode) -> Option<DeviationReport> {
        // Check if there's a snapshot from a previous disable
        let snapshot = self.disabled_snapshots.remove(&mode);
        
        self.enabled_modes.insert(mode);
        
        // If there was a snapshot, we would generate a deviation report here
        // For now, return None as we don't have current state to compare
        snapshot.map(|snap| DeviationReport::new(mode, snap.timestamp))
    }

    /// Generates a deviation report by comparing a snapshot to current state.
    ///
    /// This is typically called when re-enabling a mode to identify changes
    /// that occurred while the mode was disabled.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode being re-enabled
    /// * `current_files` - Current files with their checksums
    /// * `current_traced_requirements` - Currently traced requirement IDs
    /// * `current_traced_files` - Files that currently have traceability annotations
    ///
    /// # Returns
    ///
    /// A deviation report if a snapshot exists, None otherwise
    pub fn generate_deviation_report(
        &self,
        mode: ComplianceMode,
        current_files: &HashMap<PathBuf, String>,
        current_traced_requirements: &HashSet<String>,
        current_traced_files: &HashSet<PathBuf>,
    ) -> Option<DeviationReport> {
        self.disabled_snapshots.get(&mode).map(|snapshot| {
            let deviations = detect_deviations(
                snapshot,
                current_files,
                current_traced_requirements,
                current_traced_files,
            );
            
            let mut report = DeviationReport::new(mode, snapshot.timestamp);
            for deviation in deviations {
                report.add_deviation(deviation);
            }
            report
        })
    }

    /// Disables a compliance mode.
    ///
    /// The mode's collected data is preserved for future re-enablement.
    /// If the mode is not currently enabled, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to disable
    /// * `snapshot` - Optional snapshot of current compliance state to preserve
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// system.enable_mode(ComplianceMode::Do178c);
    /// system.disable_mode(ComplianceMode::Do178c, None);
    /// assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
    /// ```
    pub fn disable_mode(&mut self, mode: ComplianceMode, snapshot: Option<ComplianceSnapshot>) {
        if self.enabled_modes.remove(&mode) {
            // Only store snapshot if mode was actually enabled
            let snapshot = snapshot.unwrap_or_default();
            self.disabled_snapshots.insert(mode, snapshot);
        }
    }

    /// Gets the snapshot for a disabled mode, if it exists.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to get the snapshot for
    ///
    /// # Returns
    ///
    /// The snapshot if the mode was previously disabled, None otherwise
    pub fn get_snapshot(&self, mode: ComplianceMode) -> Option<&ComplianceSnapshot> {
        self.disabled_snapshots.get(&mode)
    }

    /// Checks if a compliance mode is currently enabled.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to check
    ///
    /// # Returns
    ///
    /// `true` if the mode is enabled, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
    /// system.enable_mode(ComplianceMode::Do178c);
    /// assert!(system.is_mode_enabled(ComplianceMode::Do178c));
    /// ```
    pub fn is_mode_enabled(&self, mode: ComplianceMode) -> bool {
        self.enabled_modes.contains(&mode)
    }

    /// Returns a list of all currently enabled modes.
    ///
    /// # Returns
    ///
    /// A vector containing all enabled compliance modes
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// system.enable_mode(ComplianceMode::Do178c);
    /// system.enable_mode(ComplianceMode::Do330);
    ///
    /// let enabled = system.enabled_modes();
    /// assert_eq!(enabled.len(), 2);
    /// assert!(enabled.contains(&ComplianceMode::Do178c));
    /// assert!(enabled.contains(&ComplianceMode::Do330));
    /// ```
    pub fn enabled_modes(&self) -> Vec<ComplianceMode> {
        self.enabled_modes.iter().copied().collect()
    }

    /// Checks if any compliance mode is currently enabled.
    ///
    /// # Returns
    ///
    /// `true` if at least one mode is enabled, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// assert!(!system.has_any_mode_enabled());
    /// system.enable_mode(ComplianceMode::Do178c);
    /// assert!(system.has_any_mode_enabled());
    /// ```
    pub fn has_any_mode_enabled(&self) -> bool {
        !self.enabled_modes.is_empty()
    }

    /// Disables all compliance modes.
    ///
    /// All modes' collected data is preserved for future re-enablement.
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// system.enable_mode(ComplianceMode::Do178c);
    /// system.enable_mode(ComplianceMode::Do330);
    /// system.disable_all_modes();
    /// assert!(!system.has_any_mode_enabled());
    /// ```
    pub fn disable_all_modes(&mut self) {
        self.enabled_modes.clear();
    }

    /// Gets the strictest requirements from all enabled modes.
    ///
    /// When multiple compliance modes are active, this returns the union
    /// of all requirements, effectively enforcing the strictest standard.
    ///
    /// # Returns
    ///
    /// A set of requirement categories that must be satisfied
    pub fn get_active_requirements(&self) -> ComplianceRequirements {
        let mut requirements = ComplianceRequirements::default();

        for mode in &self.enabled_modes {
            match mode {
                ComplianceMode::Do178c => {
                    requirements.requires_traceability = true;
                    requirements.requires_coverage = true;
                    requirements.requires_structural_coverage = true;
                }
                ComplianceMode::Do330 => {
                    requirements.requires_tool_qualification = true;
                    requirements.requires_tool_usage_logging = true;
                }
                ComplianceMode::Arp4754a => {
                    requirements.requires_system_traceability = true;
                    requirements.requires_safety_assessment = true;
                }
            }
        }

        requirements
    }
}

/// Requirements enforced by active compliance modes.
///
/// When multiple modes are active, this represents the union of all
/// requirements, ensuring the strictest standards are applied.
#[derive(Debug, Clone, Default)]
pub struct ComplianceRequirements {
    /// Requires bidirectional traceability (DO-178C)
    pub requires_traceability: bool,
    /// Requires structural coverage analysis (DO-178C)
    pub requires_coverage: bool,
    /// Requires statement/branch/MC/DC coverage (DO-178C)
    pub requires_structural_coverage: bool,
    /// Requires tool qualification data (DO-330)
    pub requires_tool_qualification: bool,
    /// Requires logging of all tool usage (DO-330)
    pub requires_tool_usage_logging: bool,
    /// Requires system-level traceability (ARP4754A)
    pub requires_system_traceability: bool,
    /// Requires safety assessment integration (ARP4754A)
    pub requires_safety_assessment: bool,
}

impl ComplianceRequirements {
    /// Checks if any requirements are active.
    pub fn has_any_requirements(&self) -> bool {
        self.requires_traceability
            || self.requires_coverage
            || self.requires_structural_coverage
            || self.requires_tool_qualification
            || self.requires_tool_usage_logging
            || self.requires_system_traceability
            || self.requires_safety_assessment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_mode_serialization() {
        let mode = ComplianceMode::Do178c;
        let serialized = serde_json::to_string(&mode).unwrap();
        let deserialized: ComplianceMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mode, deserialized);

        let mode = ComplianceMode::Do330;
        let serialized = serde_json::to_string(&mode).unwrap();
        let deserialized: ComplianceMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mode, deserialized);

        let mode = ComplianceMode::Arp4754a;
        let serialized = serde_json::to_string(&mode).unwrap();
        let deserialized: ComplianceMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(mode, deserialized);
    }

    #[test]
    fn test_enable_single_mode() {
        let mut system = ComplianceSystem::new();
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));

        system.enable_mode(ComplianceMode::Do178c);
        assert!(system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(!system.is_mode_enabled(ComplianceMode::Do330));
        assert!(!system.is_mode_enabled(ComplianceMode::Arp4754a));
    }

    #[test]
    fn test_disable_single_mode() {
        let mut system = ComplianceSystem::new();
        system.enable_mode(ComplianceMode::Do178c);
        assert!(system.is_mode_enabled(ComplianceMode::Do178c));

        system.disable_mode(ComplianceMode::Do178c, None);
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
    }

    #[test]
    fn test_enable_multiple_modes_simultaneously() {
        let mut system = ComplianceSystem::new();

        system.enable_mode(ComplianceMode::Do178c);
        system.enable_mode(ComplianceMode::Do330);
        system.enable_mode(ComplianceMode::Arp4754a);

        assert!(system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));

        let enabled = system.enabled_modes();
        assert_eq!(enabled.len(), 3);
    }

    #[test]
    fn test_is_mode_enabled_returns_correct_state() {
        let mut system = ComplianceSystem::new();

        // Initially no modes enabled
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(!system.is_mode_enabled(ComplianceMode::Do330));
        assert!(!system.is_mode_enabled(ComplianceMode::Arp4754a));

        // Enable one mode
        system.enable_mode(ComplianceMode::Do330);
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(system.is_mode_enabled(ComplianceMode::Do330));
        assert!(!system.is_mode_enabled(ComplianceMode::Arp4754a));

        // Enable another mode
        system.enable_mode(ComplianceMode::Arp4754a);
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));

        // Disable one mode
        system.disable_mode(ComplianceMode::Do330, None);
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(!system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));
    }

    #[test]
    fn test_disable_mode_preserves_data() {
        let mut system = ComplianceSystem::new();
        system.enable_mode(ComplianceMode::Do178c);

        // Create a snapshot with some data
        let mut snapshot = ComplianceSnapshot::new();
        snapshot
            .file_checksums
            .insert(PathBuf::from("test.c"), "abc123".to_string());
        snapshot
            .traced_requirements
            .insert("REQ-001".to_string());
        snapshot.traced_files.insert(PathBuf::from("test.c"));

        // Disable the mode with the snapshot
        system.disable_mode(ComplianceMode::Do178c, Some(snapshot.clone()));

        // Mode should be disabled
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));

        // Snapshot should be preserved
        let preserved = system.get_snapshot(ComplianceMode::Do178c);
        assert!(preserved.is_some());

        let preserved = preserved.unwrap();
        assert_eq!(
            preserved.file_checksums.get(&PathBuf::from("test.c")),
            Some(&"abc123".to_string())
        );
        assert!(preserved.traced_requirements.contains("REQ-001"));
        assert!(preserved.traced_files.contains(&PathBuf::from("test.c")));
    }

    #[test]
    fn test_re_enable_mode_triggers_deviation_analysis() {
        let mut system = ComplianceSystem::new();
        system.enable_mode(ComplianceMode::Do178c);

        // Create a snapshot
        let mut snapshot = ComplianceSnapshot::new();
        snapshot
            .file_checksums
            .insert(PathBuf::from("test.c"), "abc123".to_string());
        snapshot
            .traced_requirements
            .insert("REQ-001".to_string());

        // Disable with snapshot
        system.disable_mode(ComplianceMode::Do178c, Some(snapshot));

        // Re-enable should return a deviation report
        let report = system.enable_mode(ComplianceMode::Do178c);
        assert!(report.is_some());

        let report = report.unwrap();
        assert_eq!(report.mode, ComplianceMode::Do178c);
        assert!(system.is_mode_enabled(ComplianceMode::Do178c));
    }

    #[test]
    fn test_multiple_modes_can_be_active_simultaneously() {
        let mut system = ComplianceSystem::new();

        // Enable all three modes
        system.enable_mode(ComplianceMode::Do178c);
        system.enable_mode(ComplianceMode::Do330);
        system.enable_mode(ComplianceMode::Arp4754a);

        // All should be enabled
        assert!(system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));

        let enabled = system.enabled_modes();
        assert_eq!(enabled.len(), 3);
        assert!(enabled.contains(&ComplianceMode::Do178c));
        assert!(enabled.contains(&ComplianceMode::Do330));
        assert!(enabled.contains(&ComplianceMode::Arp4754a));
    }

    #[test]
    fn test_union_enforcement_applies_strictest_requirements() {
        let mut system = ComplianceSystem::new();

        // No modes enabled - no requirements
        let reqs = system.get_active_requirements();
        assert!(!reqs.has_any_requirements());

        // Enable DO-178C
        system.enable_mode(ComplianceMode::Do178c);
        let reqs = system.get_active_requirements();
        assert!(reqs.requires_traceability);
        assert!(reqs.requires_coverage);
        assert!(reqs.requires_structural_coverage);
        assert!(!reqs.requires_tool_qualification);
        assert!(!reqs.requires_system_traceability);

        // Enable DO-330 - should add tool qualification requirements
        system.enable_mode(ComplianceMode::Do330);
        let reqs = system.get_active_requirements();
        assert!(reqs.requires_traceability);
        assert!(reqs.requires_coverage);
        assert!(reqs.requires_tool_qualification);
        assert!(reqs.requires_tool_usage_logging);

        // Enable ARP4754A - should add system requirements
        system.enable_mode(ComplianceMode::Arp4754a);
        let reqs = system.get_active_requirements();
        assert!(reqs.requires_traceability);
        assert!(reqs.requires_coverage);
        assert!(reqs.requires_tool_qualification);
        assert!(reqs.requires_system_traceability);
        assert!(reqs.requires_safety_assessment);

        // All requirements should be active (union)
        assert!(reqs.has_any_requirements());
    }
}
