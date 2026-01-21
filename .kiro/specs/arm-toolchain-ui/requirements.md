# Requirements Document: ARM Toolchain UI

## Introduction

This feature implements the complete user interface for the ARM toolchain integration in Axiom IDE. The UI exposes all backend functionality including toolchain configuration, build operations, compiler visualization, compliance management, and memory analysis. The interface follows Axiom's design language and integrates seamlessly with the existing IDE layout.

**Context**: This UI builds upon the completed ARM toolchain backend (`.kiro/specs/arm-toolchain-integration/`) which provides:
- Toolchain detection and validation
- ARM compilation, linking, and binary generation
- Compiler stage visualization (preprocessor, assembly, disassembly)
- Makefile workflow support
- DO-178C/DO-330/ARP4754A compliance systems
- Tauri commands and Svelte stores

## Glossary

- **Toolchain_Settings_Panel**: UI component for configuring ARM toolchain paths, MCU settings, and compiler options
- **Build_Panel**: UI component for initiating and monitoring compilation, linking, and binary generation
- **Compiler_Visualizer**: UI component for viewing intermediate compiler outputs (preprocessor, assembly, disassembly, symbols)
- **Compliance_Panel**: UI component for managing DO-178C/DO-330/ARP4754A modes and viewing compliance artifacts
- **Memory_Map_Viewer**: UI component for visualizing linker memory layout and usage statistics
- **MCU_Preset**: Pre-configured settings for common ARM Cortex-M processors
- **Build_Output**: Real-time streaming output from compilation and linking operations
- **Traceability_Matrix**: Visual representation of requirement-to-code-to-test links
- **Coverage_Report**: Visual representation of code coverage metrics

## Requirements

### Requirement 1: Toolchain Settings Panel - Detection Display

**User Story:** As a developer, I want to see all detected ARM toolchains on my system, so that I can choose which toolchain to use for my project.

#### Acceptance Criteria

1. WHEN the Toolchain_Settings_Panel loads, THE Toolchain_Settings_Panel SHALL display a list of all detected ARM toolchains
2. WHEN displaying a toolchain, THE Toolchain_Settings_Panel SHALL show the toolchain version (e.g., "14.3.1")
3. WHEN displaying a toolchain, THE Toolchain_Settings_Panel SHALL show the installation source (Homebrew, STM32CubeIDE, System, Manual)
4. WHEN displaying a toolchain, THE Toolchain_Settings_Panel SHALL show completeness status (Complete or Incomplete with missing tools)
5. WHEN a toolchain is incomplete, THE Toolchain_Settings_Panel SHALL display which tools are missing
6. WHEN no toolchains are detected, THE Toolchain_Settings_Panel SHALL display installation instructions for the current platform
7. WHEN the user clicks "Refresh", THE Toolchain_Settings_Panel SHALL re-scan for toolchains

### Requirement 2: Toolchain Settings Panel - Selection and Configuration

**User Story:** As a developer, I want to select and configure my ARM toolchain, so that I can customize compilation settings for my target hardware.

#### Acceptance Criteria

1. WHEN the user selects a toolchain, THE Toolchain_Settings_Panel SHALL mark it as the active toolchain
2. WHEN a toolchain is selected, THE Toolchain_Settings_Panel SHALL display all tool paths (gcc, g++, as, ld, objcopy, objdump, size, gdb)
3. WHEN the user clicks "Add Custom Path", THE Toolchain_Settings_Panel SHALL allow entering a custom toolchain path
4. WHEN a custom path is entered, THE Toolchain_Settings_Panel SHALL validate the path and show validation results
5. WHEN the user modifies settings, THE Toolchain_Settings_Panel SHALL persist changes to project or global configuration
6. WHEN displaying settings, THE Toolchain_Settings_Panel SHALL indicate which settings are project-specific vs global

### Requirement 3: Toolchain Settings Panel - MCU Configuration

**User Story:** As an embedded developer, I want to configure MCU-specific settings through a visual interface, so that I don't need to remember compiler flag syntax.

#### Acceptance Criteria

