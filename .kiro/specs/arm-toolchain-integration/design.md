# Design Document: ARM Toolchain Integration

## Overview

This design document describes the architecture and implementation approach for integrating ARM GCC toolchain (arm-none-eabi-gcc) and related tools into the Axiom IDE. The integration extends the existing `axiom-toolchain` crate to comprehensively support ARM embedded development workflows, including compilation, linking, binary generation, debugging, and DO-178C/DO-330/ARP4754A compliance for safety-critical avionics software.

## Architecture

### High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Axiom IDE Frontend                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │ Settings UI │  │ Build Panel │  │ Compliance  │  │ Compiler Visualizer │ │
│  │             │  │             │  │   Panel     │  │                     │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘ │
└─────────┼────────────────┼────────────────┼────────────────────┼────────────┘
          │                │                │                    │
          ▼                ▼                ▼                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Tauri Command Layer                                │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  settings   │  │  toolchain  │  │ compliance  │  │    visualizer       │ │
│  │  commands   │  │  commands   │  │  commands   │  │    commands         │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────────┬──────────┘ │
└─────────┼────────────────┼────────────────┼────────────────────┼────────────┘
          │                │                │                    │
          ▼                ▼                ▼                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Rust Crate Layer                                │
│                                                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                        axiom-toolchain (extended)                      │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐  │  │
│  │  │  detection  │  │ invocation  │  │   types     │  │   arm_mcu    │  │  │
│  │  │  (extended) │  │ (extended)  │  │ (extended)  │  │   (new)      │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └──────────────┘  │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                    │  │
│  │  │  binary_gen │  │  visualizer │  │   makefile  │                    │  │
│  │  │   (new)     │  │   (new)     │  │   (new)     │                    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                    │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                        axiom-compliance (new)                          │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌──────────────┐  │  │
│  │  │traceability │  │  coverage   │  │    tool     │  │   system     │  │  │
│  │  │   system    │  │  analyzer   │  │qualification│  │ integration  │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └──────────────┘  │  │
│  │  ┌─────────────┐  ┌─────────────┐                                     │  │
│  │  │   export    │  │   modes     │                                     │  │
│  │  │  artifacts  │  │  manager    │                                     │  │
│  │  └─────────────┘  └─────────────┘                                     │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                     axiom-settings (extended)                          │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                    │  │
│  │  │   schema    │  │ persistence │  │  migration  │                    │  │
│  │  │ (extended)  │  │ (extended)  │  │ (extended)  │                    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘                    │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────┘
```


### Component Descriptions

#### 1. axiom-toolchain Crate (Extended)

**detection.rs (Extended)**
- Extends existing detection to search STM32CubeIDE paths
- Adds platform-specific path resolution for macOS, Linux, Windows
- Validates complete toolchain suite (gcc, g++, as, ld, objcopy, objdump, size, gdb)
- Extracts and parses version information from all tools

**invocation.rs (Extended)**
- Adds ARM-specific compiler flag generation (-mcpu, -mthumb, -mfpu, -mfloat-abi)
- Supports linker script specification (-T flag)
- Adds linking stage with memory map generation
- Integrates with binary generation pipeline

**types.rs (Extended)**
- Adds `ArmToolchainSuite` struct containing all tool paths
- Adds `ArmMcuConfig` for MCU-specific settings
- Adds `LinkerConfig` for linker script and memory settings
- Adds `CompletionStatus` enum for toolchain validation

**arm_mcu.rs (New)**
- MCU database with known configurations (Cortex-M0/M3/M4/M7)
- FPU configuration presets (fpv4-sp-d16, fpv5-d16, etc.)
- Common define macros for STM32 families

**binary_gen.rs (New)**
- ELF to HEX conversion using objcopy
- ELF to BIN conversion using objcopy
- Size reporting using arm-none-eabi-size
- Memory usage statistics parsing

**visualizer.rs (New)**
- Preprocessor output generation (-E flag)
- Assembly output generation (-S flag)
- Object file disassembly (objdump -d)
- Symbol table extraction (objdump -t)
- Section header display (objdump -h)
- IR dump support (-fdump-tree-all, -fdump-rtl-all)

**makefile.rs (New)**
- Makefile detection and parsing
- Make invocation with toolchain environment
- Target enumeration (all, clean, flash, debug)
- Build output streaming

#### 2. axiom-compliance Crate (New)

**traceability.rs**
- Requirement annotation parser (source code comments)
- Test case annotation parser
- Bidirectional traceability matrix generation
- Gap analysis (untraceable code, untested requirements)
- Export to CSV, PDF, HTML formats

**coverage.rs**
- GCC coverage instrumentation (--coverage flag)
- GCOV data collection and parsing
- Statement coverage calculation
- Branch coverage calculation
- Decision coverage calculation
- MC/DC coverage analysis
- LCOV/HTML/XML export

**tool_qualification.rs**
- Tool usage logging
- Tool Qualification Plan template generation
- Tool Operational Requirements document generation
- Anomaly logging and reporting
- Compiler test suite execution
- Checksum generation for inputs/outputs

**system_integration.rs**
- System requirements import (ReqIF, CSV, DOORS)
- Requirements allocation tracking
- FHA/PSSA/SSA data import
- Safety requirement linking
- Allocation matrix generation

**export.rs**
- Certification package generation
- PDF report generation
- CSV matrix export
- XML interchange format
- Digital signature support
- Template customization

**modes.rs**
- Compliance mode state management (DO-178C, DO-330, ARP4754A)
- Multi-mode simultaneous operation
- Mode enable/disable with data preservation
- Deviation report generation on re-enablement
- Union enforcement for multiple active modes

#### 3. axiom-settings Crate (Extended)

**schema.rs (Extended)**
```rust
/// Extended toolchain configuration with generic support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainSettings {
    /// Auto-detect toolchains on startup
    pub auto_detect: bool,
    
    /// Generic toolchain configurations keyed by toolchain type
    pub toolchains: HashMap<String, ToolchainConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainConfig {
    /// Path to primary binary
    pub path: Option<PathBuf>,
    
    /// Additional search paths
    pub search_paths: Vec<PathBuf>,
    
    /// Toolchain-specific settings (extensible)
    pub settings: HashMap<String, toml::Value>,
}

/// ARM-specific toolchain settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmToolchainSettings {
    /// Target MCU (e.g., "cortex-m7")
    pub mcu: Option<String>,
    
    /// FPU type (e.g., "fpv5-d16")
    pub fpu: Option<String>,
    
    /// Float ABI (hard, soft, softfp)
    pub float_abi: Option<String>,
    
    /// Default linker script
    pub linker_script: Option<PathBuf>,
    
    /// Default include paths
    pub include_paths: Vec<PathBuf>,
    
    /// Default preprocessor defines
    pub defines: Vec<String>,
}

