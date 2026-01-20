# Implementation Tasks: ARM Toolchain Integration

## CRITICAL: Testing Requirements

**Before moving to the next task group:**
1. All unit tests for the current module MUST pass
2. All integration tests for the current module MUST pass
3. Run `cargo test` to verify no regressions in existing code
4. Run `cargo clippy` to ensure no new warnings
5. Only proceed when ALL tests are green

**No breaking changes allowed** - existing functionality must continue to work.

---

## Task 1: Create Test Reference Project Structure
**Validates: All requirements (test foundation)**

- [ ] 1.1 Create `tests/fixtures/arm-reference-project/` directory structure
- [ ] 1.2 Create `Core/Inc/main.h` with basic STM32 definitions
- [ ] 1.3 Create `Core/Inc/config.h` with project configuration
- [ ] 1.4 Create `Core/Src/main.c` with REQ annotations for traceability testing
- [ ] 1.5 Create `Drivers/gpio.c` and `Drivers/gpio.h` with GPIO driver implementation
- [ ] 1.6 Create `Drivers/uart.c` and `Drivers/uart.h` with UART driver stubs
- [ ] 1.7 Create `Drivers/timer.c` and `Drivers/timer.h` with timer driver stubs
- [ ] 1.8 Create `STM32F103C8_FLASH.ld` linker script (64KB flash, 20KB RAM)
- [ ] 1.9 Create `Makefile` with targets: all, clean, flash, debug, coverage, edge-*
- [ ] 1.10 Create `edge_cases/empty_file.c` (empty source)
- [ ] 1.11 Create `edge_cases/syntax_error.c` (intentional errors)
- [ ] 1.12 Create `edge_cases/linker_overflow.c` (128KB array for overflow test)
- [ ] 1.13 Create `edge_cases/inline_assembly.c` (ARM inline asm: MRS, MSR, CPSID, DSB)
- [ ] 1.14 Create `edge_cases/missing_include.c` (references nonexistent header)
- [ ] 1.15 Create `compliance/traced_module.c` with full REQ annotations
- [ ] 1.16 Create `compliance/untraced_module.c` without annotations
- [ ] 1.17 Create `compliance/requirements.csv` with requirement definitions
- [ ] 1.18 Create `Tests/test_gpio.c` with TEST: REQ-xxx annotations
- [ ] 1.19 Create `.axiom/toolchain.toml` project config file
- [ ] 1.20 **VERIFY**: All files created and Makefile syntax valid

---

## Task 2: Extend Toolchain Detection for STM32CubeIDE
**Validates: Requirements 1, 2, 11, 12**

### 2.1 Implementation
- [ ] 2.1.1 Add STM32CubeIDE macOS paths to `ARM_GCC_PATHS` in detection.rs
- [ ] 2.1.2 Add STM32CubeIDE Linux paths to detection.rs
- [ ] 2.1.3 Add STM32CubeIDE Windows paths to detection.rs
- [ ] 2.1.4 Implement glob pattern expansion for versioned plugin directories
- [ ] 2.1.5 Add `ToolchainSource` enum (Homebrew, Stm32CubeIde, SystemPath, Manual)
- [ ] 2.1.6 Add `ToolchainCompleteness` enum (Complete, Incomplete { missing: Vec<String> })
- [ ] 2.1.7 Create `ArmToolchainSuite` struct with all tool paths (gcc, g++, as, ld, objcopy, objdump, size, gdb)
- [ ] 2.1.8 Implement `validate_toolchain_suite()` to check all tools exist
- [ ] 2.1.9 Implement `detect_source()` to identify toolchain origin from path
- [ ] 2.1.10 Update `detect_all()` to return `Vec<ArmToolchainSuite>`

### 2.2 Unit Tests
- [ ] 2.2.1 Test `parse_arm_gcc_version()` with standard ARM toolchain output
- [ ] 2.2.2 Test `parse_arm_gcc_version()` with STM32CubeIDE toolchain output
- [ ] 2.2.3 Test `detect_source()` returns Homebrew for /opt/homebrew paths
- [ ] 2.2.4 Test `detect_source()` returns Stm32CubeIde for STM32CubeIDE paths
- [ ] 2.2.5 Test `validate_toolchain_suite()` marks complete when all tools present
- [ ] 2.2.6 Test `validate_toolchain_suite()` marks incomplete with missing tools list
- [ ] 2.2.7 Test version comparison `is_version_compatible("14.3.1", "8.0.0")` returns true
- [ ] 2.2.8 Test version comparison `is_version_compatible("7.9.9", "8.0.0")` returns false

