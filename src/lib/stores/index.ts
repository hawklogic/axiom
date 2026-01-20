// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Store exports.
 */

export { settingsStore, type Settings } from './settings';
export { editorStore, type OpenFile } from './editor';
export { gitStore, type RepoStatus, type StatusEntry } from './git';
export { toolchainStore, type DetectedToolchain } from './toolchain';
export { terminalStore, type TerminalSession } from './terminal';
export { workspace, hasWorkspace, type DirEntry, type TreeNode } from './workspace';
export { consoleStore, type LogEntry, type LogLevel } from './console';
export {
  armToolchainStore,
  type ArmToolchainSuite,
  type ArmMcuConfig,
  type ArmCompileRequest,
  type CompileResult,
  type ArmLinkRequest,
  type LinkResult,
  type BinaryOutputConfig,
  type BinaryResult,
  type SizeStats,
  type MakefileInfo,
  type MakeResult,
  type Diagnostic,
  DEFAULT_MCU_CONFIGS,
} from './armToolchain';
export {
  complianceStore,
  type ComplianceMode,
  type TraceabilityLink,
  type TraceabilityMatrix,
  type UntraceableFunction,
  type FileCoverage,
  type BranchInfo,
  type CoverageReport,
  type ComplianceRequirements,
  type ComplianceStatus,
  type DeviationReport,
} from './compliance';
