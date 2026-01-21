// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! ARM toolchain command handlers.

use axiom_toolchain::{
    compile_arm, detect_arm_toolchains, detect_makefile, generate_bin, generate_hex,
    get_assembly_output, get_disassembly, get_preprocessor_output, get_section_headers,
    get_size_stats, get_symbol_table, link_arm, run_make, ArmCompileRequest, ArmLinkRequest,
    ArmMcuConfig, ArmToolchainSuite, CompileResult, LinkResult, LinkerConfig,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::state::AppState;

/// Serializable version of ArmToolchainSuite for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmToolchainSuiteResponse {
    pub gcc: String,
    pub gxx: String,
    pub as_: String,
    pub ld: String,
    pub objcopy: String,
    pub objdump: String,
    pub size: String,
    pub gdb: String,
    pub version: String,
    pub source: String,
    pub completeness: String,
    pub missing: Vec<String>,
}

impl From<ArmToolchainSuite> for ArmToolchainSuiteResponse {
    fn from(suite: ArmToolchainSuite) -> Self {
        let (completeness, missing) = match suite.completeness {
            axiom_toolchain::ToolchainCompleteness::Complete => {
                ("Complete".to_string(), Vec::new())
            }
            axiom_toolchain::ToolchainCompleteness::Incomplete { missing } => {
                ("Incomplete".to_string(), missing)
            }
        };

        let source = match suite.source {
            axiom_toolchain::ToolchainSource::Homebrew => "Homebrew",
            axiom_toolchain::ToolchainSource::Stm32CubeIde => "STM32CubeIDE",
            axiom_toolchain::ToolchainSource::SystemPath => "System",
            axiom_toolchain::ToolchainSource::Manual => "Manual",
        };

        Self {
            gcc: suite.gcc.display().to_string(),
            gxx: suite.gxx.display().to_string(),
            as_: suite.as_.display().to_string(),
            ld: suite.ld.display().to_string(),
            objcopy: suite.objcopy.display().to_string(),
            objdump: suite.objdump.display().to_string(),
            size: suite.size.display().to_string(),
            gdb: suite.gdb.display().to_string(),
            version: suite.version,
            source: source.to_string(),
            completeness,
            missing,
        }
    }
}

/// Serializable MCU configuration for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmMcuConfigRequest {
    pub cpu: String,
    pub thumb: bool,
    pub fpu: Option<String>,
    pub float_abi: String,
    pub defines: Vec<String>,
}

/// Serializable compile request for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmCompileRequestData {
    pub source: String,
    pub output: String,
    pub mcu: ArmMcuConfigRequest,
    pub include_paths: Vec<String>,
    pub optimization: u8,
    pub debug: bool,
}

/// Serializable link request for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmLinkRequestData {
    pub objects: Vec<String>,
    pub output: String,
    pub linker_script: String,
    pub generate_map: bool,
    pub map_path: Option<String>,
    pub mcu: ArmMcuConfigRequest,
}

/// Serializable binary output configuration for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryOutputConfigData {
    pub hex: bool,
    pub bin: bool,
    pub size_report: bool,
}

/// Serializable binary result for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryResultData {
    pub hex_path: Option<String>,
    pub bin_path: Option<String>,
    pub size_stats: Option<axiom_toolchain::SizeStats>,
}

/// Serializable makefile info for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakefileInfoData {
    pub path: String,
    pub targets: Vec<String>,
}

/// Serializable make result for Tauri commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakeResultData {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Detect all ARM toolchains.
#[tauri::command]
pub fn detect_arm_toolchains_cmd(
    _state: State<AppState>,
) -> Result<Vec<ArmToolchainSuiteResponse>, String> {
    let toolchains = detect_arm_toolchains();
    Ok(toolchains.into_iter().map(Into::into).collect())
}

/// Validate a toolchain at a specific path.
#[tauri::command]
pub fn validate_toolchain_path(
    _state: State<AppState>,
    path: String,
) -> Result<ArmToolchainSuiteResponse, String> {
    let gcc_path = PathBuf::from(&path);

    if !gcc_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // Try to detect toolchain at this path
    let toolchains = detect_arm_toolchains();

    for suite in toolchains {
        if suite.gcc == gcc_path {
            return Ok(suite.into());
        }
    }

    Err(format!("No valid ARM toolchain found at: {}", path))
}