### 2.3 Integration Tests
- [ ] 2.3.1 Test `detect_all()` finds toolchain if arm-none-eabi-gcc installed
- [ ] 2.3.2 Test `detect_at_path()` returns None for nonexistent path
- [ ] 2.3.3 Test detected toolchain has valid version string
- [ ] 2.3.4 Test detected toolchain has existing gcc path

### 2.4 Property-Based Test
- [ ] 2.4.1 Write P1: For any returned toolchain, gcc path exists and version is non-empty

### 2.5 Verification Gate
- [ ] 2.5.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 2.5.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 2.5.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 3: Implement ARM MCU Configuration Module
**Validates: Requirements 3, 4**

### 3.1 Implementation
- [ ] 3.1.1 Create `crates/axiom-toolchain/src/arm_mcu.rs` module
- [ ] 3.1.2 Add `mod arm_mcu;` and `pub use arm_mcu::*;` to lib.rs
- [ ] 3.1.3 Create `FloatAbi` enum (Soft, SoftFp, Hard)
- [ ] 3.1.4 Create `ArmMcuConfig` struct (cpu, thumb, fpu, float_abi, defines)
- [ ] 3.1.5 Implement `ArmMcuConfig::compiler_flags()` method
- [ ] 3.1.6 Create `LinkerConfig` struct (script, generate_map, map_path, flags)
- [ ] 3.1.7 Implement `ArmMcuConfig::linker_flags(&LinkerConfig)` method
- [ ] 3.1.8 Add preset constructors: `cortex_m0()`, `cortex_m3()`, `cortex_m4()`, `cortex_m7()`
- [ ] 3.1.9 Implement `validate_linker_config()` to check script exists

### 3.2 Unit Tests
- [ ] 3.2.1 Test `compiler_flags()` includes `-mcpu=cortex-m3` for cortex_m3 config
- [ ] 3.2.2 Test `compiler_flags()` includes `-mthumb` when thumb=true
- [ ] 3.2.3 Test `compiler_flags()` includes `-mfpu=fpv5-d16` when fpu specified
- [ ] 3.2.4 Test `compiler_flags()` includes `-mfloat-abi=hard` for Hard ABI
- [ ] 3.2.5 Test `compiler_flags()` includes `-DSTM32H750xx` for defines
- [ ] 3.2.6 Test `linker_flags()` includes `-T<script>` for linker script
- [ ] 3.2.7 Test `linker_flags()` includes `-Wl,-Map=output.map` when generate_map=true
- [ ] 3.2.8 Test `linker_flags()` includes `-Wl,--gc-sections`
- [ ] 3.2.9 Test `validate_linker_config()` returns error for nonexistent script

### 3.3 Property-Based Test
- [ ] 3.3.1 Write P2: For any ArmMcuConfig, flags contain -mcpu with specified CPU

### 3.4 Verification Gate
- [ ] 3.4.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 3.4.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 3.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 4: Extend Compilation System for ARM
**Validates: Requirements 3, 6, 7**

### 4.1 Implementation
- [ ] 4.1.1 Create `ArmCompileRequest` struct in types.rs
- [ ] 4.1.2 Add `source`, `output`, `mcu: ArmMcuConfig`, `include_paths`, `optimization`, `debug` fields
- [ ] 4.1.3 Implement `ArmCompileRequest::new()` constructor
- [ ] 4.1.4 Implement `with_mcu()`, `with_include_path()`, `with_define()` builder methods
- [ ] 4.1.5 Implement `build_arm_compile_command()` in invocation.rs
- [ ] 4.1.6 Implement `compile_arm()` function that invokes arm-none-eabi-gcc
- [ ] 4.1.7 Ensure include paths are added in order with -I flags
- [ ] 4.1.8 Parse compiler errors/warnings from stderr

### 4.2 Unit Tests
- [ ] 4.2.1 Test `build_arm_compile_command()` includes -c flag
- [ ] 4.2.2 Test `build_arm_compile_command()` includes source and output paths
- [ ] 4.2.3 Test `build_arm_compile_command()` includes MCU flags from config
- [ ] 4.2.4 Test include paths appear in order specified
- [ ] 4.2.5 Test optimization level -O0 through -O3
- [ ] 4.2.6 Test debug flag -g3 when debug=true