1. WHEN the Toolchain_Settings_Panel displays MCU settings, THE Toolchain_Settings_Panel SHALL show a dropdown for CPU type (cortex-m0, cortex-m3, cortex-m4, cortex-m7)
2. WHEN the user selects a CPU preset, THE Toolchain_Settings_Panel SHALL auto-populate FPU and float ABI settings
3. WHEN the Toolchain_Settings_Panel displays MCU settings, THE Toolchain_Settings_Panel SHALL show FPU type dropdown (none, fpv4-sp-d16, fpv5-d16, etc.)
4. WHEN the Toolchain_Settings_Panel displays MCU settings, THE Toolchain_Settings_Panel SHALL show float ABI dropdown (soft, softfp, hard)
5. WHEN the user adds a preprocessor define, THE Toolchain_Settings_Panel SHALL add it to the defines list
6. WHEN the user removes a preprocessor define, THE Toolchain_Settings_Panel SHALL remove it from the defines list
7. WHEN the user configures include paths, THE Toolchain_Settings_Panel SHALL allow adding, removing, and reordering paths
8. WHEN the user selects a linker script, THE Toolchain_Settings_Panel SHALL show a file picker filtered to .ld files

### Requirement 4: Build Panel - Build Operations

**User Story:** As a developer, I want to compile, link, and generate binaries through a visual interface, so that I can build my ARM project without using the command line.

#### Acceptance Criteria

1. WHEN the Build_Panel loads, THE Build_Panel SHALL display buttons for Compile, Link, and Build All operations
2. WHEN the user clicks "Compile", THE Build_Panel SHALL compile the current file or selected files
3. WHEN the user clicks "Link", THE Build_Panel SHALL link all object files with the configured linker script
4. WHEN the user clicks "Build All", THE Build_Panel SHALL perform compile, link, and binary generation in sequence
5. WHEN a build operation starts, THE Build_Panel SHALL disable build buttons to prevent concurrent builds
6. WHEN a build operation completes, THE Build_Panel SHALL re-enable build buttons
7. WHEN the user clicks "Clean", THE Build_Panel SHALL remove all build artifacts

### Requirement 5: Build Panel - Output Display

**User Story:** As a developer, I want to see build output in real-time, so that I can monitor compilation progress and identify errors quickly.

#### Acceptance Criteria

1. WHEN a build operation runs, THE Build_Panel SHALL stream stdout and stderr in real-time
2. WHEN displaying build output, THE Build_Panel SHALL syntax-highlight compiler messages
3. WHEN displaying errors, THE Build_Panel SHALL highlight error lines in red
4. WHEN displaying warnings, THE Build_Panel SHALL highlight warning lines in yellow
5. WHEN the user clicks an error message, THE Build_Panel SHALL navigate to the error location in the editor
6. WHEN a build completes successfully, THE Build_Panel SHALL display a success message with build time
7. WHEN a build fails, THE Build_Panel SHALL display the error count and first error message

### Requirement 6: Build Panel - Binary Output

**User Story:** As an embedded developer, I want to see binary output information after a successful build, so that I can verify my firmware size and prepare for flashing.

#### Acceptance Criteria

1. WHEN binary generation completes, THE Build_Panel SHALL display paths to generated files (ELF, HEX, BIN)
2. WHEN binary generation completes, THE Build_Panel SHALL display memory usage statistics (text, data, bss, total)
3. WHEN displaying memory usage, THE Build_Panel SHALL show a visual bar chart of memory regions
4. WHEN memory usage exceeds 80% of a region, THE Build_Panel SHALL highlight that region in yellow
5. WHEN memory usage exceeds 95% of a region, THE Build_Panel SHALL highlight that region in red
6. WHEN the user clicks a binary file path, THE Build_Panel SHALL copy the path to clipboard

### Requirement 7: Build Panel - Makefile Support

**User Story:** As a developer with existing Makefile projects, I want to use make targets through the IDE, so that I can leverage my existing build configuration.

#### Acceptance Criteria

1. WHEN a Makefile is detected in the project, THE Build_Panel SHALL display available make targets
2. WHEN displaying make targets, THE Build_Panel SHALL show common targets (all, clean, flash, debug)
3. WHEN the user clicks a make target, THE Build_Panel SHALL execute make with that target
4. WHEN make runs, THE Build_Panel SHALL stream make output in real-time
5. WHEN make fails, THE Build_Panel SHALL display the exit code and error output

### Requirement 8: Compiler Visualizer - Stage Selection

**User Story:** As a developer debugging compilation issues, I want to view intermediate compiler outputs, so that I can understand how my code is being transformed.

#### Acceptance Criteria

1. WHEN the Compiler_Visualizer loads, THE Compiler_Visualizer SHALL display tabs for Preprocessor, Assembly, Disassembly, Symbols, and Sections
2. WHEN the user selects the Preprocessor tab, THE Compiler_Visualizer SHALL show preprocessed source code
3. WHEN the user selects the Assembly tab, THE Compiler_Visualizer SHALL show generated assembly code
4. WHEN the user selects the Disassembly tab, THE Compiler_Visualizer SHALL show disassembled object code
5. WHEN the user selects the Symbols tab, THE Compiler_Visualizer SHALL show the symbol table
6. WHEN the user selects the Sections tab, THE Compiler_Visualizer SHALL show section headers

