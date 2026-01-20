// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Compliance mode management for DO-178C, DO-330, and ARP4754A standards.
//!
//! This module provides functionality to enable, disable, and manage multiple
//! compliance modes simultaneously.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

/// Manages the state of compliance modes for a project.
///
/// The ComplianceSystem allows enabling and disabling multiple compliance modes
/// simultaneously. When modes are disabled, their data is preserved for future
/// re-enablement and deviation analysis.
#[derive(Debug, Clone, Default)]
pub struct ComplianceSystem {
    /// Set of currently enabled compliance modes
    enabled_modes: HashSet<ComplianceMode>,
}

impl ComplianceSystem {
    /// Creates a new ComplianceSystem with no modes enabled.
    pub fn new() -> Self {
        Self {
            enabled_modes: HashSet::new(),
        }
    }

    /// Enables a compliance mode.
    ///
    /// If the mode is already enabled, this is a no-op.
    /// Multiple modes can be enabled simultaneously.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to enable
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// system.enable_mode(ComplianceMode::Do178c);
    /// assert!(system.is_mode_enabled(ComplianceMode::Do178c));
    /// ```
    pub fn enable_mode(&mut self, mode: ComplianceMode) {
        self.enabled_modes.insert(mode);
    }

    /// Disables a compliance mode.
    ///
    /// The mode's collected data is preserved for future re-enablement.
    /// If the mode is not currently enabled, this is a no-op.
    ///
    /// # Arguments
    ///
    /// * `mode` - The compliance mode to disable
    ///
    /// # Examples
    ///
    /// ```
    /// use axiom_compliance::{ComplianceSystem, ComplianceMode};
    ///
    /// let mut system = ComplianceSystem::new();
    /// system.enable_mode(ComplianceMode::Do178c);
    /// system.disable_mode(ComplianceMode::Do178c);
    /// assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
    /// ```
    pub fn disable_mode(&mut self, mode: ComplianceMode) {
        self.enabled_modes.remove(&mode);
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

        system.disable_mode(ComplianceMode::Do178c);
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
        system.disable_mode(ComplianceMode::Do330);
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(!system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));
    }
}
