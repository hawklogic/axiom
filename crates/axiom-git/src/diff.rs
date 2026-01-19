// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Git diff operations.

use crate::{GitError, Repository};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A diff hunk (chunk of changes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    /// Old file start line.
    pub old_start: u32,
    /// Old file line count.
    pub old_lines: u32,
    /// New file start line.
    pub new_start: u32,
    /// New file line count.
    pub new_lines: u32,
    /// Lines in the hunk.
    pub lines: Vec<DiffLine>,
}

/// A single line in a diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    /// Origin character (+, -, ' ').
    pub origin: char,
    /// Line content.
    pub content: String,
    /// Old line number (if applicable).
    pub old_lineno: Option<u32>,
    /// New line number (if applicable).
    pub new_lineno: Option<u32>,
}

/// A file diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    /// Old path (None if new file).
    pub old_path: Option<PathBuf>,
    /// New path (None if deleted file).
    pub new_path: Option<PathBuf>,
    /// Hunks in the diff.
    pub hunks: Vec<DiffHunk>,
    /// Whether the file is binary.
    pub is_binary: bool,
}

/// Get diff between HEAD and working directory.
pub fn get_working_diff(repo: &Repository) -> Result<Vec<FileDiff>, GitError> {
    let head = repo.inner().head()?.peel_to_tree()?;
    let diff = repo.inner().diff_tree_to_workdir(Some(&head), None)?;

    parse_diff(&diff)
}

/// Get diff between HEAD and index (staged changes).
pub fn get_staged_diff(repo: &Repository) -> Result<Vec<FileDiff>, GitError> {
    let head = repo.inner().head()?.peel_to_tree()?;
    let diff = repo.inner().diff_tree_to_index(Some(&head), None, None)?;

    parse_diff(&diff)
}

/// Get diff for a specific file.
pub fn get_file_diff(repo: &Repository, path: &std::path::Path) -> Result<Option<FileDiff>, GitError> {
    // Use pathspec to filter diff to only the requested file - much faster
    let head = repo.inner().head()?.peel_to_tree()?;
    
    let mut opts = git2::DiffOptions::new();
    opts.pathspec(path);
    opts.include_untracked(false);
    
    let diff = repo.inner().diff_tree_to_workdir(Some(&head), Some(&mut opts))?;
    
    let mut diffs = parse_diff(&diff)?;
    
    // Should only have 0 or 1 results due to pathspec filter
    Ok(diffs.pop())
}

/// Parse a git2 diff into our types.
fn parse_diff(diff: &git2::Diff) -> Result<Vec<FileDiff>, GitError> {
    let mut file_diffs = Vec::new();

    diff.print(git2::DiffFormat::Patch, |delta, hunk, line| {
        // Get or create file diff entry
        let old_path = delta.old_file().path().map(|p| p.to_path_buf());
        let new_path = delta.new_file().path().map(|p| p.to_path_buf());
        let is_binary = delta.old_file().is_binary() || delta.new_file().is_binary();

        // Find or create file diff
        let file_diff = file_diffs
            .iter_mut()
            .find(|f: &&mut FileDiff| f.old_path == old_path && f.new_path == new_path);

        let file_diff = match file_diff {
            Some(f) => f,
            None => {
                file_diffs.push(FileDiff {
                    old_path: old_path.clone(),
                    new_path: new_path.clone(),
                    hunks: Vec::new(),
                    is_binary,
                });
                file_diffs.last_mut().unwrap()
            }
        };

        // Handle hunk header
        if let Some(hunk) = hunk {
            let last_hunk = file_diff.hunks.last();
            let is_new_hunk = last_hunk.map(|h| {
                h.old_start != hunk.old_start() || h.new_start != hunk.new_start()
            }).unwrap_or(true);

            if is_new_hunk {
                file_diff.hunks.push(DiffHunk {
                    old_start: hunk.old_start(),
                    old_lines: hunk.old_lines(),
                    new_start: hunk.new_start(),
                    new_lines: hunk.new_lines(),
                    lines: Vec::new(),
                });
            }
        }

        // Handle line
        if let Some(hunk) = file_diff.hunks.last_mut() {
            let origin = line.origin();
            if origin == '+' || origin == '-' || origin == ' ' {
                let content = String::from_utf8_lossy(line.content()).to_string();
                hunk.lines.push(DiffLine {
                    origin,
                    content,
                    old_lineno: line.old_lineno(),
                    new_lineno: line.new_lineno(),
                });
            }
        }

        true
    })?;

    Ok(file_diffs)
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

        // Create initial file and commit
        fs::write(dir.path().join("file.txt"), "line1\nline2\nline3\n").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(std::path::Path::new("file.txt")).unwrap();
        index.write().unwrap();

        let sig = repo.signature().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial", &tree, &[])
            .unwrap();

        let repo = Repository::open(dir.path()).unwrap();
        (dir, repo)
    }

    #[test]
    fn test_no_diff() {
        let (_dir, repo) = init_test_repo();
        let diff = get_working_diff(&repo).unwrap();
        assert!(diff.is_empty());
    }

    #[test]
    fn test_modified_file_diff() {
        let (dir, repo) = init_test_repo();

        // Modify the file
        fs::write(dir.path().join("file.txt"), "line1\nmodified\nline3\n").unwrap();

        let diff = get_working_diff(&repo).unwrap();
        assert_eq!(diff.len(), 1);
        assert_eq!(diff[0].new_path, Some(PathBuf::from("file.txt")));
        assert!(!diff[0].hunks.is_empty());
    }
}