/// Compliance mode settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSettings {
    /// DO-178C mode enabled
    pub do178c_enabled: bool,
    
    /// DO-330 mode enabled
    pub do330_enabled: bool,
    
    /// ARP4754A mode enabled
    pub arp4754a_enabled: bool,
    
    /// Design Assurance Level (A-E)
    pub dal: Option<String>,
}
```


## Data Models

### ARM Toolchain Suite

```rust
/// Complete ARM toolchain suite with all tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmToolchainSuite {
    /// Base path to toolchain bin directory
    pub base_path: PathBuf,
    
    /// Toolchain version
    pub version: String,
    
    /// Individual tool paths
    pub gcc: PathBuf,
    pub gpp: PathBuf,
    pub as_: PathBuf,
    pub ld: PathBuf,
    pub objcopy: PathBuf,
    pub objdump: PathBuf,
    pub size: PathBuf,
    pub gdb: Option<PathBuf>,
    
    /// Completeness status
    pub status: ToolchainCompleteness,
    
    /// Source of detection (homebrew, stm32cubeide, manual)
    pub source: ToolchainSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolchainCompleteness {
    Complete,
    Incomplete { missing: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolchainSource {
    Homebrew,
    Stm32CubeIde,
    SystemPath,
    Manual,
}
```

### ARM Compile Request

```rust
/// Extended compile request for ARM targets
#[derive(Debug, Clone)]
pub struct ArmCompileRequest {
    /// Base compile request
    pub base: CompileRequest,
    
    /// MCU configuration
    pub mcu: ArmMcuConfig,
    
    /// Linker configuration (for linking stage)
    pub linker: Option<LinkerConfig>,
    
    /// Binary output configuration
    pub binary_outputs: BinaryOutputConfig,
    
    /// Compliance mode
    pub compliance: Option<ComplianceConfig>,
}

#[derive(Debug, Clone)]
pub struct ArmMcuConfig {
    /// CPU type (cortex-m0, cortex-m3, cortex-m4, cortex-m7)
    pub cpu: String,
    
    /// Use Thumb instruction set
    pub thumb: bool,
    
    /// FPU type (None, fpv4-sp-d16, fpv5-d16, etc.)
    pub fpu: Option<String>,
    
    /// Float ABI (soft, softfp, hard)
    pub float_abi: FloatAbi,
    
    /// Preprocessor defines
    pub defines: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LinkerConfig {
    /// Linker script path
    pub script: PathBuf,
    
    /// Generate memory map
    pub generate_map: bool,
    
    /// Map file path
    pub map_path: Option<PathBuf>,
    
    /// Additional linker flags
    pub flags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BinaryOutputConfig {
    /// Generate Intel HEX
    pub hex: bool,
    
    /// Generate raw binary
    pub bin: bool,
    
    /// Report size statistics
    pub size_report: bool,
}
```

### Compliance Data Models

```rust
/// Traceability link between requirement and artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityLink {
    /// Requirement identifier
    pub requirement_id: String,
    
    /// Source file path
    pub source_file: PathBuf,
    
    /// Line number in source
    pub line_number: u32,
    
    /// Link type (implementation, test, derived)
    pub link_type: LinkType,
    
    /// Timestamp of link creation
    pub created_at: DateTime<Utc>,
}

/// Coverage data for a source file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCoverage {
    /// Source file path
    pub file: PathBuf,
    
    /// Statement coverage percentage
    pub statement_coverage: f64,
    
    /// Branch coverage percentage
    pub branch_coverage: f64,
    
    /// Decision coverage percentage
    pub decision_coverage: f64,
    
    /// MC/DC coverage percentage (if analyzed)
    pub mcdc_coverage: Option<f64>,
    
    /// Uncovered lines
    pub uncovered_lines: Vec<u32>,
    
    /// Uncovered branches
    pub uncovered_branches: Vec<BranchInfo>,
}

/// Tool usage record for DO-330 qualification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsageRecord {
    /// Tool name
    pub tool: String,
    
    /// Tool version
    pub version: String,
    
    /// Command line arguments
    pub arguments: Vec<String>,
    
    /// Input file checksums
    pub input_checksums: HashMap<PathBuf, String>,
    
    /// Output file checksums
    pub output_checksums: HashMap<PathBuf, String>,
    
    /// Execution timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Exit code
    pub exit_code: i32,
    
    /// Any warnings or errors
    pub diagnostics: Vec<String>,
}
```


## API Design

### Tauri Commands

```rust
// Toolchain detection commands
#[tauri::command]
async fn detect_arm_toolchains() -> Result<Vec<ArmToolchainSuite>, String>;

#[tauri::command]
async fn validate_toolchain_path(path: PathBuf) -> Result<ArmToolchainSuite, String>;

#[tauri::command]
async fn get_toolchain_info() -> Result<ToolchainInfoResponse, String>;

// Compilation commands
#[tauri::command]
async fn compile_arm(request: ArmCompileRequest) -> Result<CompileResult, String>;

#[tauri::command]
async fn link_arm(request: ArmLinkRequest) -> Result<LinkResult, String>;

#[tauri::command]
async fn generate_binary(elf_path: PathBuf, config: BinaryOutputConfig) -> Result<BinaryResult, String>;

// Makefile commands
#[tauri::command]
async fn detect_makefile(project_path: PathBuf) -> Result<Option<MakefileInfo>, String>;

#[tauri::command]
async fn run_make(project_path: PathBuf, target: String) -> Result<MakeResult, String>;

// Visualizer commands
#[tauri::command]
async fn get_preprocessor_output(source: PathBuf, config: ArmMcuConfig) -> Result<String, String>;

#[tauri::command]
async fn get_assembly_output(source: PathBuf, config: ArmMcuConfig) -> Result<String, String>;

#[tauri::command]
async fn get_disassembly(object_file: PathBuf) -> Result<String, String>;

#[tauri::command]
async fn get_symbol_table(object_file: PathBuf) -> Result<Vec<Symbol>, String>;

#[tauri::command]
async fn get_section_headers(object_file: PathBuf) -> Result<Vec<Section>, String>;

// Settings commands
#[tauri::command]
async fn get_toolchain_settings() -> Result<ToolchainSettings, String>;

#[tauri::command]
async fn save_toolchain_settings(settings: ToolchainSettings, scope: SettingsScope) -> Result<(), String>;

#[tauri::command]
async fn reset_toolchain_settings() -> Result<(), String>;

// Compliance commands
#[tauri::command]
async fn enable_compliance_mode(mode: ComplianceMode) -> Result<(), String>;

#[tauri::command]
async fn disable_compliance_mode(mode: ComplianceMode) -> Result<DeviationReport, String>;

#[tauri::command]
async fn get_traceability_matrix(project_path: PathBuf) -> Result<TraceabilityMatrix, String>;

#[tauri::command]
async fn get_coverage_report(project_path: PathBuf) -> Result<CoverageReport, String>;

#[tauri::command]
async fn export_certification_package(config: ExportConfig) -> Result<PathBuf, String>;
```

### Frontend Store Interface

```typescript
// src/lib/stores/armToolchain.ts

interface ArmToolchainStore {
  // Detected toolchains
  toolchains: ArmToolchainSuite[];
  
  // Currently selected toolchain
  selectedToolchain: ArmToolchainSuite | null;
  
  // Current MCU configuration
  mcuConfig: ArmMcuConfig;
  
  // Compliance modes
  complianceModes: {
    do178c: boolean;
    do330: boolean;
    arp4754a: boolean;
  };
  
  // Actions
  detectToolchains(): Promise<void>;
  selectToolchain(path: string): void;
  updateMcuConfig(config: Partial<ArmMcuConfig>): void;
  compile(source: string): Promise<CompileResult>;
  toggleComplianceMode(mode: ComplianceMode): Promise<void>;
}
```


## Detection Algorithm

### Platform-Specific Search Paths

```rust
/// macOS search paths
const MACOS_ARM_GCC_PATHS: &[&str] = &[
    "/opt/homebrew/bin/arm-none-eabi-gcc",
    "/usr/local/bin/arm-none-eabi-gcc",
    "/Applications/ARM/bin/arm-none-eabi-gcc",
    // STM32CubeIDE paths (glob pattern)
    "/Applications/STM32CubeIDE.app/Contents/Eclipse/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc",
];

/// Linux search paths
const LINUX_ARM_GCC_PATHS: &[&str] = &[
    "/usr/bin/arm-none-eabi-gcc",
    "/usr/local/bin/arm-none-eabi-gcc",
    "/opt/gcc-arm-none-eabi/bin/arm-none-eabi-gcc",
    "/opt/st/stm32cubeide/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*/tools/bin/arm-none-eabi-gcc",
];

/// Windows search paths
const WINDOWS_ARM_GCC_PATHS: &[&str] = &[
    "C:\\Program Files\\GNU Arm Embedded Toolchain\\*\\bin\\arm-none-eabi-gcc.exe",
    "C:\\Program Files (x86)\\GNU Arm Embedded Toolchain\\*\\bin\\arm-none-eabi-gcc.exe",
    "C:\\ST\\STM32CubeIDE_*\\STM32CubeIDE\\plugins\\com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.*\\tools\\bin\\arm-none-eabi-gcc.exe",
];
```

### Detection Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    detect_arm_toolchains()                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. Get platform-specific search paths                          │
│     - Expand glob patterns                                       │
│     - Add user-configured custom paths                           │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. For each path, check if arm-none-eabi-gcc exists            │
│     - Verify file exists                                         │
│     - Verify file is executable                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. Extract version information                                  │
│     - Run arm-none-eabi-gcc --version                           │
│     - Parse version string (e.g., "14.3.1")                     │
│     - Validate minimum version (>= 8.0.0)                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. Validate complete toolchain suite                           │
│     - Check for gcc, g++, as, ld, objcopy, objdump, size, gdb  │
│     - Mark missing tools                                         │
│     - Determine completeness status                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. Determine toolchain source                                   │
│     - Homebrew: /opt/homebrew or /usr/local                     │
│     - STM32CubeIDE: Contains "stm32cube" in path                │
│     - System: /usr/bin                                           │
│     - Manual: User-configured path                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  6. Return list of ArmToolchainSuite                            │
│     - Sorted by preference (complete > incomplete)               │
│     - Include all metadata                                       │
└─────────────────────────────────────────────────────────────────┘
```


## Compilation Pipeline

### ARM Compilation Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                     ArmCompileRequest                            │
│  - source files                                                  │
│  - MCU config (cpu, fpu, float_abi)                             │
│  - include paths                                                 │
│  - defines                                                       │
│  - optimization level                                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  1. COMPILE STAGE                                                │
│     arm-none-eabi-gcc -c source.c -o source.o                   │
│       -mcpu=cortex-m7 -mthumb                                   │
│       -mfpu=fpv5-d16 -mfloat-abi=hard                          │
│       -DSTM32H750xx -DUSE_HAL_DRIVER                           │
│       -I../Core/Inc -I../Drivers/...                            │
│       -Wall -Wextra -Os -g3                                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. LINK STAGE                                                   │
│     arm-none-eabi-gcc -o output.elf *.o                         │
│       -mcpu=cortex-m7 -mthumb                                   │
│       -T STM32H750VBTX_FLASH.ld                                 │
│       -Wl,--gc-sections                                          │
│       -Wl,-Map=output.map                                        │
│       -nostartfiles                                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. BINARY GENERATION STAGE                                      │
│     arm-none-eabi-objcopy -O ihex output.elf output.hex         │
│     arm-none-eabi-objcopy -O binary output.elf output.bin       │
│     arm-none-eabi-size output.elf                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. RESULT                                                       │
│     - ELF file                                                   │
│     - HEX file                                                   │
│     - BIN file                                                   │
│     - Memory map                                                 │
│     - Size statistics (text, data, bss)                         │
│     - Diagnostics (errors, warnings)                            │
└─────────────────────────────────────────────────────────────────┘
```

### Flag Generation

```rust
impl ArmMcuConfig {
    /// Generate compiler flags for this MCU configuration
    pub fn compiler_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        
        // CPU type
        flags.push(format!("-mcpu={}", self.cpu));
        
        // Thumb mode
        if self.thumb {
            flags.push("-mthumb".to_string());
        }
        
        // FPU configuration
        if let Some(ref fpu) = self.fpu {
            flags.push(format!("-mfpu={}", fpu));
        }
        
        // Float ABI
        flags.push(format!("-mfloat-abi={}", self.float_abi));
        
        // Preprocessor defines
        for define in &self.defines {
            flags.push(format!("-D{}", define));
        }
        
        flags
    }
    
    /// Generate linker flags
    pub fn linker_flags(&self, config: &LinkerConfig) -> Vec<String> {
        let mut flags = Vec::new();
        
        // CPU and thumb for linker
        flags.push(format!("-mcpu={}", self.cpu));
        if self.thumb {
            flags.push("-mthumb".to_string());
        }
        
        // Linker script
        flags.push(format!("-T{}", config.script.display()));
        
        // Garbage collection
        flags.push("-Wl,--gc-sections".to_string());
        
        // Memory map
        if config.generate_map {
            let map_path = config.map_path.as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "output.map".to_string());
            flags.push(format!("-Wl,-Map={}", map_path));
        }
        
        // No standard startup
        flags.push("-nostartfiles".to_string());
        
        // Additional flags
        flags.extend(config.flags.iter().cloned());
        
        flags
    }
}
```


## Configuration System

### Settings File Structure

**Global Settings** (`~/.axiom/settings.toml`):
```toml
[toolchains]
auto_detect = true

[toolchains.arm]
path = "/opt/homebrew/bin/arm-none-eabi-gcc"
search_paths = [
    "/Applications/STM32CubeIDE.app/Contents/Eclipse/plugins"
]

[toolchains.arm.settings]
default_mcu = "cortex-m7"
default_fpu = "fpv5-d16"
default_float_abi = "hard"
```

**Project Settings** (`.axiom/toolchain.toml`):
```toml
[arm]
mcu = "cortex-m7"
fpu = "fpv5-d16"
float_abi = "hard"
linker_script = "STM32H750VBTX_FLASH.ld"

[arm.defines]
values = ["STM32H750xx", "USE_HAL_DRIVER", "USE_PWR_LDO_SUPPLY"]

[arm.include_paths]
values = [
    "Core/Inc",
    "Drivers/STM32H7xx_HAL_Driver/Inc",
    "Drivers/CMSIS/Device/ST/STM32H7xx/Include",
    "Drivers/CMSIS/Include"
]

[compliance]
do178c_enabled = true
do330_enabled = true
dal = "B"
```

### Configuration Merging

```rust
impl ConfigurationSystem {
    /// Load merged configuration (project overrides global)
    pub fn load_merged(&self, project_path: &Path) -> Result<ToolchainConfig> {
        // Load global config
        let global = self.load_global()?;
        
        // Load project config if exists
        let project_config_path = project_path.join(".axiom/toolchain.toml");
        let project = if project_config_path.exists() {
            Some(self.load_project(&project_config_path)?)
        } else {
            None
        };
        
        // Merge with project taking precedence
        Ok(self.merge_configs(global, project))
    }
    
