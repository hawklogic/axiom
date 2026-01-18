// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

//! Git command handlers.

use axiom_git::{CommitInfo, FileDiff, RemoteStatus, RepoStatus, Repository};
use std::path::Path;

/// Get git status for a repository.
#[tauri::command]
pub fn git_status(path: String) -> Result<RepoStatus, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    axiom_git::get_status(&repo).map_err(|e| e.to_string())
}

/// Get git diff for working directory.
#[tauri::command]
pub fn git_diff(path: String) -> Result<Vec<FileDiff>, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    axiom_git::get_working_diff(&repo).map_err(|e| e.to_string())
}

/// Get git diff for a specific file.
#[tauri::command]
pub fn git_file_diff(repo_path: String, file_path: String) -> Result<Option<FileDiff>, String> {
    let repo = Repository::discover(Path::new(&repo_path)).map_err(|e| e.to_string())?;
    axiom_git::get_file_diff(&repo, Path::new(&file_path)).map_err(|e| e.to_string())
}

/// Stage a file.
#[tauri::command]
pub fn git_stage(repo_path: String, file_path: String) -> Result<(), String> {
    let repo = Repository::discover(Path::new(&repo_path)).map_err(|e| e.to_string())?;
    repo.stage(Path::new(&file_path)).map_err(|e| e.to_string())
}

/// Unstage a file.
#[tauri::command]
pub fn git_unstage(repo_path: String, file_path: String) -> Result<(), String> {
    let repo = Repository::discover(Path::new(&repo_path)).map_err(|e| e.to_string())?;
    repo.unstage(Path::new(&file_path)).map_err(|e| e.to_string())
}

/// Commit staged changes.
#[tauri::command]
pub fn git_commit(path: String, message: String) -> Result<String, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.commit(&message).map_err(|e| e.to_string())
}

/// Get current branch name.
#[tauri::command]
pub fn git_branch(path: String) -> Result<Option<String>, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.current_branch().map_err(|e| e.to_string())
}

/// Push to remote.
#[tauri::command]
pub fn git_push(path: String, remote: String, branch: String) -> Result<(), String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.push(&remote, &branch).map_err(|e| e.to_string())
}

/// Pull from remote.
#[tauri::command]
pub fn git_pull(path: String) -> Result<(), String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.pull().map_err(|e| e.to_string())
}

/// Get the most recent commit info.
#[tauri::command]
pub fn git_last_commit(path: String) -> Result<Option<CommitInfo>, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.last_commit().map_err(|e| e.to_string())
}

/// Check if local branch is ahead/behind remote.
#[tauri::command]
pub fn git_remote_status(path: String, branch: String) -> Result<RemoteStatus, String> {
    let repo = Repository::discover(Path::new(&path)).map_err(|e| e.to_string())?;
    repo.remote_status(&branch).map_err(|e| e.to_string())
}
