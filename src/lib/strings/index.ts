// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Axiom UI Strings Catalog
 * 
 * All user-facing strings are centralized here.
 * Never hardcode strings in components.
 * 
 * Voice: Minimalist, dry, precise.
 * See docs/voice.md for guidelines.
 */

// =============================================================================
// Application
// =============================================================================

export const APP = {
  name: 'Axiom',
  tagline: 'Deterministic. Inspectable. Offline.',
  steward: 'by HawkLogic Systems',
  copyright: 'Copyright 2024 HawkLogic Systems',
  license: 'Apache-2.0',
} as const;

// =============================================================================
// Panel Titles
// =============================================================================

export const PANELS = {
  fileExplorer: 'Files',
  sourceControl: 'Source Control',
  astViewer: 'AST',
  assemblyView: 'Assembly',
  debugPanel: 'Debug',
  terminal: 'Terminal',
  settings: 'Settings',
  about: 'About',
} as const;

// =============================================================================
// Button Labels
// =============================================================================

export const BUTTONS = {
  save: 'Save',
  cancel: 'Cancel',
  delete: 'Delete',
  close: 'Close',
  open: 'Open',
  compile: 'Compile',
  run: 'Run',
  refresh: 'Refresh',
  stage: 'Stage',
  unstage: 'Unstage',
  commit: 'Commit',
  reset: 'Reset to Default',
} as const;

// =============================================================================
// Status Messages
// =============================================================================

export const STATUS = {
  ready: 'Ready',
  building: 'Building...',
  buildComplete: 'Build complete.',
  buildFailed: 'Build failed.',
  saving: 'Saving...',
  saved: 'Saved.',
  loading: 'Loading...',
  noFileOpen: 'No file open.',
  noToolchain: 'No toolchain detected.',
  settingsSaved: 'Settings saved.',
} as const;

// =============================================================================
// Error Messages
// =============================================================================

export const ERRORS = {
  fileNotFound: 'File not found.',
  parseError: 'Parse error.',
  compileError: 'Compilation failed.',
  toolchainError: 'Toolchain error.',
  gitError: 'Git operation failed.',
  unknownError: 'An error occurred.',
} as const;

// =============================================================================
// Empty States
// =============================================================================

export const EMPTY = {
  noFiles: 'No files.',
  noChanges: 'No changes.',
  noSymbols: 'No symbols.',
  noResults: 'No results.',
  noTerminal: 'No terminal.',
} as const;

// =============================================================================
// Dialog Text
// =============================================================================

export const DIALOGS = {
  unsavedChanges: 'You have unsaved changes.',
  discardChanges: 'Discard changes?',
  deleteFile: 'Delete this file?',
  deleteConfirm: 'This cannot be undone.',
  resetSettings: 'Reset all settings to default?',
} as const;

// =============================================================================
// Tooltips
// =============================================================================

export const TOOLTIPS = {
  newFile: 'New file',
  openFile: 'Open file',
  openFolder: 'Open folder',
  saveFile: 'Save file',
  closeFile: 'Close file',
  splitHorizontal: 'Split horizontal',
  splitVertical: 'Split vertical',
  toggleTheme: 'Toggle theme',
  openSettings: 'Open settings',
  openTerminal: 'Open terminal',
  refreshFiles: 'Refresh files',
} as const;

// =============================================================================
// Settings Labels
// =============================================================================

export const SETTINGS = {
  categories: {
    toolchains: 'Toolchains',
    build: 'Build',
    editor: 'Editor',
    assembly: 'Assembly',
    debug: 'Debugging',
    ui: 'Appearance',
    about: 'About',
  },
  toolchains: {
    clangPath: 'Clang path',
    gccPath: 'GCC path',
    armGccPath: 'ARM GCC path',
    autoDetect: 'Auto-detect toolchains',
  },
  editor: {
    fontSize: 'Font size',
    tabSize: 'Tab size',
    fontFamily: 'Font family',
    lineNumbers: 'Show line numbers',
    wordWrap: 'Word wrap',
  },
  ui: {
    theme: 'Theme',
    themeDark: 'Dark',
    themeLight: 'Light',
  },
} as const;

// =============================================================================
// About Panel
// =============================================================================

export const ABOUT = {
  title: 'About Axiom',
  version: 'Version',
  philosophy: 'Axiom reveals ground truth. No AI. No cloud. No telemetry.',
  viewLicense: 'View License',
  viewPhilosophy: 'View Philosophy',
} as const;

// =============================================================================
// Splash Screen
// =============================================================================

export const SPLASH = {
  loading: 'Initializing...',
} as const;
