# Implementation Tasks: ARM Toolchain UI

## Testing Framework

All UI components will be tested using:
- **Vitest** for unit tests
- **@testing-library/svelte** for component tests
- **fast-check** for property-based tests

---

## Task 1: Create ARM Toolchain Panel Container
**Validates: Requirement 16**

- [ ] 1. Create ARM Toolchain Panel Container
  - [x] 1.1 Create `src/lib/components/arm/` directory structure
  - [x] 1.2 Create `ArmToolchainPanel.svelte` with tab navigation
  - [x] 1.3 Implement tab state management (settings, build, visualizer, compliance, memory)
  - [x] 1.4 Add tab icons and labels
  - [x] 1.5 Implement keyboard navigation (Tab/Shift+Tab)
  - [x] 1.6 Add loading overlay for async operations
  - [x] 1.7 Style tabs to match Axiom design language
  - [x] 1.8 Write unit test: tabs render correctly
  - [x] 1.9 Write unit test: tab switching updates active tab

---

## Task 2: Integrate ARM Panel into Sidebar
**Validates: Requirement 16**

- [ ] 2. Integrate ARM Panel into Sidebar
  - [x] 2.1 Add ARM/Embedded icon to Sidebar.svelte
  - [x] 2.2 Add 'arm' to activePanel options in +page.svelte
  - [x] 2.3 Render ArmToolchainPanel when activePanel === 'arm'
  - [x] 2.4 Add build progress indicator to sidebar icon
  - [x] 2.5 Write unit test: ARM icon appears in sidebar
  - [x] 2.6 Write unit test: clicking ARM icon shows ARM panel


---

## Task 3: Implement Toolchain Settings Panel - Detection Display
**Validates: Requirement 1**

- [x] 3. Implement Toolchain Settings Panel - Detection Display
  - [x] 3.1 Create `ToolchainSettings.svelte` component
  - [x] 3.2 Subscribe to `armToolchainStore.toolchains`
  - [x] 3.3 Display toolchain list with version, source, and completeness
  - [x] 3.4 Show missing tools for incomplete toolchains
  - [x] 3.5 Implement "Refresh" button calling `detectToolchains()`
  - [x] 3.6 Show platform-specific installation instructions when no toolchains found
  - [x] 3.7 Style toolchain cards with selection indicator
  - [x] 3.8 Write unit test: toolchains render with correct data
  - [x] 3.9 Write unit test: refresh button triggers detection
  - [x] 3.10 Write unit test: empty state shows installation instructions

---

## Task 4: Implement Toolchain Settings Panel - Selection and Configuration
**Validates: Requirement 2**

- [ ] 4. Implement Toolchain Settings Panel - Selection and Configuration
  - [x] 4.1 Implement toolchain selection on card click
  - [x] 4.2 Display all tool paths for selected toolchain
  - [x] 4.3 Add "Add Custom Path" button and input field
  - [x] 4.4 Implement custom path validation with feedback
  - [x] 4.5 Add settings scope toggle (Project/Global)
  - [x] 4.6 Implement "Save" button to persist settings
  - [x] 4.7 Show indicator for project vs global settings
  - [x] 4.8 Write unit test: selection updates store
  - [x] 4.9 Write unit test: custom path validation works
  - [x] 4.10 Write property test P1: settings persist across reload

---

## Task 5: Implement Toolchain Settings Panel - MCU Configuration
**Validates: Requirement 3**

- [ ] 5. Implement Toolchain Settings Panel - MCU Configuration
  - [x] 5.1 Add CPU preset dropdown (cortex-m0, m3, m4, m7)
  - [-] 5.2 Implement auto-populate of FPU and float ABI on preset selection
  - [ ] 5.3 Add FPU type dropdown
  - [ ] 5.4 Add float ABI dropdown (soft, softfp, hard)
  - [ ] 5.5 Add Thumb mode checkbox
  - [ ] 5.6 Implement preprocessor defines list with add/remove
  - [ ] 5.7 Implement include paths list with add/remove/reorder
  - [ ] 5.8 Add linker script file picker (filtered to .ld)
  - [ ] 5.9 Write unit test: preset selection auto-populates fields
  - [ ] 5.10 Write unit test: defines can be added and removed
  - [ ] 5.11 Write property test P2: preset selection always sets valid FPU/ABI combo

---