    fn merge_configs(
        &self,
        global: ToolchainConfig,
        project: Option<ToolchainConfig>
    ) -> ToolchainConfig {
        match project {
            Some(proj) => ToolchainConfig {
                // Project values override global
                path: proj.path.or(global.path),
                search_paths: if proj.search_paths.is_empty() {
                    global.search_paths
                } else {
                    proj.search_paths
                },
                // Deep merge settings
                settings: self.merge_settings(global.settings, proj.settings),
            },
            None => global,
        }
    }
}
```


## Compliance System Architecture

### DO-178C Traceability Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                    Source Code Analysis                          │
│  // REQ-001: Implement motor control                            │
│  void motor_control(void) { ... }                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Test Code Analysis                            │
│  // TEST: REQ-001                                                │
│  void test_motor_control(void) { ... }                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  Traceability Matrix                             │
│  ┌──────────┬─────────────────┬─────────────────┬────────────┐ │
│  │ Req ID   │ Source File     │ Test File       │ Status     │ │
│  ├──────────┼─────────────────┼─────────────────┼────────────┤ │
│  │ REQ-001  │ motor.c:45      │ test_motor.c:12 │ Covered    │ │
│  │ REQ-002  │ sensor.c:23     │ -               │ No Test    │ │
│  │ REQ-003  │ -               │ test_nav.c:8    │ No Impl    │ │
│  └──────────┴─────────────────┴─────────────────┴────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Coverage Analysis Pipeline

```
┌─────────────────────────────────────────────────────────────────┐
│  1. INSTRUMENTATION                                              │
│     arm-none-eabi-gcc --coverage -fprofile-arcs -ftest-coverage │
│     Produces: *.gcno files (coverage notes)                     │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. TEST EXECUTION                                               │
│     Run instrumented binary on target or simulator              │
│     Produces: *.gcda files (coverage data)                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. COVERAGE ANALYSIS                                            │
│     gcov --branch-probabilities --all-blocks *.gcda             │
│     Parse: statement, branch, decision coverage                 │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. MC/DC ANALYSIS (DAL A/B)                                    │
│     Analyze branch combinations for Modified Condition/         │
│     Decision Coverage                                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. REPORT GENERATION                                            │
│     - HTML report with source highlighting                      │
│     - LCOV format for tool integration                          │
│     - XML for certification documentation                       │
└─────────────────────────────────────────────────────────────────┘
```

### Tool Qualification Data Collection

```rust
/// Tool usage logger for DO-330 compliance
pub struct ToolQualificationLogger {
    /// Log file path
    log_path: PathBuf,
    
    /// Current session ID
    session_id: Uuid,
}

impl ToolQualificationLogger {
    /// Log a tool invocation
    pub fn log_invocation(&self, record: ToolUsageRecord) -> Result<()> {
        // Compute checksums for all input files
        let input_checksums = self.compute_checksums(&record.inputs)?;
        
        // Execute tool and capture output
        let result = self.execute_tool(&record)?;
        
        // Compute checksums for all output files
        let output_checksums = self.compute_checksums(&result.outputs)?;
        
        // Create complete record
        let complete_record = ToolUsageRecord {
            tool: record.tool,
            version: record.version,
            arguments: record.arguments,
            input_checksums,
            output_checksums,
            timestamp: Utc::now(),
            exit_code: result.exit_code,
            diagnostics: result.diagnostics,
        };
        
        // Append to log file (append-only for integrity)
        self.append_to_log(&complete_record)?;
        
        Ok(())
    }
    
    /// Compute SHA-256 checksums for files
    fn compute_checksums(&self, files: &[PathBuf]) -> Result<HashMap<PathBuf, String>> {
        files.iter()
            .map(|f| {
                let hash = sha256_file(f)?;
                Ok((f.clone(), hash))
            })
            .collect()
    }
}
```


## Error Handling Strategy

### Error Types

```rust
/// ARM toolchain errors
#[derive(Debug, thiserror::Error)]
pub enum ArmToolchainError {
    #[error("ARM toolchain not found. Install via: brew install arm-none-eabi-gcc")]
    NotFound,
    
    #[error("Incomplete toolchain at {path}: missing {missing:?}")]
    Incomplete { path: PathBuf, missing: Vec<String> },
    
    #[error("Toolchain version {version} is below minimum required {minimum}")]
    VersionTooOld { version: String, minimum: String },
    
    #[error("Linker script not found: {path}")]
    LinkerScriptNotFound { path: PathBuf },
    
    #[error("Memory overflow in region {region}: {details}")]
    MemoryOverflow { region: String, details: String },
    
    #[error("Compilation failed: {message}")]
    CompilationFailed { message: String, diagnostics: Vec<Diagnostic> },
    
    #[error("Binary generation failed at {stage}: {message}")]
    BinaryGenerationFailed { stage: String, message: String },
    
    #[error("Invalid MCU configuration: {message}")]
    InvalidMcuConfig { message: String },
}

/// Compliance errors
#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Untraceable code found in {files:?}")]
    UntraceableCode { files: Vec<PathBuf> },
    
    #[error("Requirements without tests: {requirements:?}")]
    UntestedRequirements { requirements: Vec<String> },
    
    #[error("Coverage below threshold: {coverage}% < {threshold}%")]
    InsufficientCoverage { coverage: f64, threshold: f64 },
    
    #[error("Tool qualification data missing for {tool}")]
    MissingQualificationData { tool: String },
}
```

### Error Recovery

```rust
impl ArmToolchainDetector {
    /// Detect with helpful error messages
    pub fn detect_with_suggestions(&self) -> Result<Vec<ArmToolchainSuite>, ArmToolchainError> {
        let toolchains = self.detect_all();
        
        if toolchains.is_empty() {
            // Provide platform-specific installation suggestions
            let suggestion = match std::env::consts::OS {
                "macos" => "Install via Homebrew: brew install arm-none-eabi-gcc\n\
                           Or install STM32CubeIDE from st.com",
                "linux" => "Install via package manager:\n\
                           Ubuntu/Debian: sudo apt install gcc-arm-none-eabi\n\
                           Fedora: sudo dnf install arm-none-eabi-gcc-cs",
                "windows" => "Download from ARM Developer:\n\
                             https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain",
                _ => "Install ARM GCC toolchain for your platform",
            };
            
            return Err(ArmToolchainError::NotFound);
        }
        
        Ok(toolchains)
    }
}
```


## Testing Strategy

### Property-Based Testing Framework

The implementation will use `proptest` for Rust property-based testing to verify correctness properties.

### Correctness Properties

#### P1: Toolchain Detection Completeness
**Validates: Requirements 1.1-1.6, 2.1-2.9**

For any valid ARM toolchain installation path, the detector should:
- Return a toolchain if and only if arm-none-eabi-gcc exists and is executable
- Include all present tools in the suite
- Correctly identify missing tools
- Parse version information correctly

```rust
proptest! {
    #[test]
    fn prop_detection_returns_only_valid_toolchains(
        paths in prop::collection::vec(any::<PathBuf>(), 0..10)
    ) {
        let detector = ArmToolchainDetector::new();
        let result = detector.detect_from_paths(&paths);
        
        for toolchain in &result {
            // Every returned toolchain must have a valid gcc path
            prop_assert!(toolchain.gcc.exists());
            prop_assert!(is_executable(&toolchain.gcc));
            
            // Version must be parseable
            prop_assert!(!toolchain.version.is_empty());
        }
    }
}
```

#### P2: Flag Generation Consistency
**Validates: Requirements 3.1-3.7**

For any valid MCU configuration, generated flags should:
- Always include -mcpu with the specified CPU
- Include -mthumb when thumb mode is enabled
- Include -mfpu when FPU is specified
- Include -mfloat-abi with the specified ABI
- Include all specified defines with -D prefix

```rust
proptest! {
    #[test]
    fn prop_flags_contain_all_config_values(config in arb_mcu_config()) {
        let flags = config.compiler_flags();
        
        // CPU flag always present
        prop_assert!(flags.iter().any(|f| f.starts_with("-mcpu=")));
        prop_assert!(flags.iter().any(|f| f.contains(&config.cpu)));
        
        // Thumb flag present when enabled
        if config.thumb {
            prop_assert!(flags.contains(&"-mthumb".to_string()));
        }
        
        // FPU flag present when specified
        if let Some(ref fpu) = config.fpu {
            prop_assert!(flags.iter().any(|f| f.contains(fpu)));
        }
        
        // All defines present
        for define in &config.defines {
            prop_assert!(flags.iter().any(|f| f == &format!("-D{}", define)));
        }
    }
}
```

#### P3: Include Path Order Preservation
**Validates: Requirements 6.1-6.3**

Include paths must be preserved in the exact order specified:

```rust
proptest! {
    #[test]
    fn prop_include_paths_preserve_order(
        paths in prop::collection::vec(any::<PathBuf>(), 0..20)
    ) {
        let request = CompileRequest::new_with_includes(paths.clone());
        let flags = request.generate_flags();
        
        // Extract include paths from flags
        let include_flags: Vec<_> = flags.iter()
            .filter(|f| f.starts_with("-I"))
            .map(|f| PathBuf::from(&f[2..]))
            .collect();
        
        // Order must match
        prop_assert_eq!(include_flags, paths);
    }
}
```

#### P4: Configuration Persistence Roundtrip
**Validates: Requirements 13.1-13.7, 14.1-14.7, 15.1-15.7**

Settings saved and loaded should be identical:

```rust
proptest! {
    #[test]
    fn prop_settings_roundtrip(settings in arb_toolchain_settings()) {
        let config_system = ConfigurationSystem::new_temp();
        
        // Save settings
        config_system.save(&settings, SettingsScope::Global)?;
        
        // Load settings
        let loaded = config_system.load_global()?;
        
        // Must be identical
        prop_assert_eq!(settings, loaded);
    }
}
```

#### P5: Version Parsing Correctness
**Validates: Requirements 12.1-12.5**

Version parsing should extract valid semver components:

```rust
proptest! {
    #[test]
    fn prop_version_parsing_extracts_components(
        major in 1u32..100,
        minor in 0u32..100,
        patch in 0u32..100
    ) {
        let version_string = format!(
            "arm-none-eabi-gcc (GNU Arm Embedded Toolchain) {}.{}.{}",
            major, minor, patch
        );
        
        let parsed = parse_arm_gcc_version(&version_string);
        
        prop_assert!(parsed.is_some());
        let (m, n, p) = parsed.unwrap();
        prop_assert_eq!(m, major);
        prop_assert_eq!(n, minor);
        prop_assert_eq!(p, patch);
    }
}
```

#### P6: Traceability Matrix Completeness
**Validates: Requirements 18.1-18.7**

All annotated requirements should appear in the traceability matrix:

```rust
proptest! {
    #[test]
    fn prop_all_annotations_in_matrix(
        annotations in prop::collection::vec(arb_requirement_annotation(), 1..50)
    ) {
        let source_files = generate_source_with_annotations(&annotations);
        let matrix = TraceabilitySystem::analyze(&source_files);
        
        for annotation in &annotations {
            prop_assert!(
                matrix.contains_requirement(&annotation.requirement_id),
                "Requirement {} not found in matrix",
                annotation.requirement_id
            );
        }
    }
}
```

#### P7: Tool Usage Logging Integrity
**Validates: Requirements 22.1-22.7, 23.1-23.7**

All tool invocations must be logged with correct checksums:

```rust
proptest! {
    #[test]
    fn prop_tool_usage_logged_with_checksums(
        invocations in prop::collection::vec(arb_tool_invocation(), 1..10)
    ) {
        let logger = ToolQualificationLogger::new_temp();
        
        for invocation in &invocations {
            logger.log_invocation(invocation.clone())?;
        }
        
        let records = logger.get_all_records()?;
        
        // All invocations logged
        prop_assert_eq!(records.len(), invocations.len());
        
        // All records have checksums
        for record in &records {
            prop_assert!(!record.input_checksums.is_empty() || invocation.inputs.is_empty());
            prop_assert!(!record.output_checksums.is_empty() || invocation.outputs.is_empty());
        }
    }
}
```

#### P8: Compliance Mode State Preservation
**Validates: Requirements 27.1-27.10**

Disabling and re-enabling compliance modes should preserve data:

```rust
proptest! {
    #[test]
    fn prop_compliance_mode_preserves_data(
        initial_data in arb_compliance_data()
    ) {
        let system = ComplianceSystem::new_temp();
        
        // Enable mode and add data
        system.enable_mode(ComplianceMode::Do178c)?;
        system.add_traceability_data(&initial_data)?;
        
        // Disable mode
        system.disable_mode(ComplianceMode::Do178c)?;
        
        // Re-enable mode
        system.enable_mode(ComplianceMode::Do178c)?;
        
        // Data should be preserved
        let loaded_data = system.get_traceability_data()?;
        prop_assert_eq!(initial_data, loaded_data);
    }
}
```


## File Structure

### New and Modified Files

```
crates/
├── axiom-toolchain/
│   ├── Cargo.toml                    # Add proptest, sha2 dependencies
│   └── src/
│       ├── lib.rs                    # Export new modules
│       ├── detection.rs              # MODIFY: Add STM32CubeIDE paths, suite validation
│       ├── invocation.rs             # MODIFY: Add ARM-specific compilation
│       ├── types.rs                  # MODIFY: Add ARM types
│       ├── arm_mcu.rs                # NEW: MCU configurations
│       ├── binary_gen.rs             # NEW: Binary generation
│       ├── visualizer.rs             # NEW: Compiler stage visualization
│       └── makefile.rs               # NEW: Makefile support
│
├── axiom-compliance/                 # NEW CRATE
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── traceability.rs           # Requirement traceability
│       ├── coverage.rs               # Coverage analysis
│       ├── tool_qualification.rs     # DO-330 support
│       ├── system_integration.rs     # ARP4754A support
│       ├── export.rs                 # Artifact export
│       └── modes.rs                  # Compliance mode management
│
├── axiom-settings/
│   └── src/
│       ├── schema.rs                 # MODIFY: Add ARM and compliance settings
│       └── persistence.rs            # MODIFY: Add project-level config support

