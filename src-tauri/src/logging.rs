// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Application logging with frontend event emission.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tauri::{AppHandle, Emitter};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/// Log level for console messages.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// A log entry for the mini-console.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub timestamp: u64,
}

impl LogEntry {
    pub fn new(level: LogLevel, source: &str, message: impl Into<String>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            level,
            message: message.into(),
            source: source.to_string(),
            timestamp,
        }
    }
}

/// Initialize the logger with the app handle.
pub fn init(handle: AppHandle) {
    let _ = APP_HANDLE.set(handle);
}

/// Emit a log entry to the frontend.
fn emit_log(entry: LogEntry) {
    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit("backend-log", entry);
    }
}

/// Log a debug message.
pub fn debug(source: &str, message: impl Into<String>) {
    let entry = LogEntry::new(LogLevel::Debug, source, message);
    emit_log(entry);
}

/// Log an info message.
pub fn info(source: &str, message: impl Into<String>) {
    let entry = LogEntry::new(LogLevel::Info, source, message);
    emit_log(entry);
}

/// Log a warning message.
pub fn warn(source: &str, message: impl Into<String>) {
    let entry = LogEntry::new(LogLevel::Warn, source, message);
    emit_log(entry);
}

/// Log an error message.
pub fn error(source: &str, message: impl Into<String>) {
    let entry = LogEntry::new(LogLevel::Error, source, message);
    emit_log(entry);
}

/// Convenience macros for logging.
#[macro_export]
macro_rules! log_debug {
    ($source:expr, $($arg:tt)*) => {
        $crate::logging::debug($source, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info {
    ($source:expr, $($arg:tt)*) => {
        $crate::logging::info($source, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($source:expr, $($arg:tt)*) => {
        $crate::logging::warn($source, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($source:expr, $($arg:tt)*) => {
        $crate::logging::error($source, format!($($arg)*))
    };
}
