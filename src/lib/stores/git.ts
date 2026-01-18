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

function createGitStore() {
  const status = writable<RepoStatus | null>(null);
  const branch = writable<string | null>(null);
  const loading = writable(false);

  const store = {
    subscribe(run: (value: { status: RepoStatus | null; branch: string | null; loading: boolean }) => void) {
      return derived([status, branch, loading], ([$status, $branch, $loading]) => ({
        status: $status,
        branch: $branch,
        loading: $loading,
      })).subscribe(run);
    },

    async refresh(path: string) {
      loading.set(true);
      try {
        const [statusResult, branchResult] = await Promise.all([
          invoke<RepoStatus>('git_status', { path }),
          invoke<string | null>('git_branch', { path }),
        ]);
        status.set(statusResult);
        branch.set(branchResult);
      } catch (e) {
        console.error('Git refresh failed:', e);
        status.set(null);
        branch.set(null);
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
  };

  return store;
}

export const gitStore = createGitStore();
