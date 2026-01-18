// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Toolchain command handlers.

use crate::state::AppState;
use axiom_toolchain::{CompileRequest, CompileResult, DetectedToolchain, ToolchainKind};
use std::path::PathBuf;
use tauri::State;

/// Detect all available toolchains.
#[tauri::command]
pub fn detect_toolchains(state: State<AppState>) -> Result<Vec<DetectedToolchain>, String> {
    let detected = axiom_toolchain::detect_all();

    // Update state
    let mut toolchains = state.toolchains.lock().map_err(|e| e.to_string())?;
    *toolchains = detected.clone();

    Ok(detected)
}

/// Get currently detected toolchains.
#[tauri::command]
pub fn get_toolchains(state: State<AppState>) -> Result<Vec<DetectedToolchain>, String> {
    let toolchains = state.toolchains.lock().map_err(|e| e.to_string())?;
    Ok(toolchains.clone())
}

/// Compile a file.
#[tauri::command]
pub fn compile_file(
    state: State<AppState>,
    source: String,
    output: String,
    toolchain_kind: Option<String>,
) -> Result<CompileResult, String> {
    let toolchains = state.toolchains.lock().map_err(|e| e.to_string())?;

    // Find requested toolchain or default to Clang
    let kind = toolchain_kind
        .as_ref()
        .and_then(|k| match k.to_lowercase().as_str() {
            "clang" => Some(ToolchainKind::Clang),
            "gcc" => Some(ToolchainKind::Gcc),
            "armgcc" | "arm-gcc" => Some(ToolchainKind::ArmGcc),
            _ => None,
        })
        .unwrap_or(ToolchainKind::Clang);

    let toolchain = toolchains
        .iter()
        .find(|t| t.kind == kind)
        .ok_or_else(|| format!("Toolchain {:?} not found", kind))?;

    let request = CompileRequest::new(PathBuf::from(source), PathBuf::from(output));
    let result = axiom_toolchain::compile(toolchain, &request);

    Ok(result)
}

/// Get compile command without executing (dry run).
#[tauri::command]
pub fn compile_dry_run(
    state: State<AppState>,
    source: String,
    output: String,
    toolchain_kind: Option<String>,
) -> Result<String, String> {
    let toolchains = state.toolchains.lock().map_err(|e| e.to_string())?;

    let kind = toolchain_kind
        .as_ref()
        .and_then(|k| match k.to_lowercase().as_str() {
            "clang" => Some(ToolchainKind::Clang),
            "gcc" => Some(ToolchainKind::Gcc),
            "armgcc" | "arm-gcc" => Some(ToolchainKind::ArmGcc),
            _ => None,
        })
        .unwrap_or(ToolchainKind::Clang);

    let toolchain = toolchains
        .iter()
        .find(|t| t.kind == kind)
        .ok_or_else(|| format!("Toolchain {:?} not found", kind))?;

    let request = CompileRequest::new(PathBuf::from(source), PathBuf::from(output));
    let command = axiom_toolchain::dry_run(toolchain, &request);

    Ok(command)
}
