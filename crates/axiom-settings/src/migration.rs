// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Settings migration system.

use crate::{Settings, SCHEMA_VERSION};

/// Error type for migration failures.
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Unknown schema version: {0}")]
    UnknownVersion(u32),

    #[error("Migration failed from version {from} to {to}: {reason}")]
    MigrationFailed { from: u32, to: u32, reason: String },
}

/// Result of a migration operation.
pub struct MigrationResult {
    /// The migrated settings.
    pub settings: Settings,
    /// Whether any migrations were applied.
    pub migrated: bool,
    /// The original version before migration.
    pub original_version: u32,
}

/// Migrate settings to the current schema version.
///
/// If settings are already at the current version, returns them unchanged.
/// If settings are from a newer version, returns an error.
pub fn migrate(mut settings: Settings) -> Result<MigrationResult, MigrationError> {
    let original_version = settings.version;

    if settings.version > SCHEMA_VERSION {
        return Err(MigrationError::UnknownVersion(settings.version));
    }

    if settings.version == SCHEMA_VERSION {
        return Ok(MigrationResult {
            settings,
            migrated: false,
            original_version,
        });
    }

    // Apply migrations sequentially
    while settings.version < SCHEMA_VERSION {
        settings = apply_migration(settings)?;
    }

    Ok(MigrationResult {
        settings,
        migrated: true,
        original_version,
    })
}

/// Apply a single migration step.
fn apply_migration(settings: Settings) -> Result<Settings, MigrationError> {
    // Add migration functions here as schema evolves:
    // 0 => migrate_v0_to_v1(settings),
    // 1 => migrate_v1_to_v2(settings),
    let v = settings.version;
    Err(MigrationError::MigrationFailed {
        from: v,
        to: v + 1,
        reason: format!("No migration defined for version {}", v),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version_no_migration() {
        let settings = Settings::default();
        let result = migrate(settings).unwrap();
        assert!(!result.migrated);
        assert_eq!(result.original_version, SCHEMA_VERSION);
    }

    #[test]
    fn test_future_version_error() {
        let mut settings = Settings::default();
        settings.version = SCHEMA_VERSION + 1;
        let result = migrate(settings);
        assert!(result.is_err());
    }
}