### 4.3 Integration Tests
- [ ] 4.3.1 Test compile simple C file from reference project (gpio.c)
- [ ] 4.3.2 Test compile with FPU configuration (cortex-m7 + fpv5-d16)
- [ ] 4.3.3 Test compile syntax_error.c returns non-zero exit code
- [ ] 4.3.4 Test compile missing_include.c returns error with "fatal error"
- [ ] 4.3.5 Test compile inline_assembly.c succeeds for ARM target

### 4.4 Property-Based Test
- [ ] 4.4.1 Write P3: Include paths in flags match order in request

### 4.5 Verification Gate
- [ ] 4.5.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 4.5.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 4.5.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 5: Implement Linker Support
**Validates: Requirements 4, 5, 10**

### 5.1 Implementation
- [ ] 5.1.1 Create `ArmLinkRequest` struct (objects, output, linker: LinkerConfig, mcu)
- [ ] 5.1.2 Implement `build_arm_link_command()` in invocation.rs
- [ ] 5.1.3 Implement `link_arm()` function
- [ ] 5.1.4 Add `-nostartfiles` and `--specs=nano.specs` flags
- [ ] 5.1.5 Implement memory overflow error detection from linker stderr
- [ ] 5.1.6 Create `LinkResult` struct with exit_code, stdout, stderr, diagnostics

### 5.2 Unit Tests
- [ ] 5.2.1 Test `build_arm_link_command()` includes -T<script>
- [ ] 5.2.2 Test `build_arm_link_command()` includes all object files
- [ ] 5.2.3 Test `build_arm_link_command()` includes MCU flags
- [ ] 5.2.4 Test memory overflow parsing detects "will not fit" message
- [ ] 5.2.5 Test memory overflow parsing detects "region overflow" message

### 5.3 Integration Tests
- [ ] 5.3.1 Test link with valid linker script from reference project
- [ ] 5.3.2 Test link with missing linker script returns error
- [ ] 5.3.3 Test link linker_overflow.c detects memory overflow

### 5.4 Verification Gate
- [ ] 5.4.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 5.4.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 5.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 6: Implement Binary Generation Module
**Validates: Requirement 5**

### 6.1 Implementation
- [ ] 6.1.1 Create `crates/axiom-toolchain/src/binary_gen.rs` module
- [ ] 6.1.2 Add module to lib.rs exports
- [ ] 6.1.3 Create `BinaryOutputConfig` struct (hex, bin, size_report)
- [ ] 6.1.4 Create `BinaryResult` struct (hex_path, bin_path, size_stats)
- [ ] 6.1.5 Create `SizeStats` struct (text, data, bss, total)
- [ ] 6.1.6 Implement `build_objcopy_hex_command()` with -O ihex
- [ ] 6.1.7 Implement `build_objcopy_bin_command()` with -O binary
- [ ] 6.1.8 Implement `generate_hex()` function
- [ ] 6.1.9 Implement `generate_bin()` function
- [ ] 6.1.10 Implement `parse_size_output()` to extract text/data/bss
- [ ] 6.1.11 Implement `get_size_stats()` function

### 6.2 Unit Tests
- [ ] 6.2.1 Test `build_objcopy_hex_command()` includes -O ihex
- [ ] 6.2.2 Test `build_objcopy_bin_command()` includes -O binary
- [ ] 6.2.3 Test `parse_size_output()` extracts correct values from arm-none-eabi-size output
- [ ] 6.2.4 Test `parse_size_output()` handles malformed output gracefully

### 6.3 Integration Tests
- [ ] 6.3.1 Test generate HEX from valid ELF (requires building reference project first)
- [ ] 6.3.2 Test generate BIN from valid ELF
- [ ] 6.3.3 Test size stats extraction from valid ELF

### 6.4 Verification Gate
- [ ] 6.4.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 6.4.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 6.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 7: Implement Compiler Stage Visualizer
**Validates: Requirement 17**