### Requirement 9: Compiler Visualizer - Output Display

**User Story:** As a developer, I want syntax-highlighted compiler output with navigation features, so that I can efficiently analyze intermediate representations.

#### Acceptance Criteria

1. WHEN displaying preprocessor output, THE Compiler_Visualizer SHALL syntax-highlight C/C++ code
2. WHEN displaying assembly output, THE Compiler_Visualizer SHALL syntax-highlight ARM assembly
3. WHEN displaying disassembly, THE Compiler_Visualizer SHALL show addresses, opcodes, and mnemonics
4. WHEN displaying symbols, THE Compiler_Visualizer SHALL show symbol name, type, size, and section
5. WHEN displaying sections, THE Compiler_Visualizer SHALL show section name, size, VMA, LMA, and flags
6. WHEN the user searches in output, THE Compiler_Visualizer SHALL highlight matching text
7. WHEN the user clicks "Refresh", THE Compiler_Visualizer SHALL regenerate the current view

### Requirement 10: Compiler Visualizer - Source Mapping

**User Story:** As a developer, I want to see the relationship between source code and generated assembly, so that I can understand compiler optimizations.

#### Acceptance Criteria

1. WHEN displaying assembly with debug info, THE Compiler_Visualizer SHALL show source file and line annotations
2. WHEN the user clicks a source annotation, THE Compiler_Visualizer SHALL navigate to that line in the editor
3. WHEN the user hovers over an assembly instruction, THE Compiler_Visualizer SHALL show a tooltip with instruction description
4. WHEN displaying disassembly, THE Compiler_Visualizer SHALL interleave source lines when debug info is available

### Requirement 11: Compliance Panel - Mode Management

**User Story:** As an avionics developer, I want to enable and manage compliance modes, so that I can ensure my project meets certification requirements.

#### Acceptance Criteria

1. WHEN the Compliance_Panel loads, THE Compliance_Panel SHALL display toggles for DO-178C, DO-330, and ARP4754A modes
2. WHEN a mode is enabled, THE Compliance_Panel SHALL show the mode toggle as active
3. WHEN the user toggles a mode on, THE Compliance_Panel SHALL enable that compliance mode
4. WHEN the user toggles a mode off, THE Compliance_Panel SHALL disable that mode and preserve collected data
5. WHEN multiple modes are enabled, THE Compliance_Panel SHALL display the union of requirements
6. WHEN a mode is re-enabled, THE Compliance_Panel SHALL show a deviation report if changes occurred

### Requirement 12: Compliance Panel - Traceability Matrix

**User Story:** As an avionics developer, I want to view the traceability matrix, so that I can verify requirement coverage and identify gaps.

#### Acceptance Criteria

1. WHEN the user views traceability, THE Compliance_Panel SHALL display a matrix of requirements to source files
2. WHEN displaying the matrix, THE Compliance_Panel SHALL show link types (Implementation, Test, Derived)
3. WHEN a requirement has no implementation, THE Compliance_Panel SHALL highlight it as a gap
4. WHEN a requirement has no test, THE Compliance_Panel SHALL highlight it as untested
5. WHEN the user clicks a matrix cell, THE Compliance_Panel SHALL navigate to the linked source location
6. WHEN the user clicks "Export", THE Compliance_Panel SHALL export the matrix to CSV

### Requirement 13: Compliance Panel - Coverage Report

**User Story:** As an avionics developer, I want to view code coverage metrics, so that I can ensure adequate test coverage for certification.

#### Acceptance Criteria

1. WHEN the user views coverage, THE Compliance_Panel SHALL display overall coverage percentages
2. WHEN displaying coverage, THE Compliance_Panel SHALL show statement, branch, and decision coverage
3. WHEN displaying coverage, THE Compliance_Panel SHALL show MC/DC coverage if analyzed
4. WHEN displaying file coverage, THE Compliance_Panel SHALL show per-file coverage percentages
5. WHEN the user clicks a file, THE Compliance_Panel SHALL show uncovered lines highlighted
6. WHEN coverage is below threshold, THE Compliance_Panel SHALL highlight the metric in red

### Requirement 14: Memory Map Viewer - Layout Visualization

**User Story:** As an embedded developer, I want to visualize memory layout, so that I can understand how my code and data are placed in memory.

#### Acceptance Criteria