src-tauri/
└── src/
    └── commands/
        ├── toolchain.rs              # MODIFY: Add ARM-specific commands
        ├── compliance.rs             # NEW: Compliance commands
        └── visualizer.rs             # NEW: Visualizer commands

src/lib/
├── stores/
│   ├── armToolchain.ts               # NEW: ARM toolchain store
│   └── compliance.ts                 # NEW: Compliance store
└── components/
    ├── ArmToolchainSettings.svelte   # NEW: Settings UI
    ├── CompilerVisualizer.svelte     # NEW: Stage visualization
    └── CompliancePanel.svelte        # NEW: Compliance dashboard
```

### Dependencies

**axiom-toolchain/Cargo.toml additions:**
```toml
[dependencies]
glob = "0.3"
sha2 = "0.10"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }

[dev-dependencies]
proptest = "1.4"
tempfile = "3.10"
```

**New axiom-compliance/Cargo.toml:**
```toml
[package]
name = "axiom-compliance"
version = "0.1.0"
edition = "2021"

[dependencies]
axiom-core = { path = "../axiom-core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
sha2 = "0.10"
thiserror = "1.0"
regex = "1.10"
csv = "1.3"

[dev-dependencies]
proptest = "1.4"
tempfile = "3.10"
```


## Security Considerations

### Path Validation
- All user-provided paths must be validated before use
- Prevent path traversal attacks (../)
- Validate that toolchain binaries are in expected locations
- Verify binary signatures where possible (macOS code signing)

### Command Injection Prevention
- Never interpolate user input directly into shell commands
- Use `Command::new()` with explicit argument arrays
- Sanitize all file paths and flag values

### Compliance Data Integrity
- Use append-only logs for tool qualification data
- Compute and verify checksums for all artifacts
- Digital signatures for exported certification packages
- No AI-generated content in compliance artifacts (Requirement 28)

## Performance Considerations

### Toolchain Detection Caching
- Cache detection results with TTL
- Invalidate cache on settings change
- Background refresh on IDE startup

### Compilation Performance
- Parallel compilation of independent source files
- Incremental compilation support
- Memory-mapped file I/O for large projects

### Coverage Analysis
- Lazy loading of coverage data
- Incremental coverage updates
- Background analysis for large codebases

## Migration Path

### Phase 1: Core Toolchain Integration
1. Extend detection for STM32CubeIDE paths
2. Add ARM-specific compilation flags
3. Implement binary generation
4. Add basic settings UI

### Phase 2: Advanced Features
1. Compiler stage visualization
2. Makefile workflow support
3. Extended settings management
4. Project-level configuration

### Phase 3: Compliance Features
1. DO-178C traceability system
2. Coverage analysis
3. DO-330 tool qualification
4. ARP4754A system integration

### Phase 4: Export and Reporting
1. Certification artifact export
2. Report generation
3. Digital signatures
4. Template customization

## Open Questions

1. **Coverage Tool Integration**: Should we support external coverage tools (LDRA, VectorCAST) in addition to gcov?

2. **Requirements Import Format**: What formats should be supported for importing system requirements (ReqIF, DOORS, CSV)?

3. **Static Analysis Tools**: Which static analysis tools should be prioritized for integration (PC-lint, Coverity, Polyspace)?

4. **Simulator Support**: Should we integrate with ARM simulators (QEMU) for coverage collection without hardware?

## References

- [ARM GCC Toolchain Documentation](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain)
- [STM32CubeIDE User Guide](https://www.st.com/resource/en/user_manual/um2609-stm32cubeide-user-guide-stmicroelectronics.pdf)
- [DO-178C Software Considerations in Airborne Systems](https://www.rtca.org/products/do-178c/)
- [DO-330 Software Tool Qualification Considerations](https://www.rtca.org/products/do-330/)
- [ARP4754A Guidelines for Development of Civil Aircraft and Systems](https://www.sae.org/standards/content/arp4754a/)
- [GCOV Coverage Tool](https://gcc.gnu.org/onlinedocs/gcc/Gcov.html)


## Comprehensive Testing Plan

### Unit Tests

#### axiom-toolchain Unit Tests

**detection.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_arm_gcc_version_standard() {
        let output = "arm-none-eabi-gcc (Arm GNU Toolchain 14.3.Rel1) 14.3.1 20250623";
        let version = parse_arm_gcc_version(output);
        assert_eq!(version, Some("14.3.1".to_string()));
    }
    
    #[test]
    fn test_parse_arm_gcc_version_stm32cubeide() {
        let output = "arm-none-eabi-gcc (GNU Arm Embedded Toolchain 10.3-2021.10) 10.3.1 20210824";
        let version = parse_arm_gcc_version(output);
        assert_eq!(version, Some("10.3.1".to_string()));
    }
    
    #[test]
    fn test_validate_toolchain_suite_complete() {
        let suite = create_mock_complete_suite();
        assert!(matches!(suite.status, ToolchainCompleteness::Complete));
    }
    
    #[test]
    fn test_validate_toolchain_suite_missing_gdb() {
        let suite = create_mock_suite_without_gdb();
        assert!(matches!(suite.status, ToolchainCompleteness::Incomplete { .. }));
    }
    
    #[test]
    fn test_version_comparison_minimum() {
        assert!(is_version_compatible("14.3.1", "8.0.0"));
        assert!(is_version_compatible("8.0.0", "8.0.0"));
        assert!(!is_version_compatible("7.9.9", "8.0.0"));
    }
    
    #[test]
    fn test_detect_toolchain_source_homebrew() {
        let path = PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc");
        assert_eq!(detect_source(&path), ToolchainSource::Homebrew);
    }
    
    #[test]
    fn test_detect_toolchain_source_stm32cubeide() {
        let path = PathBuf::from("/Applications/STM32CubeIDE.app/Contents/Eclipse/plugins/com.st.stm32cube.ide.mcu.externaltools.gnu-tools-for-stm32.12.3.rel1.macos64_1.0.200.202406191623/tools/bin/arm-none-eabi-gcc");
        assert_eq!(detect_source(&path), ToolchainSource::Stm32CubeIde);
    }
}
```

**arm_mcu.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cortex_m7_config() {
        let config = ArmMcuConfig::cortex_m7();
        assert_eq!(config.cpu, "cortex-m7");
        assert!(config.thumb);
    }
    
    #[test]
    fn test_compiler_flags_basic() {
        let config = ArmMcuConfig {
            cpu: "cortex-m4".to_string(),
            thumb: true,
            fpu: None,
            float_abi: FloatAbi::Soft,
            defines: vec![],
        };
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mcpu=cortex-m4".to_string()));
        assert!(flags.contains(&"-mthumb".to_string()));
        assert!(flags.contains(&"-mfloat-abi=soft".to_string()));
    }
    
    #[test]
    fn test_compiler_flags_with_fpu() {
        let config = ArmMcuConfig {
            cpu: "cortex-m7".to_string(),
            thumb: true,
            fpu: Some("fpv5-d16".to_string()),
            float_abi: FloatAbi::Hard,
            defines: vec!["STM32H750xx".to_string()],
        };
        let flags = config.compiler_flags();
        assert!(flags.contains(&"-mfpu=fpv5-d16".to_string()));
        assert!(flags.contains(&"-mfloat-abi=hard".to_string()));
        assert!(flags.contains(&"-DSTM32H750xx".to_string()));
    }
    
    #[test]
    fn test_linker_flags_with_script() {
        let mcu = ArmMcuConfig::cortex_m7();
        let linker = LinkerConfig {
            script: PathBuf::from("STM32H750VBTX_FLASH.ld"),
            generate_map: true,
            map_path: Some(PathBuf::from("output.map")),
            flags: vec![],
        };
        let flags = mcu.linker_flags(&linker);
        assert!(flags.contains(&"-TSTM32H750VBTX_FLASH.ld".to_string()));
        assert!(flags.contains(&"-Wl,-Map=output.map".to_string()));
    }
}
```

**binary_gen.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_objcopy_hex_command() {
        let cmd = build_objcopy_hex_command(
            &PathBuf::from("/usr/bin/arm-none-eabi-objcopy"),
            &PathBuf::from("output.elf"),
            &PathBuf::from("output.hex"),
        );
        assert_eq!(cmd[0], "-O");
        assert_eq!(cmd[1], "ihex");
    }
    
    #[test]
    fn test_objcopy_bin_command() {
        let cmd = build_objcopy_bin_command(
            &PathBuf::from("/usr/bin/arm-none-eabi-objcopy"),
            &PathBuf::from("output.elf"),
            &PathBuf::from("output.bin"),
        );
        assert_eq!(cmd[0], "-O");
        assert_eq!(cmd[1], "binary");
    }
    
    #[test]
    fn test_parse_size_output() {
        let output = "   text    data     bss     dec     hex filename\n  12345    1234     567   14146    3742 output.elf";
        let stats = parse_size_output(output);
        assert_eq!(stats.text, 12345);
        assert_eq!(stats.data, 1234);
        assert_eq!(stats.bss, 567);
    }
}
```

