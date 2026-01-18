// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Filesystem command handlers.

use crate::logging;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Directory entry information.
#[derive(Debug, Serialize, Deserialize)]
pub struct DirEntry {
    /// Entry name.
    pub name: String,
    /// Full path.
    pub path: String,
    /// Whether this is a directory.
    pub is_dir: bool,
    /// Whether this is a file.
    pub is_file: bool,
    /// File size in bytes (if file).
    pub size: Option<u64>,
}

/// Read directory contents.
#[tauri::command]
pub fn read_dir(path: String) -> Result<Vec<DirEntry>, String> {
    logging::debug("fs", &format!("read_dir: {}", path));
    let path = Path::new(&path);

    if !path.exists() {
        logging::warn("fs", "Path does not exist");
        return Err("Path does not exist".to_string());
    }

    if !path.is_dir() {
        logging::warn("fs", "Path is not a directory");
        return Err("Path is not a directory".to_string());
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;

        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            size: if metadata.is_file() {
                Some(metadata.len())
            } else {
                None
            },
        });
    }

    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    logging::debug("fs", &format!("read_dir: {} entries", entries.len()));
    Ok(entries)
}

/// Read file contents.
#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    logging::info("fs", &format!("read_file: {}", path));
    let result = fs::read_to_string(&path).map_err(|e| {
        logging::error("fs", &format!("read_file failed: {}", e));
        e.to_string()
    })?;
    logging::debug("fs", &format!("read_file: {} bytes", result.len()));
    Ok(result)
}

/// Write file contents.
#[tauri::command]
pub fn write_file(path: String, contents: String) -> Result<(), String> {
    logging::info("fs", &format!("write_file: {}", path));
    fs::write(&path, &contents).map_err(|e| {
        logging::error("fs", &format!("write_file failed: {}", e));
        e.to_string()
    })?;
    logging::debug("fs", &format!("write_file: {} bytes written", contents.len()));
    Ok(())
}
