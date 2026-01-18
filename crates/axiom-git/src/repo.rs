// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Git repository operations.

use git2::{Repository as Git2Repo, Signature};
use std::path::{Path, PathBuf};

/// Git error type.
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Not a git repository")]
    NotARepository,

    #[error("Git error: {0}")]
    Git2(#[from] git2::Error),

    #[error("No commits in repository")]
    NoCommits,
}

/// Git repository wrapper.
pub struct Repository {
    inner: Git2Repo,
    path: PathBuf,
}

impl Repository {
    /// Open a repository at the given path.
    pub fn open(path: &Path) -> Result<Self, GitError> {
        let inner = Git2Repo::open(path)?;
        let path = inner.workdir().unwrap_or(path).to_path_buf();
        Ok(Self { inner, path })
    }

    /// Discover a repository from a path.
    pub fn discover(path: &Path) -> Result<Self, GitError> {
        let inner = Git2Repo::discover(path)?;
        let path = inner.workdir().unwrap_or(path).to_path_buf();
        Ok(Self { inner, path })
    }

    /// Get the repository root path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the current branch name.
    pub fn current_branch(&self) -> Result<Option<String>, GitError> {
        let head = match self.inner.head() {
            Ok(head) => head,
            Err(_) => return Ok(None),
        };

        if head.is_branch() {
            Ok(head.shorthand().map(|s| s.to_string()))
        } else {
            // Detached HEAD
            Ok(head.target().map(|oid| oid.to_string()[..7].to_string()))
        }
    }

    /// List all branches.
    pub fn branches(&self) -> Result<Vec<String>, GitError> {
        let branches = self.inner.branches(Some(git2::BranchType::Local))?;
        let mut names = Vec::new();

        for branch in branches {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                names.push(name.to_string());
            }
        }

        Ok(names)
    }

    /// Stage a file.
    pub fn stage(&self, path: &Path) -> Result<(), GitError> {
        let mut index = self.inner.index()?;
        index.add_path(path)?;
        index.write()?;
        Ok(())
    }

    /// Unstage a file.
    pub fn unstage(&self, path: &Path) -> Result<(), GitError> {
        let head = self.inner.head()?.peel_to_commit()?;
        self.inner.reset_default(Some(&head.into_object()), [path])?;
        Ok(())
    }

    /// Commit staged changes.
    pub fn commit(&self, message: &str) -> Result<String, GitError> {
        let mut index = self.inner.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.inner.find_tree(tree_id)?;

        let sig = self.inner.signature()?;
        let head = self.inner.head()?.peel_to_commit()?;

        let commit_id = self.inner.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&head],
        )?;

        Ok(commit_id.to_string())
    }

    /// Get the inner git2 repository (for advanced operations).
    pub fn inner(&self) -> &Git2Repo {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn init_test_repo() -> (TempDir, Repository) {
        let dir = TempDir::new().unwrap();
        let repo = Git2Repo::init(dir.path()).unwrap();

        // Configure user for commits
        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@test.com").unwrap();

        // Create initial commit
        let sig = repo.signature().unwrap();
        let tree_id = repo.index().unwrap().write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();

        let repo = Repository::open(dir.path()).unwrap();
        (dir, repo)
    }

    #[test]
    fn test_open_repo() {
        let (dir, repo) = init_test_repo();
        assert!(repo.path().exists());
    }

    #[test]
    fn test_current_branch() {
        let (_dir, repo) = init_test_repo();
        let branch = repo.current_branch().unwrap();
        // Default branch is usually "master" or "main"
        assert!(branch.is_some());
    }

    #[test]
    fn test_stage_and_commit() {
        let (dir, repo) = init_test_repo();

        // Create a file
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "hello").unwrap();

        // Stage it
        repo.stage(Path::new("test.txt")).unwrap();

        // Commit it
        let commit_id = repo.commit("Add test file").unwrap();
        assert!(!commit_id.is_empty());
    }
}