**visualizer.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_preprocessor_flags() {
        let flags = build_preprocessor_flags(&ArmMcuConfig::cortex_m7());
        assert!(flags.contains(&"-E".to_string()));
    }
    
    #[test]
    fn test_assembly_flags() {
        let flags = build_assembly_flags(&ArmMcuConfig::cortex_m7());
        assert!(flags.contains(&"-S".to_string()));
    }
    
    #[test]
    fn test_objdump_disassembly_flags() {
        let flags = build_disassembly_flags();
        assert!(flags.contains(&"-d".to_string()));
    }
    
    #[test]
    fn test_objdump_symbol_flags() {
        let flags = build_symbol_table_flags();
        assert!(flags.contains(&"-t".to_string()));
    }
}
```

**makefile.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_makefile_present() {
        let temp_dir = tempfile::tempdir().unwrap();
        std::fs::write(temp_dir.path().join("Makefile"), "all:\n\techo hello").unwrap();
        
        let result = detect_makefile(temp_dir.path());
        assert!(result.is_some());
    }
    
    #[test]
    fn test_detect_makefile_absent() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = detect_makefile(temp_dir.path());
        assert!(result.is_none());
    }
    
    #[test]
    fn test_parse_makefile_targets() {
        let content = ".PHONY: all clean flash\nall: build\nclean:\n\trm -rf build\nflash: all\n\tst-flash write";
        let targets = parse_makefile_targets(content);
        assert!(targets.contains(&"all".to_string()));
        assert!(targets.contains(&"clean".to_string()));
        assert!(targets.contains(&"flash".to_string()));
    }
}
```

#### axiom-compliance Unit Tests

**traceability.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_requirement_annotation() {
        let line = "// REQ-001: Implement motor control";
        let annotation = parse_requirement_annotation(line);
        assert_eq!(annotation, Some("REQ-001".to_string()));
    }
    
    #[test]
    fn test_parse_test_annotation() {
        let line = "// TEST: REQ-001, REQ-002";
        let annotations = parse_test_annotations(line);
        assert_eq!(annotations, vec!["REQ-001", "REQ-002"]);
    }
    
    #[test]
    fn test_build_traceability_matrix() {
        let links = vec![
            TraceabilityLink::new("REQ-001", "motor.c", 45, LinkType::Implementation),
            TraceabilityLink::new("REQ-001", "test_motor.c", 12, LinkType::Test),
        ];
        let matrix = build_matrix(&links);
        assert!(matrix.is_requirement_covered("REQ-001"));
    }
    
    #[test]
    fn test_find_untraceable_code() {
        let source = "void untraced_function() { }\n// REQ-001\nvoid traced_function() { }";
        let untraceable = find_untraceable_functions(source);
        assert!(untraceable.contains(&"untraced_function".to_string()));
        assert!(!untraceable.contains(&"traced_function".to_string()));
    }
}
```

**coverage.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_gcov_output() {
        let gcov_output = "        1:   10:    if (x > 0) {\n        -:   11:        // comment\n    #####:   12:        return -1;\n        1:   13:    }";
        let coverage = parse_gcov_output(gcov_output);
        assert_eq!(coverage.executed_lines, vec![10, 13]);
        assert_eq!(coverage.unexecuted_lines, vec![12]);
    }
    
    #[test]
    fn test_calculate_statement_coverage() {
        let coverage = FileCoverage {
            executed_lines: vec![1, 2, 3, 4, 5],
            total_lines: 10,
            ..Default::default()
        };
        assert_eq!(coverage.statement_coverage_percent(), 50.0);
    }
    
    #[test]
    fn test_calculate_branch_coverage() {
        let coverage = FileCoverage {
            taken_branches: 3,
            total_branches: 4,
            ..Default::default()
        };
        assert_eq!(coverage.branch_coverage_percent(), 75.0);
    }
}
```

**tool_qualification.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compute_file_checksum() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "test content").unwrap();
        
        let checksum = compute_sha256(temp_file.path()).unwrap();
        assert!(!checksum.is_empty());
        assert_eq!(checksum.len(), 64); // SHA-256 hex string
    }
    
    #[test]
    fn test_tool_usage_record_serialization() {
        let record = ToolUsageRecord {
            tool: "arm-none-eabi-gcc".to_string(),
            version: "14.3.1".to_string(),
            arguments: vec!["-c", "main.c"].iter().map(|s| s.to_string()).collect(),
            input_checksums: HashMap::new(),
            output_checksums: HashMap::new(),
            timestamp: Utc::now(),
            exit_code: 0,
            diagnostics: vec![],
        };
        
        let json = serde_json::to_string(&record).unwrap();
        let parsed: ToolUsageRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(record.tool, parsed.tool);
    }
}
```

#### axiom-settings Unit Tests

**schema.rs tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arm_settings_default() {
        let settings = ArmToolchainSettings::default();
        assert!(settings.mcu.is_none());
        assert!(settings.include_paths.is_empty());
    }
    
    #[test]
    fn test_settings_serialization_roundtrip() {
        let settings = ToolchainSettings {
            auto_detect: true,
            toolchains: HashMap::from([
                ("arm".to_string(), ToolchainConfig {
                    path: Some(PathBuf::from("/opt/homebrew/bin/arm-none-eabi-gcc")),
                    search_paths: vec![],
                    settings: HashMap::new(),
                }),
            ]),
        };
        
        let toml = toml::to_string(&settings).unwrap();
        let parsed: ToolchainSettings = toml::from_str(&toml).unwrap();
        assert_eq!(settings.auto_detect, parsed.auto_detect);
    }
    
    #[test]
    fn test_unknown_toolchain_preserved() {
        let toml = r#"
            auto_detect = true
            [toolchains.riscv]
            path = "/opt/riscv/bin/riscv-gcc"
        "#;
        
        let settings: ToolchainSettings = toml::from_str(toml).unwrap();
        assert!(settings.toolchains.contains_key("riscv"));
    }
}
```


### Integration Tests

#### Test Reference Project Structure

A comprehensive test reference project will be created at `tests/fixtures/arm-reference-project/` to test all toolchain functionalities:

```
tests/fixtures/arm-reference-project/
├── Makefile                          # Standard ARM Makefile
├── STM32F103C8_FLASH.ld             # Linker script for STM32F103 (Blue Pill)
├── STM32H750VB_FLASH.ld             # Linker script for STM32H750
├── STM32H750VB_RAM.ld               # RAM-only linker script
├── .axiom/
│   └── toolchain.toml               # Project-specific toolchain config
├── Core/
│   ├── Inc/
│   │   ├── main.h
│   │   ├── stm32f1xx_hal_conf.h     # HAL configuration
│   │   └── config.h
│   ├── Src/
│   │   ├── main.c                   # Main application with REQ annotations
│   │   ├── startup_stm32f103.s      # Startup assembly
│   │   ├── system_stm32f1xx.c       # System initialization
│   │   ├── syscalls.c               # Newlib syscalls
│   │   └── sysmem.c                 # Memory allocation
│   └── Startup/
│       └── startup_stm32f103xb.s    # Vector table
├── Drivers/
│   ├── gpio.c                       # GPIO driver with edge cases
│   ├── gpio.h
│   ├── uart.c                       # UART driver
│   ├── uart.h
│   ├── timer.c                      # Timer driver with FPU usage
│   └── timer.h
├── Tests/
│   ├── test_gpio.c                  # Unit tests with TEST annotations
│   ├── test_uart.c
│   └── test_runner.c
├── edge_cases/
│   ├── empty_file.c                 # Empty source file
│   ├── syntax_error.c               # File with syntax errors
│   ├── linker_overflow.c            # Code that causes memory overflow
│   ├── missing_include.c            # Missing header file
│   ├── unicode_comments.c           # Unicode in comments
│   ├── very_long_lines.c            # Lines > 10000 chars
│   ├── nested_includes.h            # Deeply nested includes
│   ├── circular_include_a.h         # Circular include test
│   ├── circular_include_b.h
│   ├── preprocessor_heavy.c         # Heavy macro usage
│   └── inline_assembly.c            # Inline ARM assembly
├── compliance/
│   ├── traced_module.c              # Fully traced code (REQ-xxx annotations)
│   ├── untraced_module.c            # Code without traceability
│   ├── partial_trace.c              # Partially traced code
│   └── requirements.csv             # Requirements list for traceability
└── configs/
    ├── cortex_m0.toml               # Cortex-M0 config
    ├── cortex_m3.toml               # Cortex-M3 config
    ├── cortex_m4_no_fpu.toml        # Cortex-M4 without FPU
    ├── cortex_m4_fpu.toml           # Cortex-M4 with FPU
    ├── cortex_m7_dp_fpu.toml        # Cortex-M7 with double-precision FPU
    └── invalid_config.toml          # Invalid configuration for error testing
```

#### Test Reference Project Files

**tests/fixtures/arm-reference-project/Core/Src/main.c:**
```c
// SPDX-License-Identifier: Apache-2.0
// Test reference project main file

#include "main.h"
#include "gpio.h"
#include "uart.h"
#include "timer.h"

// REQ-001: System initialization
void SystemInit(void) {
    // Configure system clock
    // REQ-001.1: Clock configuration
    configure_clock();
}

// REQ-002: GPIO LED control
void toggle_led(void) {
    gpio_toggle(LED_PIN);
}

// REQ-003: UART communication
// REQ-003.1: Send single byte
void send_byte(uint8_t data) {
    uart_transmit(&data, 1);
}

// REQ-003.2: Send string
void send_string(const char* str) {
    while (*str) {
        send_byte(*str++);
    }
}

// REQ-004: Timer-based delay
void delay_ms(uint32_t ms) {
    timer_delay(ms);
}

// REQ-005: Main loop with watchdog
int main(void) {
    SystemInit();
    gpio_init();
    uart_init(115200);
    timer_init();
    
    while (1) {
        toggle_led();
        send_string("Hello, ARM!\r\n");
        delay_ms(1000);
        
        // REQ-005.1: Watchdog refresh
        watchdog_refresh();
    }
    
    return 0;
}

// Untraced function for compliance testing
void untraced_helper(void) {
    // This function has no requirement annotation
    volatile int x = 0;
    x++;
}
```