### 7.1 Implementation
- [ ] 7.1.1 Create `crates/axiom-toolchain/src/visualizer.rs` module
- [ ] 7.1.2 Add module to lib.rs exports
- [ ] 7.1.3 Implement `build_preprocessor_flags()` returning -E flag
- [ ] 7.1.4 Implement `get_preprocessor_output()` function
- [ ] 7.1.5 Implement `build_assembly_flags()` returning -S flag
- [ ] 7.1.6 Implement `get_assembly_output()` function
- [ ] 7.1.7 Implement `build_disassembly_flags()` returning objdump -d
- [ ] 7.1.8 Implement `get_disassembly()` function
- [ ] 7.1.9 Implement `build_symbol_table_flags()` returning objdump -t
- [ ] 7.1.10 Implement `get_symbol_table()` function
- [ ] 7.1.11 Implement `build_section_headers_flags()` returning objdump -h
- [ ] 7.1.12 Implement `get_section_headers()` function

### 7.2 Unit Tests
- [ ] 7.2.1 Test `build_preprocessor_flags()` contains -E
- [ ] 7.2.2 Test `build_assembly_flags()` contains -S
- [ ] 7.2.3 Test `build_disassembly_flags()` contains -d
- [ ] 7.2.4 Test `build_symbol_table_flags()` contains -t
- [ ] 7.2.5 Test `build_section_headers_flags()` contains -h

### 7.3 Integration Tests
- [ ] 7.3.1 Test preprocessor output for gpio.c (if headers available)
- [ ] 7.3.2 Test assembly output for inline_assembly.c contains ARM instructions
- [ ] 7.3.3 Test disassembly of compiled object file

### 7.4 Verification Gate
- [ ] 7.4.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 7.4.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 7.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 8: Implement Makefile Support
**Validates: Requirement 8**

### 8.1 Implementation
- [ ] 8.1.1 Create `crates/axiom-toolchain/src/makefile.rs` module
- [ ] 8.1.2 Add module to lib.rs exports
- [ ] 8.1.3 Create `MakefileInfo` struct (path, targets)
- [ ] 8.1.4 Create `MakeResult` struct (exit_code, stdout, stderr)
- [ ] 8.1.5 Implement `detect_makefile()` to find Makefile in directory
- [ ] 8.1.6 Implement `parse_makefile_targets()` to extract .PHONY targets
- [ ] 8.1.7 Implement `run_make()` to execute make with target
- [ ] 8.1.8 Pass toolchain path as PREFIX environment variable

### 8.2 Unit Tests
- [ ] 8.2.1 Test `detect_makefile()` finds Makefile in directory
- [ ] 8.2.2 Test `detect_makefile()` returns None for empty directory
- [ ] 8.2.3 Test `parse_makefile_targets()` extracts all, clean, flash targets
- [ ] 8.2.4 Test `parse_makefile_targets()` handles .PHONY declaration

### 8.3 Integration Tests
- [ ] 8.3.1 Test detect Makefile in reference project
- [ ] 8.3.2 Test run `make clean` in reference project
- [ ] 8.3.3 Test run `make` with invalid target returns error

### 8.4 Verification Gate
- [ ] 8.4.1 Run `cargo test -p axiom-toolchain` - ALL MUST PASS
- [ ] 8.4.2 Run `cargo clippy -p axiom-toolchain` - NO WARNINGS
- [ ] 8.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 9: Extend Settings Schema for ARM Toolchain
**Validates: Requirements 13, 14, 15**

### 9.1 Implementation
- [ ] 9.1.1 Add `ToolchainConfig` struct to schema.rs with generic HashMap settings
- [ ] 9.1.2 Update `ToolchainSettings` to use `HashMap<String, ToolchainConfig>`
- [ ] 9.1.3 Create `ArmToolchainSettings` struct (mcu, fpu, float_abi, linker_script, include_paths, defines)
- [ ] 9.1.4 Add `ComplianceSettings` struct (do178c_enabled, do330_enabled, arp4754a_enabled, dal)
- [ ] 9.1.5 Implement `load_project_settings()` from .axiom/toolchain.toml
- [ ] 9.1.6 Implement `merge_settings()` with project overriding global
- [ ] 9.1.7 Ensure unknown toolchain types are preserved through serialization

### 9.2 Unit Tests
- [ ] 9.2.1 Test `ArmToolchainSettings` default values
- [ ] 9.2.2 Test settings serialization roundtrip (serialize then deserialize equals original)
- [ ] 9.2.3 Test unknown toolchain "riscv" preserved after roundtrip
- [ ] 9.2.4 Test `merge_settings()` project path overrides global path
- [ ] 9.2.5 Test `merge_settings()` project auto_detect overrides global