/// Compile ARM source code.
#[tauri::command]
pub fn compile_arm_cmd(
    _state: State<AppState>,
    request: ArmCompileRequestData,
    gcc_path: String,
) -> Result<CompileResult, String> {
    let mcu = ArmMcuConfig {
        cpu: request.mcu.cpu,
        thumb: request.mcu.thumb,
        fpu: request.mcu.fpu.unwrap_or_default(),
        float_abi: match request.mcu.float_abi.as_str() {
            "hard" => axiom_toolchain::FloatAbi::Hard,
            "soft" => axiom_toolchain::FloatAbi::Soft,
            "softfp" => axiom_toolchain::FloatAbi::SoftFp,
            _ => return Err(format!("Invalid float ABI: {}", request.mcu.float_abi)),
        },
        defines: request.mcu.defines,
    };

    let mut compile_request = ArmCompileRequest::new(
        PathBuf::from(request.source),
        PathBuf::from(request.output),
        mcu,
    )
    .with_optimization(request.optimization)
    .with_debug(request.debug);

    for path in request.include_paths {
        compile_request = compile_request.with_include_path(path);
    }

    let result = compile_arm(&PathBuf::from(gcc_path), &compile_request);
    Ok(result)
}

/// Link ARM object files.
#[tauri::command]
pub fn link_arm_cmd(
    _state: State<AppState>,
    request: ArmLinkRequestData,
    gcc_path: String,
) -> Result<LinkResult, String> {
    let mcu = ArmMcuConfig {
        cpu: request.mcu.cpu,
        thumb: request.mcu.thumb,
        fpu: request.mcu.fpu.unwrap_or_default(),
        float_abi: match request.mcu.float_abi.as_str() {
            "hard" => axiom_toolchain::FloatAbi::Hard,
            "soft" => axiom_toolchain::FloatAbi::Soft,
            "softfp" => axiom_toolchain::FloatAbi::SoftFp,
            _ => return Err(format!("Invalid float ABI: {}", request.mcu.float_abi)),
        },
        defines: request.mcu.defines,
    };

    let mut linker = LinkerConfig::new(&request.linker_script);
    if request.generate_map {
        if let Some(map_path) = request.map_path {
            linker = linker.with_map(map_path);
        }
    }

    let link_request = ArmLinkRequest::new(
        request.objects.into_iter().map(PathBuf::from).collect(),
        PathBuf::from(request.output),
        linker,
        mcu,
    );

    let result = link_arm(&PathBuf::from(gcc_path), &link_request);
    Ok(result)
}

/// Generate binary files from ELF.
#[tauri::command]
pub fn generate_binary_cmd(
    _state: State<AppState>,
    elf_path: String,
    config: BinaryOutputConfigData,
    objcopy_path: String,
    size_path: String,
) -> Result<BinaryResultData, String> {
    let elf = PathBuf::from(&elf_path);
    let objcopy = PathBuf::from(&objcopy_path);
    let size = PathBuf::from(&size_path);

    let mut result = BinaryResultData {
        hex_path: None,
        bin_path: None,
        size_stats: None,
    };

    // Generate HEX if requested
    if config.hex {
        let hex_path = elf.with_extension("hex");
        generate_hex(&objcopy, &elf, &hex_path)?;
        result.hex_path = Some(hex_path.display().to_string());
    }

    // Generate BIN if requested
    if config.bin {
        let bin_path = elf.with_extension("bin");
        generate_bin(&objcopy, &elf, &bin_path)?;
        result.bin_path = Some(bin_path.display().to_string());
    }

    // Get size stats if requested
    if config.size_report {
        let stats = get_size_stats(&size, &elf)?;
        result.size_stats = Some(stats);
    }

    Ok(result)
}

/// Get preprocessor output for a source file.
#[tauri::command]
pub fn get_preprocessor_output_cmd(
    _state: State<AppState>,
    source: String,
    mcu: ArmMcuConfigRequest,
    gcc_path: String,
) -> Result<String, String> {
    let mcu_config = ArmMcuConfig {
        cpu: mcu.cpu,
        thumb: mcu.thumb,
        fpu: mcu.fpu.unwrap_or_default(),
        float_abi: match mcu.float_abi.as_str() {
            "hard" => axiom_toolchain::FloatAbi::Hard,
            "soft" => axiom_toolchain::FloatAbi::Soft,
            "softfp" => axiom_toolchain::FloatAbi::SoftFp,
            _ => return Err(format!("Invalid float ABI: {}", mcu.float_abi)),
        },
        defines: mcu.defines,
    };

    let flags = mcu_config.compiler_flags();
    get_preprocessor_output(&PathBuf::from(gcc_path), &PathBuf::from(source), &flags)
}