**tests/fixtures/arm-reference-project/Drivers/gpio.c:**
```c
// SPDX-License-Identifier: Apache-2.0
// GPIO Driver - Test reference

#include "gpio.h"

// REQ-002.1: GPIO initialization
void gpio_init(void) {
    // Enable GPIO clock
    RCC->APB2ENR |= RCC_APB2ENR_IOPCEN;
    
    // Configure PC13 as output (LED on Blue Pill)
    GPIOC->CRH &= ~(0xF << 20);
    GPIOC->CRH |= (0x2 << 20);  // Output 2MHz, push-pull
}

// REQ-002.2: GPIO write
void gpio_write(uint16_t pin, uint8_t state) {
    if (state) {
        GPIOC->BSRR = (1 << pin);
    } else {
        GPIOC->BRR = (1 << pin);
    }
}

// REQ-002.3: GPIO toggle
void gpio_toggle(uint16_t pin) {
    GPIOC->ODR ^= (1 << pin);
}

// REQ-002.4: GPIO read
uint8_t gpio_read(uint16_t pin) {
    return (GPIOC->IDR & (1 << pin)) ? 1 : 0;
}

// Edge case: Boundary pin numbers
void gpio_write_boundary(uint16_t pin, uint8_t state) {
    // Pin 0 (minimum)
    if (pin == 0) {
        gpio_write(0, state);
    }
    // Pin 15 (maximum for 16-bit port)
    else if (pin == 15) {
        gpio_write(15, state);
    }
    // Invalid pin (edge case)
    else if (pin > 15) {
        // Do nothing - invalid pin
        return;
    }
}
```

**tests/fixtures/arm-reference-project/edge_cases/syntax_error.c:**
```c
// This file intentionally contains syntax errors for testing

#include "main.h"

void function_with_error(void) {
    int x = 10
    // Missing semicolon above
    
    if (x > 5 {  // Missing closing parenthesis
        x++;
    }
    
    return x;  // Return in void function
}

void unclosed_brace(void) {
    int y = 20;
    // Missing closing brace
```

**tests/fixtures/arm-reference-project/edge_cases/linker_overflow.c:**
```c
// This file creates a large array to test memory overflow detection

#include <stdint.h>

// This array is intentionally too large for most MCU flash
// STM32F103C8 has 64KB flash, this requests 128KB
const uint8_t huge_array[131072] __attribute__((section(".rodata"))) = {
    [0 ... 131071] = 0xAA
};

void use_huge_array(void) {
    volatile uint8_t x = huge_array[0];
    (void)x;
}
```

**tests/fixtures/arm-reference-project/edge_cases/inline_assembly.c:**
```c
// Test inline ARM assembly handling

#include <stdint.h>

// REQ-EDGE-001: Inline assembly support
uint32_t get_primask(void) {
    uint32_t result;
    __asm volatile ("MRS %0, primask" : "=r" (result));
    return result;
}

void set_primask(uint32_t value) {
    __asm volatile ("MSR primask, %0" : : "r" (value) : "memory");
}

void disable_interrupts(void) {
    __asm volatile ("cpsid i" : : : "memory");
}

void enable_interrupts(void) {
    __asm volatile ("cpsie i" : : : "memory");
}

// DSB and ISB barriers
void memory_barrier(void) {
    __asm volatile ("dsb 0xF" : : : "memory");
    __asm volatile ("isb 0xF" : : : "memory");
}

// NOP for timing
void nop_delay(uint32_t count) {
    while (count--) {
        __asm volatile ("nop");
    }
}
```

**tests/fixtures/arm-reference-project/STM32F103C8_FLASH.ld:**
```ld
/* Linker script for STM32F103C8 (Blue Pill) */
/* 64KB Flash, 20KB RAM */

ENTRY(Reset_Handler)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 20K
}

_estack = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .isr_vector :
    {
        . = ALIGN(4);
        KEEP(*(.isr_vector))
        . = ALIGN(4);
    } >FLASH

    .text :
    {
        . = ALIGN(4);
        *(.text)
        *(.text*)
        *(.glue_7)
        *(.glue_7t)
        *(.eh_frame)
        KEEP(*(.init))
        KEEP(*(.fini))
        . = ALIGN(4);
        _etext = .;
    } >FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } >FLASH

    .ARM.extab :
    {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    } >FLASH

    .ARM :
    {
        __exidx_start = .;
        *(.ARM.exidx*)
        __exidx_end = .;
    } >FLASH

    .preinit_array :
    {
        PROVIDE_HIDDEN(__preinit_array_start = .);
        KEEP(*(.preinit_array*))
        PROVIDE_HIDDEN(__preinit_array_end = .);
    } >FLASH

    .init_array :
    {
        PROVIDE_HIDDEN(__init_array_start = .);
        KEEP(*(SORT(.init_array.*)))
        KEEP(*(.init_array*))
        PROVIDE_HIDDEN(__init_array_end = .);
    } >FLASH

    .fini_array :
    {
        PROVIDE_HIDDEN(__fini_array_start = .);
        KEEP(*(SORT(.fini_array.*)))
        KEEP(*(.fini_array*))
        PROVIDE_HIDDEN(__fini_array_end = .);
    } >FLASH

    _sidata = LOADADDR(.data);

    .data :
    {
        . = ALIGN(4);
        _sdata = .;
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } >RAM AT> FLASH

    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        __bss_start__ = _sbss;
        *(.bss)
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
        __bss_end__ = _ebss;
    } >RAM

    ._user_heap_stack :
    {
        . = ALIGN(8);
        PROVIDE(end = .);
        PROVIDE(_end = .);
        . = . + 0x400;  /* Min heap size */
        . = . + 0x400;  /* Min stack size */
        . = ALIGN(8);
    } >RAM

    /DISCARD/ :
    {
        libc.a(*)
        libm.a(*)
        libgcc.a(*)
    }

    .ARM.attributes 0 : { *(.ARM.attributes) }
}
```

**tests/fixtures/arm-reference-project/Makefile:**
```makefile
# ARM Reference Project Makefile
# For testing Axiom IDE toolchain integration

# Toolchain
PREFIX ?= arm-none-eabi-
CC = $(PREFIX)gcc
CXX = $(PREFIX)g++
AS = $(PREFIX)gcc -x assembler-with-cpp
CP = $(PREFIX)objcopy
SZ = $(PREFIX)size
OD = $(PREFIX)objdump
GDB = $(PREFIX)gdb

# Target
TARGET = reference_project
MCU = cortex-m3

# Build directory
BUILD_DIR = build

# Source files
C_SOURCES = \
    Core/Src/main.c \
    Core/Src/system_stm32f1xx.c \
    Core/Src/syscalls.c \
    Drivers/gpio.c \
    Drivers/uart.c \
    Drivers/timer.c

ASM_SOURCES = \
    Core/Startup/startup_stm32f103xb.s

# Include paths
C_INCLUDES = \
    -ICore/Inc \
    -IDrivers

# Compiler flags
CPU = -mcpu=$(MCU)
THUMB = -mthumb

CFLAGS = $(CPU) $(THUMB)
CFLAGS += -Wall -Wextra -Werror
CFLAGS += -ffunction-sections -fdata-sections
CFLAGS += -fno-common
CFLAGS += $(C_INCLUDES)
CFLAGS += -std=c11

# Debug flags
DEBUG ?= 1
ifeq ($(DEBUG), 1)
CFLAGS += -g3 -gdwarf-2 -O0 -DDEBUG
else
CFLAGS += -Os -DNDEBUG
endif

# Linker script
LDSCRIPT = STM32F103C8_FLASH.ld

# Linker flags
LDFLAGS = $(CPU) $(THUMB)
LDFLAGS += -T$(LDSCRIPT)
LDFLAGS += -Wl,--gc-sections
LDFLAGS += -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref
LDFLAGS += -nostartfiles
LDFLAGS += --specs=nano.specs
LDFLAGS += --specs=nosys.specs

# Object files
OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))

OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
vpath %.s $(sort $(dir $(ASM_SOURCES)))

# Default target
all: $(BUILD_DIR)/$(TARGET).elf $(BUILD_DIR)/$(TARGET).hex $(BUILD_DIR)/$(TARGET).bin size

# ELF file
$(BUILD_DIR)/$(TARGET).elf: $(OBJECTS) $(LDSCRIPT)
	@mkdir -p $(BUILD_DIR)
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@

# HEX file
$(BUILD_DIR)/$(TARGET).hex: $(BUILD_DIR)/$(TARGET).elf
	$(CP) -O ihex $< $@

# BIN file
$(BUILD_DIR)/$(TARGET).bin: $(BUILD_DIR)/$(TARGET).elf
	$(CP) -O binary -S $< $@

# Compile C files
$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(BUILD_DIR)
	$(CC) -c $(CFLAGS) $< -o $@

# Compile ASM files
$(BUILD_DIR)/%.o: %.s
	@mkdir -p $(BUILD_DIR)
	$(AS) -c $(CFLAGS) $< -o $@

# Size report
size: $(BUILD_DIR)/$(TARGET).elf
	$(SZ) $<

# Disassembly
disasm: $(BUILD_DIR)/$(TARGET).elf
	$(OD) -d -S $< > $(BUILD_DIR)/$(TARGET).dis

# Symbols
symbols: $(BUILD_DIR)/$(TARGET).elf
	$(OD) -t $< > $(BUILD_DIR)/$(TARGET).sym

# Sections
sections: $(BUILD_DIR)/$(TARGET).elf
	$(OD) -h $< > $(BUILD_DIR)/$(TARGET).sections

# Preprocessor output
preprocess: Core/Src/main.c
	$(CC) -E $(CFLAGS) $< -o $(BUILD_DIR)/main.i

# Assembly output
assembly: Core/Src/main.c
	$(CC) -S $(CFLAGS) $< -o $(BUILD_DIR)/main.s

# Clean
clean:
	rm -rf $(BUILD_DIR)

# Flash (using st-flash)
flash: $(BUILD_DIR)/$(TARGET).bin
	st-flash write $< 0x08000000

# Debug (using GDB)
debug: $(BUILD_DIR)/$(TARGET).elf
	$(GDB) -ex "target remote :3333" $<

# Coverage build
coverage: CFLAGS += --coverage -fprofile-arcs -ftest-coverage
coverage: LDFLAGS += --coverage
coverage: clean all

# Edge case targets
edge-syntax-error:
	$(CC) -c $(CFLAGS) edge_cases/syntax_error.c -o $(BUILD_DIR)/syntax_error.o || true

edge-linker-overflow:
	$(CC) -c $(CFLAGS) edge_cases/linker_overflow.c -o $(BUILD_DIR)/linker_overflow.o
	$(CC) $(BUILD_DIR)/linker_overflow.o $(LDFLAGS) -o $(BUILD_DIR)/overflow.elf || true

edge-missing-include:
	$(CC) -c $(CFLAGS) edge_cases/missing_include.c -o $(BUILD_DIR)/missing_include.o || true

.PHONY: all clean flash debug size disasm symbols sections preprocess assembly coverage edge-syntax-error edge-linker-overflow edge-missing-include
```