## Task 6: Implement Build Panel - Build Operations
**Validates: Requirement 4**

- [ ] 6. Implement Build Panel - Build Operations
  - [ ] 6.1 Create `BuildPanel.svelte` component
  - [ ] 6.2 Add Compile, Link, Build All, and Clean buttons
  - [ ] 6.3 Implement compile operation calling `armToolchainStore.compile()`
  - [ ] 6.4 Implement link operation calling `armToolchainStore.link()`
  - [ ] 6.5 Implement Build All (compile → link → binary generation)
  - [ ] 6.6 Implement Clean operation
  - [ ] 6.7 Disable buttons during build operations
  - [ ] 6.8 Re-enable buttons after build completes
  - [ ] 6.9 Write unit test: buttons render correctly
  - [ ] 6.10 Write unit test: buttons disabled during build


---

## Task 7: Implement Build Panel - Output Display
**Validates: Requirement 5**

- [ ] 7. Implement Build Panel - Output Display
  - [ ] 7.1 Create `BuildOutput.svelte` component
  - [ ] 7.2 Implement real-time output streaming display
  - [ ] 7.3 Add syntax highlighting for compiler messages
  - [ ] 7.4 Highlight error lines in red
  - [ ] 7.5 Highlight warning lines in yellow
  - [ ] 7.6 Parse error messages for file/line/column
  - [ ] 7.7 Implement click-to-navigate on error messages
  - [ ] 7.8 Show success message with build time on completion
  - [ ] 7.9 Show error count and first error on failure
  - [ ] 7.10 Add "Clear" button to clear output
  - [ ] 7.11 Write unit test: output lines render with correct styling
  - [ ] 7.12 Write unit test: error click triggers navigation
  - [ ] 7.13 Write property test P3: error navigation matches error location

---

## Task 8: Implement Build Panel - Binary Output
**Validates: Requirement 6**

- [ ] 8. Implement Build Panel - Binary Output
  - [ ] 8.1 Display generated file paths (ELF, HEX, BIN)
  - [ ] 8.2 Display memory usage statistics (text, data, bss, total)
  - [ ] 8.3 Create `MemoryBar.svelte` component for usage visualization
  - [ ] 8.4 Implement memory bar with percentage fill
  - [ ] 8.5 Highlight bar in yellow when usage > 80%
  - [ ] 8.6 Highlight bar in red when usage > 95%
  - [ ] 8.7 Implement click-to-copy for file paths
  - [ ] 8.8 Write unit test: memory bars render with correct percentages
  - [ ] 8.9 Write unit test: warning/critical thresholds apply correct colors
  - [ ] 8.10 Write property test P4: displayed percentage matches calculated ratio

---

## Task 9: Implement Build Panel - Makefile Support
**Validates: Requirement 7**

- [ ] 9. Implement Build Panel - Makefile Support
  - [ ] 9.1 Detect Makefile on panel load using `detectMakefile()`
  - [ ] 9.2 Display available make targets as buttons
  - [ ] 9.3 Show common targets (all, clean, flash, debug)
  - [ ] 9.4 Implement make target execution on button click
  - [ ] 9.5 Stream make output to BuildOutput component
  - [ ] 9.6 Display exit code and error on make failure
  - [ ] 9.7 Write unit test: make targets render when Makefile detected
  - [ ] 9.8 Write unit test: make execution streams output

---

## Task 10: Implement Compiler Visualizer - Stage Selection
**Validates: Requirement 8**

- [ ] 10. Implement Compiler Visualizer - Stage Selection
  - [ ] 10.1 Create `CompilerVisualizer.svelte` component
  - [ ] 10.2 Add source file selector dropdown
  - [ ] 10.3 Add tabs for Preprocessor, Assembly, Disassembly, Symbols, Sections
  - [ ] 10.4 Implement tab switching with content loading
  - [ ] 10.5 Call appropriate store method for each tab
  - [ ] 10.6 Add "Refresh" button to regenerate current view
  - [ ] 10.7 Show loading state during content generation
  - [ ] 10.8 Write unit test: tabs render correctly
  - [ ] 10.9 Write unit test: tab switch loads correct content


---

## Task 11: Implement Compiler Visualizer - Output Display
**Validates: Requirement 9**