/// Get assembly output for a source file.
#[tauri::command]
pub fn get_assembly_output_cmd(
    _state: State<AppState>,
    source: String,
    output: String,
    mcu: ArmMcuConfigRequest,
    gcc_path: String,
) -> Result<String, String> {
    let mcu_config = ArmMcuConfig {
        cpu: mcu.cpu,
        thumb: mcu.thumb,
        fpu: mcu.fpu.unwrap_or_default(),
        float_abi: match mcu.float_abi.as_str() {
            "hard" => axiom_toolchain::FloatAbi::Hard,
            "soft" => axiom_toolchain::FloatAbi::Soft,
            "softfp" => axiom_toolchain::FloatAbi::SoftFp,
            _ => return Err(format!("Invalid float ABI: {}", mcu.float_abi)),
        },
        defines: mcu.defines,
    };

    let flags = mcu_config.compiler_flags();
    get_assembly_output(
        &PathBuf::from(gcc_path),
        &PathBuf::from(source),
        &PathBuf::from(output),
        &flags,
    )
}

/// Get disassembly of an object file.
#[tauri::command]
pub fn get_disassembly_cmd(
    _state: State<AppState>,
    object_file: String,
    objdump_path: String,
) -> Result<String, String> {
    get_disassembly(&PathBuf::from(objdump_path), &PathBuf::from(object_file))
}

/// Get symbol table from an object file.
#[tauri::command]
pub fn get_symbol_table_cmd(
    _state: State<AppState>,
    object_file: String,
    objdump_path: String,
) -> Result<String, String> {
    get_symbol_table(&PathBuf::from(objdump_path), &PathBuf::from(object_file))
}

/// Get section headers from an object file.
#[tauri::command]
pub fn get_section_headers_cmd(
    _state: State<AppState>,
    object_file: String,
    objdump_path: String,
) -> Result<String, String> {
    get_section_headers(&PathBuf::from(objdump_path), &PathBuf::from(object_file))
}

/// Detect Makefile in a directory.
#[tauri::command]
pub fn detect_makefile_cmd(
    _state: State<AppState>,
    project_path: String,
) -> Result<Option<MakefileInfoData>, String> {
    let path = PathBuf::from(project_path);

    if let Some(info) = detect_makefile(&path) {
        Ok(Some(MakefileInfoData {
            path: info.path.display().to_string(),
            targets: info.targets,
        }))
    } else {
        Ok(None)
    }
}

/// Run make with a specific target.
#[tauri::command]
pub fn run_make_cmd(
    _state: State<AppState>,
    project_path: String,
    target: String,
    toolchain_prefix: Option<String>,
) -> Result<MakeResultData, String> {
    let path = PathBuf::from(project_path);
    let prefix = toolchain_prefix.map(PathBuf::from);

    let result = run_make(&path, &target, prefix.as_deref());
    Ok(MakeResultData {
        exit_code: result.exit_code,
        stdout: result.stdout,
        stderr: result.stderr,
    })
}

/// Save ARM toolchain settings.
#[tauri::command]
pub fn save_arm_toolchain_settings_cmd(
    state: State<AppState>,
    gcc_path: Option<String>,
    mcu_config: ArmMcuConfigRequest,
    scope: String,
) -> Result<(), String> {
    use axiom_settings::save_default;
    use std::collections::HashMap;

    let mut settings = state.settings.lock().map_err(|e| e.to_string())?;

    // Update ARM toolchain path
    if let Some(path) = gcc_path {
        settings.toolchains.arm_gcc_path = Some(PathBuf::from(path));
    }

    // Create ARM toolchain config in the generic toolchains map
    let mut arm_settings = HashMap::new();
    arm_settings.insert(
        "cpu".to_string(),
        toml::Value::String(mcu_config.cpu.clone()),
    );
    arm_settings.insert(
        "thumb".to_string(),
        toml::Value::Boolean(mcu_config.thumb),
    );
    if let Some(fpu) = &mcu_config.fpu {
        arm_settings.insert("fpu".to_string(), toml::Value::String(fpu.clone()));
    }
    arm_settings.insert(
        "float_abi".to_string(),
        toml::Value::String(mcu_config.float_abi.clone()),
    );
    arm_settings.insert(
        "defines".to_string(),
        toml::Value::Array(
            mcu_config
                .defines
                .iter()
                .map(|d| toml::Value::String(d.clone()))
                .collect(),
        ),
    );

    let arm_config = axiom_settings::ToolchainConfig {
        path: settings.toolchains.arm_gcc_path.clone(),
        search_paths: vec![],
        settings: arm_settings,
    };

    settings
        .toolchains
        .toolchains
        .insert("arm".to_string(), arm_config);

    // Save based on scope
    match scope.as_str() {
        "global" => {
            save_default(&*settings).map_err(|e| e.to_string())?;
        }
        "project" => {
            // For project scope, we need the project path
            // For now, save to default location
            // TODO: Implement project-specific settings path
            save_default(&*settings).map_err(|e| e.to_string())?;
        }
        _ => return Err(format!("Invalid scope: {}", scope)),
    }

    Ok(())
}
