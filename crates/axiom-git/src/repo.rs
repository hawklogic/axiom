// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Git repository operations.

use git2::Repository as Git2Repo;
use serde::{Deserialize, Serialize};
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

/// Commit information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub id: String,
    pub short_id: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub timestamp: i64,
}

/// Remote tracking status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteStatus {
    pub ahead: usize,
    pub behind: usize,
    pub has_remote: bool,
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

    /// Push to remote.
    pub fn push(&self, remote_name: &str, branch: &str) -> Result<(), GitError> {
        let mut remote = self.inner.find_remote(remote_name)?;
        let refspec = format!("refs/heads/{}", branch);
        remote.push(&[&refspec], None)?;
        Ok(())
    }

    /// Pull from remote (fast-forward only).
    pub fn pull(&self) -> Result<(), GitError> {
        // Get current branch
        let head = self.inner.head()?;
        let branch_name = head.shorthand().ok_or(GitError::NotARepository)?;
        
        // Fetch from origin
        let mut remote = self.inner.find_remote("origin")?;
        remote.fetch(&[branch_name], None, None)?;
        
        // Fast-forward merge
        let fetch_head = self.inner.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.inner.reference_to_annotated_commit(&fetch_head)?;
        
        let analysis = self.inner.merge_analysis(&[&fetch_commit])?;
        
        if analysis.0.is_up_to_date() {
            Ok(())
        } else if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch_name);
            let mut reference = self.inner.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            self.inner.set_head(&refname)?;
            self.inner.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            Ok(())
        } else {
            Err(GitError::Git2(git2::Error::from_str("Cannot fast-forward")))
        }
    }

    /// Get the most recent commit.
    pub fn last_commit(&self) -> Result<Option<CommitInfo>, GitError> {
        let head = match self.inner.head() {
            Ok(head) => head,
            Err(_) => return Ok(None),
        };

        let commit = head.peel_to_commit()?;
        let id = commit.id().to_string();
        let short_id = id[..7].to_string();
        let message = commit.message().unwrap_or("").to_string();
        let author = commit.author();
        
        Ok(Some(CommitInfo {
            id,
            short_id,
            message,
            author: author.name().unwrap_or("Unknown").to_string(),
            email: author.email().unwrap_or("").to_string(),
            timestamp: commit.time().seconds(),
        }))
    }

    /// Check if local branch is ahead/behind remote.
    pub fn remote_status(&self, branch: &str) -> Result<RemoteStatus, GitError> {
        let local_ref = format!("refs/heads/{}", branch);
        let remote_ref = format!("refs/remotes/origin/{}", branch);

        let local = match self.inner.find_reference(&local_ref) {
            Ok(r) => r,
            Err(_) => return Ok(RemoteStatus { ahead: 0, behind: 0, has_remote: false }),
        };

        let remote = match self.inner.find_reference(&remote_ref) {
            Ok(r) => r,
            Err(_) => return Ok(RemoteStatus { ahead: 0, behind: 0, has_remote: false }),
        };

        let local_oid = local.target().ok_or(GitError::NotARepository)?;
        let remote_oid = remote.target().ok_or(GitError::NotARepository)?;

        let (ahead, behind) = self.inner.graph_ahead_behind(local_oid, remote_oid)?;

        Ok(RemoteStatus {
            ahead,
            behind,
            has_remote: true,
        })
    }

    /// Get commit history.
    pub fn log(&self, limit: usize) -> Result<Vec<CommitInfo>, GitError> {
        let mut revwalk = self.inner.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut commits = Vec::new();
        for oid in revwalk.take(limit) {
            let oid = oid?;
            let commit = self.inner.find_commit(oid)?;
            
            let id = commit.id().to_string();
            let short_id = id[..7].to_string();
            let message = commit.message().unwrap_or("").to_string();
            let author = commit.author();
            
            commits.push(CommitInfo {
                id,
                short_id,
                message,
                author: author.name().unwrap_or("Unknown").to_string(),
                email: author.email().unwrap_or("").to_string(),
                timestamp: commit.time().seconds(),
            });
        }

        Ok(commits)
    }

    /// Get files changed in a commit.
    pub fn commit_files(&self, commit_id: &str) -> Result<Vec<String>, GitError> {
        let oid = git2::Oid::from_str(commit_id)?;
        let commit = self.inner.find_commit(oid)?;
        
        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = self.inner.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            None,
        )?;

        let mut files = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
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