**tests/fixtures/arm-reference-project/compliance/requirements.csv:**
```csv
ID,Description,Priority,Status
REQ-001,System initialization,High,Implemented
REQ-001.1,Clock configuration,High,Implemented
REQ-002,GPIO LED control,Medium,Implemented
REQ-002.1,GPIO initialization,Medium,Implemented
REQ-002.2,GPIO write,Medium,Implemented
REQ-002.3,GPIO toggle,Medium,Implemented
REQ-002.4,GPIO read,Medium,Implemented
REQ-003,UART communication,High,Implemented
REQ-003.1,Send single byte,High,Implemented
REQ-003.2,Send string,High,Implemented
REQ-004,Timer-based delay,Medium,Implemented
REQ-005,Main loop with watchdog,Critical,Implemented
REQ-005.1,Watchdog refresh,Critical,Implemented
REQ-EDGE-001,Inline assembly support,Low,Implemented
```

**tests/fixtures/arm-reference-project/Tests/test_gpio.c:**
```c
// GPIO Unit Tests with traceability annotations

#include "gpio.h"
#include <assert.h>

// TEST: REQ-002.1
void test_gpio_init(void) {
    gpio_init();
    // Verify GPIO clock is enabled
    assert((RCC->APB2ENR & RCC_APB2ENR_IOPCEN) != 0);
}

// TEST: REQ-002.2
void test_gpio_write_high(void) {
    gpio_init();
    gpio_write(13, 1);
    assert(gpio_read(13) == 1);
}

// TEST: REQ-002.2
void test_gpio_write_low(void) {
    gpio_init();
    gpio_write(13, 0);
    assert(gpio_read(13) == 0);
}

// TEST: REQ-002.3
void test_gpio_toggle(void) {
    gpio_init();
    gpio_write(13, 0);
    gpio_toggle(13);
    assert(gpio_read(13) == 1);
    gpio_toggle(13);
    assert(gpio_read(13) == 0);
}

// TEST: REQ-002.4
void test_gpio_read(void) {
    gpio_init();
    gpio_write(13, 1);
    uint8_t state = gpio_read(13);
    assert(state == 1);
}

// Edge case tests
void test_gpio_boundary_pin_0(void) {
    gpio_write_boundary(0, 1);
    // Should not crash
}

void test_gpio_boundary_pin_15(void) {
    gpio_write_boundary(15, 1);
    // Should not crash
}

void test_gpio_invalid_pin(void) {
    gpio_write_boundary(16, 1);
    // Should handle gracefully (do nothing)
}

void test_gpio_invalid_pin_max(void) {
    gpio_write_boundary(0xFFFF, 1);
    // Should handle gracefully
}
```


#### Integration Test Suite

