// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Terminal session management.

use crate::{Pty, TerminalError, TerminalSize};
use std::collections::HashMap;

/// Unique identifier for a terminal session.
pub type SessionId = u32;

/// A terminal session.
pub struct Session {
    /// Session ID.
    pub id: SessionId,
    /// The underlying PTY.
    pub pty: Pty,
    /// Terminal title (if set).
    pub title: Option<String>,
}

impl Session {
    /// Create a new session.
    pub fn new(id: SessionId, size: TerminalSize) -> Result<Self, TerminalError> {
        let pty = Pty::new(size)?;
        Ok(Self {
            id,
            pty,
            title: None,
        })
    }

    /// Start the shell.
    pub fn start(&self) -> Result<(), TerminalError> {
        self.pty.spawn_shell()
    }

    /// Write to the session.
    pub fn write(&self, data: &[u8]) -> Result<usize, TerminalError> {
        self.pty.write(data)
    }

    /// Read from the session.
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, TerminalError> {
        self.pty.read(buf)
    }

    /// Resize the session.
    pub fn resize(&self, size: TerminalSize) -> Result<(), TerminalError> {
        self.pty.resize(size)
    }
}

/// Manager for multiple terminal sessions.
pub struct SessionManager {
    sessions: HashMap<SessionId, Session>,
    next_id: SessionId,
    default_size: TerminalSize,
}

impl SessionManager {
    /// Create a new session manager.
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_id: 1,
            default_size: TerminalSize::default(),
        }
    }

    /// Set default terminal size for new sessions.
    pub fn set_default_size(&mut self, size: TerminalSize) {
        self.default_size = size;
    }

    /// Create a new terminal session.
    pub fn create_session(&mut self) -> Result<SessionId, TerminalError> {
        let id = self.next_id;
        self.next_id += 1;

        let session = Session::new(id, self.default_size)?;
        session.start()?;
        self.sessions.insert(id, session);

        Ok(id)
    }

    /// Get a session by ID.
    pub fn get(&self, id: SessionId) -> Option<&Session> {
        self.sessions.get(&id)
    }

    /// Get a mutable session by ID.
    pub fn get_mut(&mut self, id: SessionId) -> Option<&mut Session> {
        self.sessions.get_mut(&id)
    }

    /// Remove a session.
    pub fn remove(&mut self, id: SessionId) -> Option<Session> {
        self.sessions.remove(&id)
    }

    /// List all session IDs.
    pub fn list(&self) -> Vec<SessionId> {
        self.sessions.keys().copied().collect()
    }

    /// Get the number of active sessions.
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_manager() {
        let mut manager = SessionManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_create_session() {
        let mut manager = SessionManager::new();
        let id = manager.create_session();
        // Note: This may fail in CI environments without a TTY
        if id.is_ok() {
            assert_eq!(manager.count(), 1);
            assert!(manager.get(id.unwrap()).is_some());
        }
    }
}