1. WHEN the Memory_Map_Viewer loads, THE Memory_Map_Viewer SHALL display memory regions (FLASH, RAM, etc.)
2. WHEN displaying a region, THE Memory_Map_Viewer SHALL show start address, size, and usage
3. WHEN displaying a region, THE Memory_Map_Viewer SHALL show a visual bar representing usage percentage
4. WHEN the user hovers over a region, THE Memory_Map_Viewer SHALL show detailed section breakdown
5. WHEN displaying sections, THE Memory_Map_Viewer SHALL show .text, .data, .bss, and other sections
6. WHEN the user clicks a section, THE Memory_Map_Viewer SHALL show symbols in that section

### Requirement 15: Memory Map Viewer - Analysis

**User Story:** As an embedded developer, I want to analyze memory usage trends, so that I can optimize my firmware size.

#### Acceptance Criteria

1. WHEN displaying memory analysis, THE Memory_Map_Viewer SHALL show largest symbols by size
2. WHEN displaying memory analysis, THE Memory_Map_Viewer SHALL show section size breakdown as a pie chart
3. WHEN the user searches for a symbol, THE Memory_Map_Viewer SHALL highlight matching symbols
4. WHEN the user clicks a symbol, THE Memory_Map_Viewer SHALL navigate to its definition in the editor
5. WHEN memory usage changes, THE Memory_Map_Viewer SHALL show delta from previous build

### Requirement 16: Sidebar Integration

**User Story:** As a developer, I want to access ARM toolchain features from the sidebar, so that I can quickly switch between IDE functions.

#### Acceptance Criteria

1. WHEN the sidebar displays, THE sidebar SHALL include an ARM/Embedded icon
2. WHEN the user clicks the ARM icon, THE sidebar SHALL show the ARM toolchain panel
3. WHEN the ARM panel displays, THE ARM panel SHALL show tabs for Settings, Build, Visualizer, Compliance, Memory
4. WHEN the user switches tabs, THE ARM panel SHALL display the corresponding panel content
5. WHEN a build is in progress, THE sidebar icon SHALL show a progress indicator

### Requirement 17: Status Bar Integration

**User Story:** As a developer, I want to see ARM toolchain status in the status bar, so that I can quickly check toolchain and build state.

#### Acceptance Criteria

1. WHEN an ARM toolchain is selected, THE status bar SHALL display the toolchain version
2. WHEN an ARM project is open, THE status bar SHALL display the target MCU
3. WHEN a build is in progress, THE status bar SHALL display build progress
4. WHEN a build completes, THE status bar SHALL display success/failure status
5. WHEN the user clicks the toolchain status, THE status bar SHALL open the toolchain settings

### Requirement 18: Error Handling and User Feedback

**User Story:** As a developer, I want clear error messages and feedback, so that I can quickly resolve issues.

#### Acceptance Criteria

1. WHEN an operation fails, THE UI SHALL display a descriptive error message
2. WHEN a toolchain is not found, THE UI SHALL suggest installation methods for the current platform
3. WHEN a linker script is missing, THE UI SHALL show the expected path
4. WHEN memory overflow occurs, THE UI SHALL highlight the overflowing region
5. WHEN displaying errors, THE UI SHALL provide actionable suggestions when possible
6. WHEN an operation succeeds, THE UI SHALL show a brief success notification

### Requirement 19: Keyboard Shortcuts

**User Story:** As a power user, I want keyboard shortcuts for common operations, so that I can work efficiently without using the mouse.

#### Acceptance Criteria

1. WHEN the user presses Cmd/Ctrl+Shift+B, THE IDE SHALL trigger Build All
2. WHEN the user presses Cmd/Ctrl+Shift+C, THE IDE SHALL trigger Compile current file
3. WHEN the user presses Cmd/Ctrl+Shift+K, THE IDE SHALL trigger Clean
4. WHEN the user presses Cmd/Ctrl+., THE IDE SHALL open toolchain settings
5. WHEN the user presses F5, THE IDE SHALL trigger flash/debug (if configured)

### Requirement 20: Accessibility

**User Story:** As a developer with accessibility needs, I want the UI to be accessible, so that I can use all features effectively.

#### Acceptance Criteria

1. WHEN displaying UI elements, THE UI SHALL use sufficient color contrast (WCAG AA)
2. WHEN displaying interactive elements, THE UI SHALL support keyboard navigation
3. WHEN displaying status information, THE UI SHALL use both color and icons/text
4. WHEN displaying tooltips, THE UI SHALL be accessible via keyboard focus
5. WHEN displaying progress, THE UI SHALL announce progress to screen readers