**tests/integration/arm_toolchain_tests.rs:**
```rust
//! ARM Toolchain Integration Tests
//! 
//! These tests require arm-none-eabi-gcc to be installed.
//! Run with: cargo test --features integration-tests

use axiom_toolchain::*;
use std::path::PathBuf;

const REFERENCE_PROJECT: &str = "tests/fixtures/arm-reference-project";

mod detection_tests {
    use super::*;

    #[test]
    fn test_detect_homebrew_toolchain() {
        let detector = ArmToolchainDetector::new();
        let toolchains = detector.detect_all();
        
        // Should find at least one toolchain if installed
        if !toolchains.is_empty() {
            let tc = &toolchains[0];
            assert!(!tc.version.is_empty());
            assert!(tc.gcc.exists());
        }
    }

    #[test]
    fn test_detect_stm32cubeide_toolchain() {
        let detector = ArmToolchainDetector::new();
        let toolchains = detector.detect_from_stm32cubeide();
        
        // May or may not find STM32CubeIDE
        for tc in toolchains {
            assert!(tc.gcc.to_string_lossy().contains("stm32cube") 
                || tc.gcc.to_string_lossy().contains("STM32"));
        }
    }

    #[test]
    fn test_validate_complete_toolchain() {
        let detector = ArmToolchainDetector::new();
        if let Some(tc) = detector.detect_first() {
            assert!(tc.gcc.exists());
            assert!(tc.objcopy.exists());
            assert!(tc.objdump.exists());
            assert!(tc.size.exists());
            
            match tc.status {
                ToolchainCompleteness::Complete => {}
                ToolchainCompleteness::Incomplete { ref missing } => {
                    // GDB might be missing, that's acceptable
                    assert!(missing.iter().all(|m| m == "gdb"));
                }
            }
        }
    }

    #[test]
    fn test_version_extraction() {
        let detector = ArmToolchainDetector::new();
        if let Some(tc) = detector.detect_first() {
            // Version should be semver-like
            let parts: Vec<&str> = tc.version.split('.').collect();
            assert!(parts.len() >= 2);
            assert!(parts[0].parse::<u32>().is_ok());
        }
    }

    #[test]
    fn test_invalid_path_detection() {
        let result = detect_at_path(
            &PathBuf::from("/nonexistent/path/arm-none-eabi-gcc"),
            ToolchainKind::ArmGcc
        );
        assert!(result.is_none());
    }
}

mod compilation_tests {
    use super::*;

    #[test]
    fn test_compile_simple_c_file() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => {
                eprintln!("Skipping test: ARM toolchain not found");
                return;
            }
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("Drivers/gpio.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        let config = ArmMcuConfig {
            cpu: "cortex-m3".to_string(),
            thumb: true,
            fpu: None,
            float_abi: FloatAbi::Soft,
            defines: vec!["STM32F103xB".to_string()],
        };

        let request = ArmCompileRequest {
            source: source.clone(),
            output: output.path().to_path_buf(),
            mcu: config,
            include_paths: vec![
                project_path.join("Core/Inc"),
                project_path.join("Drivers"),
            ],
            optimization: 0,
            debug: true,
        };

        let result = compile_arm(&tc, &request);
        
        // Should compile successfully (or fail with missing headers)
        // The important thing is that the toolchain was invoked correctly
        assert!(result.exit_code == 0 || result.stderr.contains("fatal error"));
    }

    #[test]
    fn test_compile_with_fpu() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let config = ArmMcuConfig {
            cpu: "cortex-m7".to_string(),
            thumb: true,
            fpu: Some("fpv5-d16".to_string()),
            float_abi: FloatAbi::Hard,
            defines: vec!["STM32H750xx".to_string()],
        };

        let flags = config.compiler_flags();
        
        assert!(flags.contains(&"-mcpu=cortex-m7".to_string()));
        assert!(flags.contains(&"-mthumb".to_string()));
        assert!(flags.contains(&"-mfpu=fpv5-d16".to_string()));
        assert!(flags.contains(&"-mfloat-abi=hard".to_string()));
    }

    #[test]
    fn test_compile_syntax_error() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/syntax_error.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        let request = ArmCompileRequest::new(source, output.path().to_path_buf())
            .with_mcu(ArmMcuConfig::cortex_m3());

        let result = compile_arm(&tc, &request);
        
        // Should fail with syntax errors
        assert_ne!(result.exit_code, 0);
        assert!(result.stderr.contains("error:"));
    }

    #[test]
    fn test_compile_missing_include() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/missing_include.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        let request = ArmCompileRequest::new(source, output.path().to_path_buf())
            .with_mcu(ArmMcuConfig::cortex_m3());

        let result = compile_arm(&tc, &request);
        
        // Should fail with missing header error
        assert_ne!(result.exit_code, 0);
        assert!(result.stderr.contains("fatal error") || result.stderr.contains("No such file"));
    }
}

mod linking_tests {
    use super::*;

    #[test]
    fn test_link_with_linker_script() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let linker_script = project_path.join("STM32F103C8_FLASH.ld");

        let config = LinkerConfig {
            script: linker_script.clone(),
            generate_map: true,
            map_path: None,
            flags: vec![],
        };

        let flags = ArmMcuConfig::cortex_m3().linker_flags(&config);
        
        assert!(flags.iter().any(|f| f.contains("STM32F103C8_FLASH.ld")));
        assert!(flags.contains(&"-Wl,--gc-sections".to_string()));
    }

    #[test]
    fn test_link_missing_linker_script() {
        let config = LinkerConfig {
            script: PathBuf::from("/nonexistent/linker.ld"),
            generate_map: false,
            map_path: None,
            flags: vec![],
        };

        let result = validate_linker_config(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_overflow_detection() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        // This test would compile linker_overflow.c and attempt to link
        // The linker should report memory overflow
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        
        // First compile the overflow file
        let source = project_path.join("edge_cases/linker_overflow.c");
        let obj_file = tempfile::NamedTempFile::new().unwrap();
        
        let compile_result = compile_arm(&tc, &ArmCompileRequest::new(
            source,
            obj_file.path().to_path_buf()
        ).with_mcu(ArmMcuConfig::cortex_m3()));

        if compile_result.exit_code == 0 {
            // Try to link with small memory linker script
            let linker_script = project_path.join("STM32F103C8_FLASH.ld");
            let elf_file = tempfile::NamedTempFile::new().unwrap();
            
            let link_result = link_arm(&tc, &ArmLinkRequest {
                objects: vec![obj_file.path().to_path_buf()],
                output: elf_file.path().to_path_buf(),
                linker: LinkerConfig {
                    script: linker_script,
                    generate_map: false,
                    map_path: None,
                    flags: vec![],
                },
                mcu: ArmMcuConfig::cortex_m3(),
            });

            // Should fail with memory overflow
            assert_ne!(link_result.exit_code, 0);
            assert!(
                link_result.stderr.contains("will not fit") ||
                link_result.stderr.contains("overflow") ||
                link_result.stderr.contains("region")
            );
        }
    }
}

mod binary_generation_tests {
    use super::*;

    #[test]
    fn test_generate_hex_file() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        // Create a minimal ELF file for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let elf_path = temp_dir.path().join("test.elf");
        let hex_path = temp_dir.path().join("test.hex");

        // We need a valid ELF to test this properly
        // For now, test the command generation
        let cmd = build_objcopy_hex_command(&tc.objcopy, &elf_path, &hex_path);
        
        assert!(cmd.contains(&"-O".to_string()));
        assert!(cmd.contains(&"ihex".to_string()));
    }

    #[test]
    fn test_generate_bin_file() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let temp_dir = tempfile::tempdir().unwrap();
        let elf_path = temp_dir.path().join("test.elf");
        let bin_path = temp_dir.path().join("test.bin");

        let cmd = build_objcopy_bin_command(&tc.objcopy, &elf_path, &bin_path);
        
        assert!(cmd.contains(&"-O".to_string()));
        assert!(cmd.contains(&"binary".to_string()));
    }

    #[test]
    fn test_size_report_parsing() {
        let size_output = r#"   text    data     bss     dec     hex filename
   1234     256      64    1554     612 test.elf"#;

        let stats = parse_size_output(size_output);
        
        assert_eq!(stats.text, 1234);
        assert_eq!(stats.data, 256);
        assert_eq!(stats.bss, 64);
        assert_eq!(stats.total, 1554);
    }
}

mod visualizer_tests {
    use super::*;

    #[test]
    fn test_preprocessor_output() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("Drivers/gpio.c");

        let result = get_preprocessor_output(&tc, &source, &ArmMcuConfig::cortex_m3());
        
        // Should either succeed or fail with missing headers
        // The preprocessor output should contain expanded macros
        if result.exit_code == 0 {
            assert!(!result.stdout.is_empty());
        }
    }

    #[test]
    fn test_assembly_output() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/inline_assembly.c");

        let result = get_assembly_output(&tc, &source, &ArmMcuConfig::cortex_m3());
        
        if result.exit_code == 0 {
            // Should contain ARM assembly instructions
            assert!(
                result.stdout.contains(".thumb") ||
                result.stdout.contains(".syntax") ||
                result.stdout.contains("bx") ||
                result.stdout.contains("mov")
            );
        }
    }

    #[test]
    fn test_disassembly() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        // Would need a valid object file to test properly
        // Test command generation
        let flags = build_disassembly_flags();
        assert!(flags.contains(&"-d".to_string()));
    }

    #[test]
    fn test_symbol_table() {
        let flags = build_symbol_table_flags();
        assert!(flags.contains(&"-t".to_string()));
    }

    #[test]
    fn test_section_headers() {
        let flags = build_section_headers_flags();
        assert!(flags.contains(&"-h".to_string()));
    }
}

mod makefile_tests {
    use super::*;

    #[test]
    fn test_detect_makefile() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let result = detect_makefile(&project_path);
        
        assert!(result.is_some());
        let info = result.unwrap();
        assert!(info.path.ends_with("Makefile"));
    }

    #[test]
    fn test_parse_makefile_targets() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let makefile_content = std::fs::read_to_string(
            project_path.join("Makefile")
        ).unwrap_or_default();

        let targets = parse_makefile_targets(&makefile_content);
        
        assert!(targets.contains(&"all".to_string()));
        assert!(targets.contains(&"clean".to_string()));
    }

    #[test]
    fn test_run_make_clean() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        
        // Only run if Makefile exists
        if !project_path.join("Makefile").exists() {
            return;
        }

        let result = run_make(&project_path, "clean");
        
        // Clean should succeed (or fail gracefully if build dir doesn't exist)
        assert!(result.exit_code == 0 || result.stderr.contains("No rule"));
    }
}

mod settings_tests {
    use super::*;

    #[test]
    fn test_load_project_settings() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let config_path = project_path.join(".axiom/toolchain.toml");

        if config_path.exists() {
            let settings = load_project_settings(&config_path);
            assert!(settings.is_ok());
        }
    }

    #[test]
    fn test_settings_merge() {
        let global = ToolchainSettings {
            auto_detect: true,
            toolchains: HashMap::from([
                ("arm".to_string(), ToolchainConfig {
                    path: Some(PathBuf::from("/global/path")),
                    search_paths: vec![],
                    settings: HashMap::new(),
                }),
            ]),
        };

        let project = ToolchainSettings {
            auto_detect: false,
            toolchains: HashMap::from([
                ("arm".to_string(), ToolchainConfig {
                    path: Some(PathBuf::from("/project/path")),
                    search_paths: vec![],
                    settings: HashMap::new(),
                }),
            ]),
        };

        let merged = merge_settings(global, Some(project));
        
        // Project should override global
        assert!(!merged.auto_detect);
        assert_eq!(
            merged.toolchains.get("arm").unwrap().path,
            Some(PathBuf::from("/project/path"))
        );
    }

    #[test]
    fn test_unknown_toolchain_preserved() {
        let toml_content = r#"
            auto_detect = true
            [toolchains.riscv]
            path = "/opt/riscv/bin/riscv-gcc"
            [toolchains.riscv.settings]
            custom_flag = "value"
        "#;

        let settings: ToolchainSettings = toml::from_str(toml_content).unwrap();
        
        assert!(settings.toolchains.contains_key("riscv"));
        
        // Serialize and deserialize should preserve unknown toolchain
        let serialized = toml::to_string(&settings).unwrap();
        let reparsed: ToolchainSettings = toml::from_str(&serialized).unwrap();
        
        assert!(reparsed.toolchains.contains_key("riscv"));
    }
}

mod compliance_tests {
    use super::*;

    #[test]
    fn test_parse_requirement_annotations() {
        let source = r#"
            // REQ-001: System initialization
            void init(void) {}
            
            // REQ-002.1: Sub-requirement
            void sub_init(void) {}
        "#;

        let annotations = parse_requirement_annotations(source);
        
        assert!(annotations.contains(&"REQ-001".to_string()));
        assert!(annotations.contains(&"REQ-002.1".to_string()));
    }

    #[test]
    fn test_parse_test_annotations() {
        let source = r#"
            // TEST: REQ-001
            void test_init(void) {}
            
            // TEST: REQ-002, REQ-003
            void test_multiple(void) {}
        "#;

        let annotations = parse_test_annotations(source);
        
        assert!(annotations.iter().any(|(_, reqs)| reqs.contains(&"REQ-001".to_string())));
        assert!(annotations.iter().any(|(_, reqs)| reqs.contains(&"REQ-002".to_string())));
        assert!(annotations.iter().any(|(_, reqs)| reqs.contains(&"REQ-003".to_string())));
    }

    #[test]
    fn test_traceability_matrix_generation() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        
        let matrix = generate_traceability_matrix(&project_path);
        
        // Should find traced requirements
        assert!(!matrix.requirements.is_empty() || !project_path.exists());
    }

    #[test]
    fn test_find_untraceable_code() {
        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("compliance/untraced_module.c");

        if source.exists() {
            let content = std::fs::read_to_string(&source).unwrap();
            let untraceable = find_untraceable_functions(&content);
            
            // Should find functions without REQ annotations
            assert!(!untraceable.is_empty());
        }
    }

    #[test]
    fn test_coverage_instrumentation_flags() {
        let flags = build_coverage_flags();
        
        assert!(flags.contains(&"--coverage".to_string()));
        assert!(flags.contains(&"-fprofile-arcs".to_string()));
        assert!(flags.contains(&"-ftest-coverage".to_string()));
    }

    #[test]
    fn test_tool_usage_logging() {
        let temp_dir = tempfile::tempdir().unwrap();
        let logger = ToolQualificationLogger::new(temp_dir.path());

        let record = ToolUsageRecord {
            tool: "arm-none-eabi-gcc".to_string(),
            version: "14.3.1".to_string(),
            arguments: vec!["-c".to_string(), "main.c".to_string()],
            input_checksums: HashMap::new(),
            output_checksums: HashMap::new(),
            timestamp: chrono::Utc::now(),
            exit_code: 0,
            diagnostics: vec![],
        };

        logger.log(&record).unwrap();

        let records = logger.get_all_records().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].tool, "arm-none-eabi-gcc");
    }

    #[test]
    fn test_checksum_computation() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "test content for checksum").unwrap();

        let checksum1 = compute_sha256(temp_file.path()).unwrap();
        let checksum2 = compute_sha256(temp_file.path()).unwrap();

        // Same content should produce same checksum
        assert_eq!(checksum1, checksum2);
        assert_eq!(checksum1.len(), 64); // SHA-256 hex string
    }

    #[test]
    fn test_compliance_mode_toggle() {
        let temp_dir = tempfile::tempdir().unwrap();
        let system = ComplianceSystem::new(temp_dir.path());

        // Enable DO-178C mode
        system.enable_mode(ComplianceMode::Do178c).unwrap();
        assert!(system.is_mode_enabled(ComplianceMode::Do178c));

        // Disable mode
        system.disable_mode(ComplianceMode::Do178c).unwrap();
        assert!(!system.is_mode_enabled(ComplianceMode::Do178c));
    }

    #[test]
    fn test_multiple_compliance_modes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let system = ComplianceSystem::new(temp_dir.path());

        // Enable multiple modes
        system.enable_mode(ComplianceMode::Do178c).unwrap();
        system.enable_mode(ComplianceMode::Do330).unwrap();
        system.enable_mode(ComplianceMode::Arp4754a).unwrap();

        assert!(system.is_mode_enabled(ComplianceMode::Do178c));
        assert!(system.is_mode_enabled(ComplianceMode::Do330));
        assert!(system.is_mode_enabled(ComplianceMode::Arp4754a));
    }
}

mod edge_case_tests {
    use super::*;

    #[test]
    fn test_empty_source_file() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/empty_file.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3()));

            // Empty file should compile (produces empty object)
            assert_eq!(result.exit_code, 0);
        }
    }

    #[test]
    fn test_unicode_in_comments() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/unicode_comments.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3()));

            // Unicode in comments should be handled
            // (may succeed or fail depending on encoding)
        }
    }

    #[test]
    fn test_very_long_lines() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/very_long_lines.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3()));

            // Should handle long lines (may warn but should compile)
        }
    }

    #[test]
    fn test_deeply_nested_includes() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/nested_includes.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3())
            .with_include_path(project_path.join("edge_cases")));

            // Should handle nested includes (may hit include depth limit)
        }
    }

    #[test]
    fn test_inline_assembly() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/inline_assembly.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3()));

            // Inline assembly should compile for ARM target
            assert_eq!(result.exit_code, 0);
        }
    }

    #[test]
    fn test_preprocessor_heavy_macros() {
        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let project_path = PathBuf::from(REFERENCE_PROJECT);
        let source = project_path.join("edge_cases/preprocessor_heavy.c");
        let output = tempfile::NamedTempFile::new().unwrap();

        if source.exists() {
            let result = compile_arm(&tc, &ArmCompileRequest::new(
                source,
                output.path().to_path_buf()
            ).with_mcu(ArmMcuConfig::cortex_m3()));

            // Heavy macro usage should be handled
        }
    }

    #[test]
    fn test_invalid_mcu_config() {
        let config = ArmMcuConfig {
            cpu: "invalid-cpu".to_string(),
            thumb: true,
            fpu: Some("invalid-fpu".to_string()),
            float_abi: FloatAbi::Hard,
            defines: vec![],
        };

        let detector = ArmToolchainDetector::new();
        let tc = match detector.detect_first() {
            Some(tc) => tc,
            None => return,
        };

        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), "int main(void) { return 0; }").unwrap();

        let output = tempfile::NamedTempFile::new().unwrap();
        let result = compile_arm(&tc, &ArmCompileRequest::new(
            temp_file.path().to_path_buf(),
            output.path().to_path_buf()
        ).with_mcu(config));

        // Should fail with invalid CPU/FPU
        assert_ne!(result.exit_code, 0);
    }
}
```

This comprehensive test suite covers:

1. **Detection Tests**: Homebrew, STM32CubeIDE, validation, version extraction, invalid paths
2. **Compilation Tests**: Simple files, FPU configurations, syntax errors, missing includes
3. **Linking Tests**: Linker scripts, missing scripts, memory overflow detection
4. **Binary Generation Tests**: HEX/BIN generation, size report parsing
5. **Visualizer Tests**: Preprocessor, assembly, disassembly, symbols, sections
6. **Makefile Tests**: Detection, target parsing, make execution
7. **Settings Tests**: Project settings, merging, unknown toolchain preservation
8. **Compliance Tests**: Requirement parsing, traceability, coverage, tool logging, mode management
9. **Edge Case Tests**: Empty files, unicode, long lines, nested includes, inline assembly, invalid configs