- [ ] 11. Implement Compiler Visualizer - Output Display
  - [ ] 11.1 Implement C/C++ syntax highlighting for preprocessor output
  - [ ] 11.2 Implement ARM assembly syntax highlighting
  - [ ] 11.3 Format disassembly with addresses, opcodes, and mnemonics
  - [ ] 11.4 Format symbol table with name, type, size, section columns
  - [ ] 11.5 Format section headers with name, size, VMA, LMA, flags
  - [ ] 11.6 Add search input with match highlighting
  - [ ] 11.7 Show match count for search results
  - [ ] 11.8 Implement line numbers in output display
  - [ ] 11.9 Write unit test: syntax highlighting applies correct classes
  - [ ] 11.10 Write unit test: search highlights matching text

---

## Task 12: Implement Compiler Visualizer - Source Mapping
**Validates: Requirement 10**

- [ ] 12. Implement Compiler Visualizer - Source Mapping
  - [ ] 12.1 Parse source file/line annotations from assembly output
  - [ ] 12.2 Display source annotations as clickable links
  - [ ] 12.3 Implement click-to-navigate to source location
  - [ ] 12.4 Add instruction tooltips with descriptions (common ARM instructions)
  - [ ] 12.5 Interleave source lines in disassembly when debug info available
  - [ ] 12.6 Write unit test: source annotations are clickable
  - [ ] 12.7 Write unit test: navigation opens correct file and line

---

## Task 13: Implement Compliance Panel - Mode Management
**Validates: Requirement 11**

- [ ] 13. Implement Compliance Panel - Mode Management
  - [ ] 13.1 Create `CompliancePanel.svelte` component
  - [ ] 13.2 Add toggle switches for DO-178C, DO-330, ARP4754A
  - [ ] 13.3 Subscribe to `complianceStore.complianceModes`
  - [ ] 13.4 Implement mode toggle calling `toggleComplianceMode()`
  - [ ] 13.5 Display active requirements based on enabled modes
  - [ ] 13.6 Show deviation report modal when mode re-enabled
  - [ ] 13.7 Style active modes with visual indicator
  - [ ] 13.8 Write unit test: toggles reflect store state
  - [ ] 13.9 Write unit test: toggle calls store action
  - [ ] 13.10 Write property test P5: UI state matches backend state

---

## Task 14: Implement Compliance Panel - Traceability Matrix
**Validates: Requirement 12**

- [ ] 14. Implement Compliance Panel - Traceability Matrix
  - [ ] 14.1 Create `TraceabilityMatrix.svelte` component
  - [ ] 14.2 Display matrix with Requirement, Implementation, Test, Status columns
  - [ ] 14.3 Show link types (Implementation, Test, Derived)
  - [ ] 14.4 Highlight requirements with no implementation as gaps
  - [ ] 14.5 Highlight requirements with no test as untested
  - [ ] 14.6 Implement cell click to navigate to source location
  - [ ] 14.7 Add "Export" button for CSV export
  - [ ] 14.8 Show coverage summary (X/Y implemented, X/Y tested)
  - [ ] 14.9 Write unit test: matrix renders with correct data
  - [ ] 14.10 Write unit test: gaps are highlighted correctly
  - [ ] 14.11 Write property test P6: all annotated requirements appear in matrix


---

## Task 15: Implement Compliance Panel - Coverage Report
**Validates: Requirement 13**

- [ ] 15. Implement Compliance Panel - Coverage Report
  - [ ] 15.1 Create `CoverageReport.svelte` component
  - [ ] 15.2 Display overall coverage percentages (statement, branch, decision)
  - [ ] 15.3 Display MC/DC coverage if available
  - [ ] 15.4 Display per-file coverage in a table
  - [ ] 15.5 Implement file click to show uncovered lines
  - [ ] 15.6 Highlight metrics below threshold in red
  - [ ] 15.7 Add coverage threshold configuration
  - [ ] 15.8 Write unit test: coverage percentages render correctly
  - [ ] 15.9 Write unit test: below-threshold metrics are highlighted

---

## Task 16: Implement Memory Map Viewer - Layout Visualization
**Validates: Requirement 14**

- [ ] 16. Implement Memory Map Viewer - Layout Visualization
  - [ ] 16.1 Create `MemoryMapViewer.svelte` component
  - [ ] 16.2 Display memory regions (FLASH, RAM, etc.) with addresses
  - [ ] 16.3 Show usage bar for each region
  - [ ] 16.4 Display used/total bytes and percentage
  - [ ] 16.5 Implement hover to show section breakdown
  - [ ] 16.6 Display sections table (.text, .data, .bss, etc.)
  - [ ] 16.7 Implement section click to show symbols in that section
  - [ ] 16.8 Write unit test: regions render with correct data
  - [ ] 16.9 Write unit test: hover shows section breakdown

