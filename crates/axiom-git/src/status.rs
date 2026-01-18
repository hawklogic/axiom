// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Git status operations.

use crate::{GitError, Repository};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Status of a file in the repository.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileStatus {
    /// File has been modified.
    Modified,
    /// File is staged for commit.
    Staged,
    /// File is untracked.
    Untracked,
    /// File has been deleted.
    Deleted,
    /// File has been renamed.
    Renamed,
    /// File is both staged and has unstaged modifications.
    StagedModified,
    /// File is ignored.
    Ignored,
    /// File has conflicts.
    Conflicted,
}

/// A file entry with its status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEntry {
    /// Path relative to repository root.
    pub path: PathBuf,
    /// Status of the file.
    pub status: FileStatus,
}

/// Repository status summary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepoStatus {
    /// Staged files.
    pub staged: Vec<StatusEntry>,
    /// Modified but unstaged files.
    pub modified: Vec<StatusEntry>,
    /// Untracked files.
    pub untracked: Vec<StatusEntry>,
    /// Deleted files.
    pub deleted: Vec<StatusEntry>,
    /// Conflicted files.
    pub conflicted: Vec<StatusEntry>,
}

impl RepoStatus {
    /// Check if there are any changes.
    pub fn has_changes(&self) -> bool {
        !self.staged.is_empty()
            || !self.modified.is_empty()
            || !self.untracked.is_empty()
            || !self.deleted.is_empty()
            || !self.conflicted.is_empty()
    }

    /// Get total count of changed files.
    pub fn total_changes(&self) -> usize {
        self.staged.len()
            + self.modified.len()
            + self.untracked.len()
            + self.deleted.len()
            + self.conflicted.len()
    }
}

/// Get repository status.
pub fn get_status(repo: &Repository) -> Result<RepoStatus, GitError> {
    let mut status = RepoStatus::default();

    let statuses = repo.inner().statuses(None)?;

    for entry in statuses.iter() {
        let path = PathBuf::from(entry.path().unwrap_or(""));
        let s = entry.status();

        // Index (staged) changes
        if s.is_index_new() || s.is_index_modified() || s.is_index_deleted() || s.is_index_renamed()
        {
            let file_status = if s.is_index_new() {
                FileStatus::Staged
            } else if s.is_index_deleted() {
                FileStatus::Deleted
            } else if s.is_index_renamed() {
                FileStatus::Renamed
            } else {
                FileStatus::Staged
            };

            status.staged.push(StatusEntry {
                path: path.clone(),
                status: file_status,
            });
        }

        // Worktree (unstaged) changes
        if s.is_wt_modified() {
            status.modified.push(StatusEntry {
                path: path.clone(),
                status: FileStatus::Modified,
            });
        }

        if s.is_wt_deleted() {
            status.deleted.push(StatusEntry {
                path: path.clone(),
                status: FileStatus::Deleted,
            });
        }

        if s.is_wt_new() {
            status.untracked.push(StatusEntry {
                path: path.clone(),
                status: FileStatus::Untracked,
            });
        }

        if s.is_conflicted() {
            status.conflicted.push(StatusEntry {
                path,
                status: FileStatus::Conflicted,
            });
        }
    }

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository as Git2Repo;
    use std::fs;
    use tempfile::TempDir;

    fn init_test_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().unwrap();
        let repo = Git2Repo::init(dir.path()).unwrap();

        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test").unwrap();
        config.set_str("user.email", "test@test.com").unwrap();

        let sig = repo.signature().unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
            .unwrap();

        let repo = Repository::open(dir.path()).unwrap();
        (dir, repo)
    }

    #[test]
    fn test_clean_status() {
        let (_dir, repo) = init_test_repo();
        let status = get_status(&repo).unwrap();
        assert!(!status.has_changes());
    }

    #[test]
    fn test_untracked_file() {
        let (dir, repo) = init_test_repo();

        fs::write(dir.path().join("new.txt"), "hello").unwrap();

        let status = get_status(&repo).unwrap();
        assert!(status.has_changes());
        assert_eq!(status.untracked.len(), 1);
        assert_eq!(status.untracked[0].path, PathBuf::from("new.txt"));
    }

    #[test]
    fn test_staged_file() {
        let (dir, repo) = init_test_repo();

        fs::write(dir.path().join("staged.txt"), "hello").unwrap();
        repo.stage(std::path::Path::new("staged.txt")).unwrap();

        let status = get_status(&repo).unwrap();
        assert!(status.has_changes());
        assert_eq!(status.staged.len(), 1);
    }
}
