// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Git store - manages git state.
 */

import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface StatusEntry {
  path: string;
  status: 'Modified' | 'Staged' | 'Untracked' | 'Deleted' | 'Renamed' | 'Conflicted';
}

export interface RepoStatus {
  staged: StatusEntry[];
  modified: StatusEntry[];
  untracked: StatusEntry[];
  deleted: StatusEntry[];
  conflicted: StatusEntry[];
}

export interface CommitInfo {
  id: string;
  short_id: string;
  message: string;
  author: string;
  email: string;
  timestamp: number;
}

export interface RemoteStatus {
  ahead: number;
  behind: number;
  has_remote: boolean;
}

function createGitStore() {
  const status = writable<RepoStatus | null>(null);
  const branch = writable<string | null>(null);
  const loading = writable(false);
  const lastCommit = writable<CommitInfo | null>(null);
  const remoteStatus = writable<RemoteStatus | null>(null);

  const store = {
    subscribe(run: (value: { 
      status: RepoStatus | null; 
      branch: string | null; 
      loading: boolean;
      lastCommit: CommitInfo | null;
      remoteStatus: RemoteStatus | null;
    }) => void) {
      return derived(
        [status, branch, loading, lastCommit, remoteStatus], 
        ([$status, $branch, $loading, $lastCommit, $remoteStatus]) => ({
          status: $status,
          branch: $branch,
          loading: $loading,
          lastCommit: $lastCommit,
          remoteStatus: $remoteStatus,
        })
      ).subscribe(run);
    },

    async refresh(path: string) {
      loading.set(true);
      try {
        const [statusResult, branchResult, commitResult] = await Promise.all([
          invoke<RepoStatus>('git_status', { path }),
          invoke<string | null>('git_branch', { path }),
          invoke<CommitInfo | null>('git_last_commit', { path }),
        ]);
        
        // Only update if data has actually changed to avoid flicker
        status.update(current => {
          if (JSON.stringify(current) !== JSON.stringify(statusResult)) {
            return statusResult;
          }
          return current;
        });
        
        branch.update(current => {
          if (current !== branchResult) {
            return branchResult;
          }
          return current;
        });
        
        lastCommit.update(current => {
          if (JSON.stringify(current) !== JSON.stringify(commitResult)) {
            return commitResult;
          }
          return current;
        });

        // Fetch remote status if we have a branch
        if (branchResult) {
          try {
            const remoteResult = await invoke<RemoteStatus>('git_remote_status', { 
              path, 
              branch: branchResult 
            });
            remoteStatus.update(current => {
              if (JSON.stringify(current) !== JSON.stringify(remoteResult)) {
                return remoteResult;
              }
              return current;
            });
          } catch (e) {
            remoteStatus.set(null);
          }
        } else {
          remoteStatus.set(null);
        }
      } catch (e) {
        console.error('Git refresh failed:', e);
        status.set(null);
        branch.set(null);
        lastCommit.set(null);
        remoteStatus.set(null);
      } finally {
        loading.set(false);
      }
    },

    async stage(repoPath: string, filePath: string) {
      await invoke('git_stage', { repoPath, filePath });
      await store.refresh(repoPath);
    },

    async unstage(repoPath: string, filePath: string) {
      await invoke('git_unstage', { repoPath, filePath });
      await store.refresh(repoPath);
    },

    async commit(path: string, message: string) {
      const commitId = await invoke<string>('git_commit', { path, message });
      await store.refresh(path);
      return commitId;
    },

    async push(path: string, remote: string, branch: string) {
      await invoke('git_push', { path, remote, branch });
      await store.refresh(path);
    },

    async pull(path: string) {
      await invoke('git_pull', { path });
      await store.refresh(path);
    },
  };

  return store;
}

export const gitStore = createGitStore();