---

## Task 17: Implement Memory Map Viewer - Analysis
**Validates: Requirement 15**

- [ ] 17. Implement Memory Map Viewer - Analysis
  - [ ] 17.1 Display largest symbols sorted by size
  - [ ] 17.2 Create section size pie chart
  - [ ] 17.3 Add symbol search input
  - [ ] 17.4 Implement symbol click to navigate to definition
  - [ ] 17.5 Store previous build stats for delta comparison
  - [ ] 17.6 Display delta from previous build (+/- bytes)
  - [ ] 17.7 Add sort options (name, size, address)
  - [ ] 17.8 Write unit test: symbols sorted by size
  - [ ] 17.9 Write unit test: search filters symbols
  - [ ] 17.10 Write property test P7: delta calculation is accurate

---

## Task 18: Implement Status Bar Integration
**Validates: Requirement 17**

- [ ] 18. Implement Status Bar Integration
  - [ ] 18.1 Add ARM toolchain section to StatusBar.svelte
  - [ ] 18.2 Display selected toolchain version
  - [ ] 18.3 Display target MCU
  - [ ] 18.4 Display build progress during builds
  - [ ] 18.5 Display build success/failure status
  - [ ] 18.6 Implement click to open toolchain settings
  - [ ] 18.7 Write unit test: status bar shows toolchain info
  - [ ] 18.8 Write unit test: click opens settings panel


---

## Task 19: Implement Error Handling and User Feedback
**Validates: Requirement 18**

- [ ] 19. Implement Error Handling and User Feedback
  - [ ] 19.1 Create toast notification component
  - [ ] 19.2 Display error messages with descriptive text
  - [ ] 19.3 Show platform-specific installation suggestions
  - [ ] 19.4 Show expected path for missing linker scripts
  - [ ] 19.5 Highlight overflowing memory regions
  - [ ] 19.6 Provide actionable suggestions in error messages
  - [ ] 19.7 Show success notifications for completed operations
  - [ ] 19.8 Write unit test: errors display with correct styling
  - [ ] 19.9 Write unit test: installation suggestions are platform-specific

---

## Task 20: Implement Keyboard Shortcuts
**Validates: Requirement 19**

- [ ] 20. Implement Keyboard Shortcuts
  - [ ] 20.1 Add global keyboard event listener
  - [ ] 20.2 Implement Cmd/Ctrl+Shift+B for Build All
  - [ ] 20.3 Implement Cmd/Ctrl+Shift+C for Compile current file
  - [ ] 20.4 Implement Cmd/Ctrl+Shift+K for Clean
  - [ ] 20.5 Implement Cmd/Ctrl+. for toolchain settings
  - [ ] 20.6 Implement F5 for flash/debug
  - [ ] 20.7 Add keyboard shortcut hints to buttons
  - [ ] 20.8 Write unit test: shortcuts trigger correct actions

---

## Task 21: Implement Accessibility Features
**Validates: Requirement 20**

- [ ] 21. Implement Accessibility Features
  - [ ] 21.1 Verify color contrast meets WCAG AA
  - [ ] 21.2 Add keyboard navigation to all interactive elements
  - [ ] 21.3 Use both color and icons/text for status
  - [ ] 21.4 Make tooltips accessible via keyboard focus
  - [ ] 21.5 Add ARIA labels to progress indicators
  - [ ] 21.6 Add screen reader announcements for build progress
  - [ ] 21.7 Write accessibility test: keyboard navigation works
  - [ ] 21.8 Write accessibility test: screen reader compatibility

---

## Task 22: Write Integration Tests
**Validates: All requirements**

- [ ] 22. Write Integration Tests
  - [ ] 22.1 Test complete build workflow (compile → link → binary)
  - [ ] 22.2 Test settings persistence across panel switches
  - [ ] 22.3 Test error navigation from build output to editor
  - [ ] 22.4 Test compliance mode toggle workflow
  - [ ] 22.5 Test memory map updates after build
  - [ ] 22.6 Test Makefile detection and execution