### 9.3 Integration Tests
- [ ] 9.3.1 Test load project settings from reference project .axiom/toolchain.toml
- [ ] 9.3.2 Test merge with global settings

### 9.4 Property-Based Test
- [ ] 9.4.1 Write P4: Any ToolchainSettings survives serialize/deserialize roundtrip unchanged

### 9.5 Verification Gate
- [ ] 9.5.1 Run `cargo test -p axiom-settings` - ALL MUST PASS
- [ ] 9.5.2 Run `cargo clippy -p axiom-settings` - NO WARNINGS
- [ ] 9.5.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 10: Create axiom-compliance Crate Structure
**Validates: Requirements 18-28 (foundation)**

### 10.1 Implementation
- [ ] 10.1.1 Create `crates/axiom-compliance/` directory
- [ ] 10.1.2 Create `Cargo.toml` with dependencies (serde, chrono, uuid, sha2, regex, csv, thiserror)
- [ ] 10.1.3 Create `src/lib.rs` with module declarations
- [ ] 10.1.4 Create `ComplianceMode` enum (Do178c, Do330, Arp4754a)
- [ ] 10.1.5 Create `src/modes.rs` with mode state management
- [ ] 10.1.6 Implement `ComplianceSystem` struct with mode enable/disable
- [ ] 10.1.7 Add axiom-compliance to workspace Cargo.toml

### 10.2 Unit Tests
- [ ] 10.2.1 Test `ComplianceMode` enum serialization
- [ ] 10.2.2 Test enable single mode
- [ ] 10.2.3 Test disable single mode
- [ ] 10.2.4 Test enable multiple modes simultaneously
- [ ] 10.2.5 Test `is_mode_enabled()` returns correct state

### 10.3 Verification Gate
- [ ] 10.3.1 Run `cargo test -p axiom-compliance` - ALL MUST PASS
- [ ] 10.3.2 Run `cargo clippy -p axiom-compliance` - NO WARNINGS
- [ ] 10.3.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 11: Implement Traceability System
**Validates: Requirement 18**

### 11.1 Implementation
- [ ] 11.1.1 Create `src/traceability.rs` module
- [ ] 11.1.2 Create `TraceabilityLink` struct (requirement_id, source_file, line_number, link_type)
- [ ] 11.1.3 Create `LinkType` enum (Implementation, Test, Derived)
- [ ] 11.1.4 Create `TraceabilityMatrix` struct
- [ ] 11.1.5 Implement `parse_requirement_annotations()` for `// REQ-xxx` pattern
- [ ] 11.1.6 Implement `parse_test_annotations()` for `// TEST: REQ-xxx` pattern
- [ ] 11.1.7 Implement `generate_traceability_matrix()` from source files
- [ ] 11.1.8 Implement `find_untraceable_functions()` for gap analysis
- [ ] 11.1.9 Implement `find_untested_requirements()` for gap analysis
- [ ] 11.1.10 Implement `export_matrix_csv()` for certification export

### 11.2 Unit Tests
- [ ] 11.2.1 Test `parse_requirement_annotations()` finds "REQ-001"
- [ ] 11.2.2 Test `parse_requirement_annotations()` finds "REQ-001.1" (sub-requirements)
- [ ] 11.2.3 Test `parse_test_annotations()` finds single requirement
- [ ] 11.2.4 Test `parse_test_annotations()` finds multiple requirements "REQ-001, REQ-002"
- [ ] 11.2.5 Test `find_untraceable_functions()` identifies functions without REQ
- [ ] 11.2.6 Test matrix contains all parsed requirements

### 11.3 Integration Tests
- [ ] 11.3.1 Test generate matrix from reference project traced_module.c
- [ ] 11.3.2 Test find untraceable code in untraced_module.c
- [ ] 11.3.3 Test export matrix to CSV

### 11.4 Property-Based Test
- [ ] 11.4.1 Write P6: All annotated requirements appear in generated matrix

### 11.5 Verification Gate
- [ ] 11.5.1 Run `cargo test -p axiom-compliance` - ALL MUST PASS
- [ ] 11.5.2 Run `cargo clippy -p axiom-compliance` - NO WARNINGS
- [ ] 11.5.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 12: Implement Coverage Analysis
**Validates: Requirement 19**

