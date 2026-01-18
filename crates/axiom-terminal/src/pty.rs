// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! PTY operations.

use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtyPair};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::os::unix::io::RawFd;

/// Terminal error type.
#[derive(Debug, thiserror::Error)]
pub enum TerminalError {
    #[error("PTY error: {0}")]
    Pty(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Lock error")]
    Lock,
}

/// PTY size dimensions.
#[derive(Debug, Clone, Copy)]
pub struct TerminalSize {
    pub rows: u16,
    pub cols: u16,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self { rows: 24, cols: 80 }
    }
}

impl From<TerminalSize> for PtySize {
    fn from(size: TerminalSize) -> Self {
        PtySize {
            rows: size.rows,
            cols: size.cols,
            pixel_width: 0,
            pixel_height: 0,
        }
    }
}

/// A PTY instance.
pub struct Pty {
    pair: PtyPair,
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    reader_fd: Option<RawFd>,
}

impl Pty {
    /// Create a new PTY with default shell.
    pub fn new(size: TerminalSize) -> Result<Self, TerminalError> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(size.into())
            .map_err(|e| TerminalError::Pty(e.to_string()))?;

        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| TerminalError::Pty(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| TerminalError::Pty(e.to_string()))?;

        // Get fd from master BEFORE any operations that might invalidate it
        let reader_fd = pair.master.as_raw_fd();

        Ok(Self {
            pair,
            reader: Arc::new(Mutex::new(reader)),
            writer: Arc::new(Mutex::new(writer)),
            reader_fd,
        })
    }

    /// Spawn a shell in the PTY.
    pub fn spawn_shell(&self) -> Result<(), TerminalError> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string());
        let mut cmd = CommandBuilder::new(&shell);
        
        // Set environment for color support
        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("CLICOLOR", "1");
        cmd.env("CLICOLOR_FORCE", "1");
        
        // For zsh: enable colors
        if shell.contains("zsh") {
            cmd.env("FORCE_COLOR", "1");
        }
        
        // Preserve important env vars
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }
        if let Ok(user) = std::env::var("USER") {
            cmd.env("USER", user);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
        if let Ok(lang) = std::env::var("LANG") {
            cmd.env("LANG", lang);
        }

        self.pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| TerminalError::Pty(e.to_string()))?;

        Ok(())
    }

    /// Spawn a specific command in the PTY.
    pub fn spawn_command(&self, program: &str, args: &[&str]) -> Result<(), TerminalError> {
        let mut cmd = CommandBuilder::new(program);
        for arg in args {
            cmd.arg(*arg);
        }

        self.pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| TerminalError::Pty(e.to_string()))?;

        Ok(())
    }

    /// Write to the PTY.
    pub fn write(&self, data: &[u8]) -> Result<usize, TerminalError> {
        let mut writer = self.writer.lock().map_err(|_| TerminalError::Lock)?;
        let n = writer.write(data)?;
        writer.flush()?;
        Ok(n)
    }

    /// Read from the PTY (blocking - waits for data).
    /// This is efficient because the OS handles waiting, no CPU cycles wasted.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, TerminalError> {
        let mut reader = self.reader.lock().map_err(|_| TerminalError::Lock)?;
        Ok(reader.read(buf)?)
    }
    
    /// Check if data is available without blocking.
    pub fn has_data(&self) -> bool {
        if let Some(fd) = self.reader_fd {
            let mut pollfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };
            unsafe {
                let result = libc::poll(&mut pollfd, 1, 0);
                result > 0 && (pollfd.revents & libc::POLLIN) != 0
            }
        } else {
            false
        }
    }

    /// Resize the PTY.
    pub fn resize(&self, size: TerminalSize) -> Result<(), TerminalError> {
        self.pair
            .master
            .resize(size.into())
            .map_err(|e| TerminalError::Pty(e.to_string()))?;
        Ok(())
    }

    /// Get a clone of the reader for async reading.
    pub fn reader(&self) -> Arc<Mutex<Box<dyn Read + Send>>> {
        self.reader.clone()
    }

    /// Get a clone of the writer for async writing.
    pub fn writer(&self) -> Arc<Mutex<Box<dyn Write + Send>>> {
        self.writer.clone()
    }
    
    /// Get the file descriptor for polling.
    pub fn get_fd(&self) -> Option<RawFd> {
        self.reader_fd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pty() {
        let pty = Pty::new(TerminalSize::default());
        assert!(pty.is_ok());
    }

    #[test]
    fn test_resize() {
        let pty = Pty::new(TerminalSize::default()).unwrap();
        let result = pty.resize(TerminalSize { rows: 40, cols: 120 });
        assert!(result.is_ok());
    }
}
