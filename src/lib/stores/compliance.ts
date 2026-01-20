// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Compliance store - manages DO-178C, DO-330, and ARP4754A compliance modes.
 */

import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// TypeScript interfaces matching the Rust types

export type ComplianceMode = 'Do178c' | 'Do330' | 'Arp4754a';

export interface TraceabilityLink {
  requirement_id: string;
  source_file: string;
  line_number: number;
  link_type: 'Implementation' | 'Test' | 'Derived';
  created_at: string;
}

export interface TraceabilityMatrix {
  links: TraceabilityLink[];
  requirements: string[];
  source_files: string[];
  test_files: string[];
  untraceable_functions: UntraceableFunction[];
  untested_requirements: string[];
}

export interface UntraceableFunction {
  function_name: string;
  file: string;
  line_number: number;
}

export interface FileCoverage {
  file: string;
  statement_coverage: number;
  branch_coverage: number;
  decision_coverage: number;
  mcdc_coverage: number | null;
  uncovered_lines: number[];
  uncovered_branches: BranchInfo[];
}

export interface BranchInfo {
  line_number: number;
  branch_id: string;
  taken: boolean;
}

export interface CoverageReport {
  files: FileCoverage[];
  total_statement_coverage: number;
  total_branch_coverage: number;
  total_decision_coverage: number;
  total_mcdc_coverage: number | null;
}

export interface ComplianceRequirements {
  requires_traceability: boolean;
  requires_coverage: boolean;
  requires_structural_coverage: boolean;
  requires_tool_qualification: boolean;
  requires_tool_usage_logging: boolean;
  requires_system_traceability: boolean;
  requires_safety_assessment: boolean;
}

export interface ComplianceStatus {
  enabled_modes: ComplianceMode[];
  has_any_mode_enabled: boolean;
  active_requirements: ComplianceRequirements;
}

export interface DeviationReport {
  mode: ComplianceMode;
  new_files: string[];
  modified_files: string[];
  deleted_files: string[];
  new_requirements: string[];
  broken_links: TraceabilityLink[];
  timestamp: string;
}

function createComplianceStore() {
  // Store for compliance mode states
  const complianceModes = writable<Set<ComplianceMode>>(new Set());
  
  // Store for traceability matrix
  const traceabilityMatrix = writable<TraceabilityMatrix | null>(null);
  
  // Store for coverage report
  const coverageReport = writable<CoverageReport | null>(null);
  
  // Store for compliance status
  const complianceStatus = writable<ComplianceStatus>({
    enabled_modes: [],
    has_any_mode_enabled: false,
    active_requirements: {
      requires_traceability: false,
      requires_coverage: false,
      requires_structural_coverage: false,
      requires_tool_qualification: false,
      requires_tool_usage_logging: false,
      requires_system_traceability: false,
      requires_safety_assessment: false,
    },
  });
  
  // Loading state
  const loading = writable(false);
  
  // Error state
  const error = writable<string | null>(null);

  // Derived stores
  const isDo178cEnabled = derived(
    complianceModes,
    ($modes) => $modes.has('Do178c')
  );

  const isDo330Enabled = derived(
    complianceModes,
    ($modes) => $modes.has('Do330')
  );

  const isArp4754aEnabled = derived(
    complianceModes,
    ($modes) => $modes.has('Arp4754a')
  );

  const hasAnyModeEnabled = derived(
    complianceModes,
    ($modes) => $modes.size > 0
  );

  const requiresTraceability = derived(
    complianceStatus,
    ($status) => $status.active_requirements.requires_traceability
  );

  const requiresCoverage = derived(
    complianceStatus,
    ($status) => $status.active_requirements.requires_coverage
  );

  return {
    // Readable stores
    complianceModes,
    traceabilityMatrix,
    coverageReport,
    complianceStatus,
    loading,
    error,
    
    // Derived stores
    isDo178cEnabled,
    isDo330Enabled,
    isArp4754aEnabled,
    hasAnyModeEnabled,
    requiresTraceability,
    requiresCoverage,

    /**
     * Toggle a compliance mode on or off.
     */
    async toggleComplianceMode(mode: ComplianceMode): Promise<void> {
      loading.set(true);
      error.set(null);
      
      try {
        let currentModes: Set<ComplianceMode> = new Set();
        complianceModes.subscribe(($modes) => {
          currentModes = new Set($modes);
        })();
        
        const isEnabled = currentModes.has(mode);
        
        if (isEnabled) {
          // Disable the mode
          await invoke('disable_compliance_mode', { mode });
          currentModes.delete(mode);
        } else {
          // Enable the mode
          const deviationReport = await invoke<DeviationReport | null>(
            'enable_compliance_mode',
            { mode }
          );
          
          // If there's a deviation report, log it
          if (deviationReport) {
            console.warn(`Deviation report for ${mode}:`, deviationReport);
          }
          
          currentModes.add(mode);
        }
        
        complianceModes.set(currentModes);
        
        // Refresh compliance status
        await this.refreshComplianceStatus();
      } catch (e) {
        const errorMsg = `Failed to toggle compliance mode ${mode}: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Refresh the traceability matrix for a project.
     */
    async refreshTraceability(projectPath: string): Promise<void> {
      loading.set(true);
      error.set(null);
      
      try {
        const matrix = await invoke<TraceabilityMatrix>(
          'get_traceability_matrix',
          { projectPath }
        );
        
        traceabilityMatrix.set(matrix);
      } catch (e) {
        const errorMsg = `Failed to refresh traceability matrix: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Refresh the coverage report for a project.
     */
    async refreshCoverage(
      projectPath: string,
      gcovData: Record<string, string>
    ): Promise<void> {
      loading.set(true);
      error.set(null);
      
      try {
        const report = await invoke<CoverageReport>(
          'get_coverage_report',
          { projectPath, gcovData }
        );
        
        coverageReport.set(report);
      } catch (e) {
        const errorMsg = `Failed to refresh coverage report: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Refresh the compliance status.
     */
    async refreshComplianceStatus(): Promise<void> {
      try {
        const status = await invoke<ComplianceStatus>('get_compliance_status');
        
        complianceStatus.set(status);
        
        // Update the compliance modes set
        complianceModes.set(new Set(status.enabled_modes));
      } catch (e) {
        const errorMsg = `Failed to refresh compliance status: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Generate a deviation report for a mode.
     */
    async generateDeviationReport(
      mode: ComplianceMode,
      projectPath: string
    ): Promise<DeviationReport | null> {
      loading.set(true);
      error.set(null);
      
      try {
        const report = await invoke<DeviationReport | null>(
          'generate_deviation_report',
          { mode, projectPath }
        );
        
        return report;
      } catch (e) {
        const errorMsg = `Failed to generate deviation report: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Clear error state.
     */
    clearError(): void {
      error.set(null);
    },

    /**
     * Initialize the compliance store by loading current status.
     */
    async initialize(): Promise<void> {
      await this.refreshComplianceStatus();
    },
  };
}

export const complianceStore = createComplianceStore();