### 12.1 Implementation
- [ ] 12.1.1 Create `src/coverage.rs` module
- [ ] 12.1.2 Create `FileCoverage` struct (file, statement_coverage, branch_coverage, uncovered_lines)
- [ ] 12.1.3 Create `CoverageReport` struct (files, total_statement, total_branch)
- [ ] 12.1.4 Implement `build_coverage_flags()` returning --coverage, -fprofile-arcs, -ftest-coverage
- [ ] 12.1.5 Implement `parse_gcov_output()` to extract line execution counts
- [ ] 12.1.6 Implement `calculate_statement_coverage()` percentage
- [ ] 12.1.7 Implement `calculate_branch_coverage()` percentage
- [ ] 12.1.8 Implement `generate_coverage_report()` from gcov data

### 12.2 Unit Tests
- [ ] 12.2.1 Test `build_coverage_flags()` contains --coverage
- [ ] 12.2.2 Test `parse_gcov_output()` identifies executed lines (count > 0)
- [ ] 12.2.3 Test `parse_gcov_output()` identifies unexecuted lines (#####)
- [ ] 12.2.4 Test `calculate_statement_coverage()` returns 50% for 5/10 lines
- [ ] 12.2.5 Test `calculate_branch_coverage()` returns 75% for 3/4 branches

### 12.3 Verification Gate
- [ ] 12.3.1 Run `cargo test -p axiom-compliance` - ALL MUST PASS
- [ ] 12.3.2 Run `cargo clippy -p axiom-compliance` - NO WARNINGS
- [ ] 12.3.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 13: Implement Tool Qualification Logger
**Validates: Requirements 20, 22, 23**

### 13.1 Implementation
- [ ] 13.1.1 Create `src/tool_qualification.rs` module
- [ ] 13.1.2 Create `ToolUsageRecord` struct (tool, version, arguments, input_checksums, output_checksums, timestamp, exit_code, diagnostics)
- [ ] 13.1.3 Create `ToolQualificationLogger` struct with log_path
- [ ] 13.1.4 Implement `compute_sha256()` for file checksum
- [ ] 13.1.5 Implement `ToolQualificationLogger::new()` constructor
- [ ] 13.1.6 Implement `log()` method with append-only file writing
- [ ] 13.1.7 Implement `get_all_records()` to read log file
- [ ] 13.1.8 Implement JSON serialization for records

### 13.2 Unit Tests
- [ ] 13.2.1 Test `compute_sha256()` returns 64-char hex string
- [ ] 13.2.2 Test `compute_sha256()` same content produces same hash
- [ ] 13.2.3 Test `ToolUsageRecord` serialization roundtrip
- [ ] 13.2.4 Test `log()` appends to file without overwriting
- [ ] 13.2.5 Test `get_all_records()` returns all logged records

### 13.3 Integration Tests
- [ ] 13.3.1 Test log multiple tool invocations
- [ ] 13.3.2 Test records contain correct checksums for input files

### 13.4 Property-Based Test
- [ ] 13.4.1 Write P7: All logged invocations are retrievable with correct data

### 13.5 Verification Gate
- [ ] 13.5.1 Run `cargo test -p axiom-compliance` - ALL MUST PASS
- [ ] 13.5.2 Run `cargo clippy -p axiom-compliance` - NO WARNINGS
- [ ] 13.5.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 14: Implement Compliance Mode Management
**Validates: Requirement 27**

### 14.1 Implementation
- [ ] 14.1.1 Extend `ComplianceSystem` with data preservation on disable
- [ ] 14.1.2 Implement `disable_mode()` that preserves collected data
- [ ] 14.1.3 Implement `DeviationReport` struct for re-enablement analysis
- [ ] 14.1.4 Implement deviation detection (new code, modified code, broken links)
- [ ] 14.1.5 Implement `generate_deviation_report()` on mode re-enable
- [ ] 14.1.6 Implement union enforcement when multiple modes active

### 14.2 Unit Tests
- [ ] 14.2.1 Test disable mode preserves data
- [ ] 14.2.2 Test re-enable mode triggers deviation analysis
- [ ] 14.2.3 Test multiple modes can be active simultaneously
- [ ] 14.2.4 Test union enforcement applies strictest requirements

### 14.3 Property-Based Test
- [ ] 14.3.1 Write P8: Data survives mode disable/re-enable cycle

### 14.4 Verification Gate
- [ ] 14.4.1 Run `cargo test -p axiom-compliance` - ALL MUST PASS
- [ ] 14.4.2 Run `cargo clippy -p axiom-compliance` - NO WARNINGS
- [ ] 14.4.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 15: Add Tauri Commands for ARM Toolchain
**Validates: Requirement 9**

### 15.1 Implementation
- [ ] 15.1.1 Create `src-tauri/src/commands/arm_toolchain.rs` module
- [ ] 15.1.2 Implement `detect_arm_toolchains` command
- [ ] 15.1.3 Implement `validate_toolchain_path` command
- [ ] 15.1.4 Implement `compile_arm` command
- [ ] 15.1.5 Implement `link_arm` command
- [ ] 15.1.6 Implement `generate_binary` command
- [ ] 15.1.7 Implement `get_preprocessor_output` command
- [ ] 15.1.8 Implement `get_assembly_output` command
- [ ] 15.1.9 Implement `get_disassembly` command
- [ ] 15.1.10 Implement `detect_makefile` command
- [ ] 15.1.11 Implement `run_make` command
- [ ] 15.1.12 Register all commands in main.rs

### 15.2 Unit Tests
- [ ] 15.2.1 Test command registration compiles without errors

### 15.3 Verification Gate
- [ ] 15.3.1 Run `cargo build -p axiom` - MUST COMPILE
- [ ] 15.3.2 Run `cargo clippy -p axiom` - NO WARNINGS
- [ ] 15.3.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 16: Add Tauri Commands for Compliance
**Validates: Requirements 18-27**

### 16.1 Implementation
- [ ] 16.1.1 Create `src-tauri/src/commands/compliance.rs` module
- [ ] 16.1.2 Implement `enable_compliance_mode` command
- [ ] 16.1.3 Implement `disable_compliance_mode` command
- [ ] 16.1.4 Implement `get_traceability_matrix` command
- [ ] 16.1.5 Implement `get_coverage_report` command
- [ ] 16.1.6 Implement `get_compliance_status` command
- [ ] 16.1.7 Register all commands in main.rs

### 16.2 Verification Gate
- [ ] 16.2.1 Run `cargo build -p axiom` - MUST COMPILE
- [ ] 16.2.2 Run `cargo clippy -p axiom` - NO WARNINGS
- [ ] 16.2.3 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 17: Create Frontend ARM Toolchain Store
**Validates: Requirement 16**

### 17.1 Implementation
- [ ] 17.1.1 Create `src/lib/stores/armToolchain.ts`
- [ ] 17.1.2 Define TypeScript interfaces for ArmToolchainSuite, ArmMcuConfig, etc.
- [ ] 17.1.3 Create writable store for detected toolchains
- [ ] 17.1.4 Create writable store for selected toolchain
- [ ] 17.1.5 Create writable store for MCU configuration
- [ ] 17.1.6 Implement `detectToolchains()` action calling Tauri command
- [ ] 17.1.7 Implement `selectToolchain()` action
- [ ] 17.1.8 Implement `compile()` action
- [ ] 17.1.9 Export store from `src/lib/stores/index.ts`

### 17.2 Verification Gate
- [ ] 17.2.1 Run `npm run check` - NO TYPE ERRORS
- [ ] 17.2.2 Run `npm run lint` - NO LINT ERRORS

---

## Task 18: Create Frontend Compliance Store
**Validates: Requirements 18-27**

### 18.1 Implementation
- [ ] 18.1.1 Create `src/lib/stores/compliance.ts`
- [ ] 18.1.2 Define TypeScript interfaces for ComplianceMode, TraceabilityMatrix, CoverageReport
- [ ] 18.1.3 Create writable store for compliance mode states
- [ ] 18.1.4 Create writable store for traceability matrix
- [ ] 18.1.5 Create writable store for coverage report
- [ ] 18.1.6 Implement `toggleComplianceMode()` action
- [ ] 18.1.7 Implement `refreshTraceability()` action
- [ ] 18.1.8 Implement `refreshCoverage()` action
- [ ] 18.1.9 Export store from `src/lib/stores/index.ts`

### 18.2 Verification Gate
- [ ] 18.2.1 Run `npm run check` - NO TYPE ERRORS
- [ ] 18.2.2 Run `npm run lint` - NO LINT ERRORS

---

## Task 19: Implement Error Handling
**Validates: Requirement 10**

### 19.1 Implementation
- [ ] 19.1.1 Create `ArmToolchainError` enum in axiom-toolchain with thiserror
- [ ] 19.1.2 Add `NotFound` variant with installation suggestions
- [ ] 19.1.3 Add `Incomplete` variant with missing tools list
- [ ] 19.1.4 Add `VersionTooOld` variant with version info
- [ ] 19.1.5 Add `LinkerScriptNotFound` variant
- [ ] 19.1.6 Add `MemoryOverflow` variant with region details
- [ ] 19.1.7 Add `CompilationFailed` variant with diagnostics
- [ ] 19.1.8 Create `ComplianceError` enum in axiom-compliance
- [ ] 19.1.9 Implement platform-specific installation suggestions

### 19.2 Unit Tests
- [ ] 19.2.1 Test error Display implementations produce helpful messages
- [ ] 19.2.2 Test macOS suggestion mentions Homebrew
- [ ] 19.2.3 Test Linux suggestion mentions apt/dnf
- [ ] 19.2.4 Test Windows suggestion mentions ARM Developer download

### 19.3 Verification Gate
- [ ] 19.3.1 Run `cargo test` (full workspace) - ALL MUST PASS
- [ ] 19.3.2 Run `cargo clippy` (full workspace) - NO WARNINGS

---

## Task 20: Write Property-Based Tests
**Validates: All requirements (verification)**

### 20.1 Implementation
- [ ] 20.1.1 Add proptest dependency to axiom-toolchain Cargo.toml
- [ ] 20.1.2 Add proptest dependency to axiom-compliance Cargo.toml
- [ ] 20.1.3 Create arbitrary generator for ArmMcuConfig
- [ ] 20.1.4 Create arbitrary generator for ToolchainSettings
- [ ] 20.1.5 Create arbitrary generator for TraceabilityLink
- [ ] 20.1.6 Write P5: Version parsing extracts valid semver components

### 20.2 Verification Gate
- [ ] 20.2.1 Run all property tests - ALL MUST PASS
- [ ] 20.2.2 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 21: Write Edge Case Integration Tests
**Validates: All requirements (robustness)**

### 21.1 Implementation
- [ ] 21.1.1 Write `test_empty_source_file` - compiles empty file
- [ ] 21.1.2 Write `test_unicode_in_comments` - handles unicode
- [ ] 21.1.3 Write `test_very_long_lines` - handles long lines
- [ ] 21.1.4 Write `test_deeply_nested_includes` - handles include depth
- [ ] 21.1.5 Write `test_inline_assembly` - compiles ARM asm
- [ ] 21.1.6 Write `test_preprocessor_heavy_macros` - handles macros
- [ ] 21.1.7 Write `test_invalid_mcu_config` - rejects invalid CPU
- [ ] 21.1.8 Write `test_circular_includes` - handles circular deps

### 21.2 Verification Gate
- [ ] 21.2.1 Run all edge case tests - ALL MUST PASS
- [ ] 21.2.2 Run `cargo test` (full workspace) - NO REGRESSIONS

---

## Task 22: Documentation and Final Verification
**Validates: All requirements (completeness)**

### 22.1 Implementation
- [ ] 22.1.1 Add rustdoc comments to all public APIs in axiom-toolchain
- [ ] 22.1.2 Add rustdoc comments to all public APIs in axiom-compliance
- [ ] 22.1.3 Update README.md with ARM toolchain usage examples
- [ ] 22.1.4 Add example .axiom/toolchain.toml to docs

### 22.2 Final Verification Gate
- [ ] 22.2.1 Run `cargo fmt --all` - format all code
- [ ] 22.2.2 Run `cargo clippy --all` - NO WARNINGS
- [ ] 22.2.3 Run `cargo test --all` - ALL TESTS PASS
- [ ] 22.2.4 Run `cargo doc --no-deps` - documentation builds
- [ ] 22.2.5 Run `npm run check` - frontend type checks pass
- [ ] 22.2.6 Run `npm run lint` - frontend lint passes
- [ ] 22.2.7 Verify reference project compiles with `make` (if toolchain available)

---

## Summary

**Total Tasks**: 22 task groups
**Total Subtasks**: ~180 items
**Property-Based Tests**: 8 (P1-P8)
**Verification Gates**: 22 (one per task group)

**Critical Rule**: Do NOT proceed to the next task group until ALL tests in the current group pass and `cargo test` shows no regressions.
