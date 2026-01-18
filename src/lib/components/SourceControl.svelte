<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { gitStore } from '$lib/stores/git';
  import { workspace } from '$lib/stores/workspace';
  import { consoleStore } from '$lib/stores/console';
  import type { StatusEntry } from '$lib/stores/git';

  let commitMessage = '';
  let refreshInterval: number;
  let selectedFiles = new Set<string>();
  let showCommitInput = false;

  $: ({ status, branch, loading } = $gitStore);
  $: workspacePath = $workspace.path;

  // Focus action for textarea
  function focusOnMount(node: HTMLTextAreaElement) {
    node.focus();
    return {};
  }

  // Combine file statuses to show both staged and unstaged changes
  interface CombinedFileStatus {
    path: string;
    stagedStatus?: StatusEntry['status'];
    unstagedStatus?: StatusEntry['status'];
  }

  $: combinedFiles = (() => {
    if (!status) return { staged: [], unstaged: [] };

    const fileMap = new Map<string, CombinedFileStatus>();

    // Process staged files
    status.staged.forEach(file => {
      const existing = fileMap.get(file.path) || { path: file.path };
      existing.stagedStatus = file.status;
      fileMap.set(file.path, existing);
    });

    // Process unstaged files (modified, untracked, deleted)
    [...status.modified, ...status.untracked, ...status.deleted].forEach(file => {
      const existing = fileMap.get(file.path) || { path: file.path };
      existing.unstagedStatus = file.status;
      fileMap.set(file.path, existing);
    });

    // Separate into staged-only and unstaged lists
    const staged: CombinedFileStatus[] = [];
    const unstaged: CombinedFileStatus[] = [];

    fileMap.forEach(file => {
      if (file.stagedStatus && file.unstagedStatus) {
        // File has both staged and unstaged changes - show in both sections
        staged.push({ path: file.path, stagedStatus: file.stagedStatus });
        unstaged.push({ path: file.path, unstagedStatus: file.unstagedStatus });
      } else if (file.stagedStatus) {
        staged.push(file);
      } else if (file.unstagedStatus) {
        unstaged.push(file);
      }
    });

    return { staged, unstaged };
  })();

  onMount(() => {
    // Initial refresh
    if (workspacePath) {
      gitStore.refresh(workspacePath);
    }

    // Auto-refresh every 5 seconds
    refreshInterval = window.setInterval(() => {
      if (workspacePath) {
        gitStore.refresh(workspacePath);
      }
    }, 5000);
  });

  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });

  async function handleRefresh() {
    if (!workspacePath) return;
    consoleStore.log('info', 'git', 'Refreshing git status...');
    await gitStore.refresh(workspacePath);
    consoleStore.log('info', 'git', 'Git status refreshed');
  }

  async function handleStage(file: StatusEntry) {
    if (!workspacePath) return;
    try {
      consoleStore.log('info', 'git', `Staging ${file.path}...`);
      await gitStore.stage(workspacePath, file.path);
      consoleStore.log('info', 'git', `Staged ${file.path}`);
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to stage ${file.path}: ${err}`);
    }
  }

  async function handleStagePath(path: string) {
    if (!workspacePath) return;
    try {
      consoleStore.log('info', 'git', `Staging ${path}...`);
      await gitStore.stage(workspacePath, path);
      consoleStore.log('info', 'git', `Staged ${path}`);
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to stage ${path}: ${err}`);
    }
  }

  async function handleUnstage(file: StatusEntry) {
    if (!workspacePath) return;
    try {
      consoleStore.log('info', 'git', `Unstaging ${file.path}...`);
      await gitStore.unstage(workspacePath, file.path);
      consoleStore.log('info', 'git', `Unstaged ${file.path}`);
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to unstage ${file.path}: ${err}`);
    }
  }

  async function handleUnstagePath(path: string) {
    if (!workspacePath) return;
    try {
      consoleStore.log('info', 'git', `Unstaging ${path}...`);
      await gitStore.unstage(workspacePath, path);
      consoleStore.log('info', 'git', `Unstaged ${path}`);
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to unstage ${path}: ${err}`);
    }
  }

  async function handleStageAll() {
    if (!workspacePath || !status) return;
    const files = [...status.modified, ...status.untracked, ...status.deleted];
    
    try {
      consoleStore.log('info', 'git', `Staging ${files.length} files...`);
      for (const file of files) {
        await gitStore.stage(workspacePath, file.path);
      }
      consoleStore.log('info', 'git', 'All changes staged');
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to stage all: ${err}`);
    }
  }

  async function handleUnstageAll() {
    if (!workspacePath || !status) return;
    
    try {
      consoleStore.log('info', 'git', `Unstaging ${status.staged.length} files...`);
      for (const file of status.staged) {
        await gitStore.unstage(workspacePath, file.path);
      }
      consoleStore.log('info', 'git', 'All changes unstaged');
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to unstage all: ${err}`);
    }
  }

  async function handleCommit() {
    if (!workspacePath || !commitMessage.trim()) return;
    
    try {
      consoleStore.log('info', 'git', `Committing with message: "${commitMessage}"`);
      const commitId = await gitStore.commit(workspacePath, commitMessage);
      consoleStore.log('info', 'git', `Committed: ${commitId.substring(0, 7)}`);
      commitMessage = '';
      showCommitInput = false;
    } catch (err) {
      consoleStore.log('error', 'git', `Failed to commit: ${err}`);
    }
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'Modified': return 'M';
      case 'Staged': return 'A';
      case 'Untracked': return 'U';
      case 'Deleted': return 'D';
      case 'Renamed': return 'R';
      case 'Conflicted': return '!';
      default: return '?';
    }
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case 'Modified': return '#e2c08d';
      case 'Staged': return '#73c991';
      case 'Untracked': return '#73c991';
      case 'Deleted': return '#f48771';
      case 'Renamed': return '#6cb6ff';
      case 'Conflicted': return '#f48771';
      default: return '#abb2bf';
    }
  }

  function getStatusLabel(status: string): string {
    switch (status) {
      case 'Modified': return 'Modified';
      case 'Staged': return 'Added';
      case 'Untracked': return 'Untracked';
      case 'Deleted': return 'Deleted';
      case 'Renamed': return 'Renamed';
      case 'Conflicted': return 'Conflicted';
      default: return 'Unknown';
    }
  }
</script>

<div class="source-control">
  {#if !workspacePath}
    <div class="empty-state">
      <p>No workspace open</p>
      <p class="hint">Open a folder to use source control</p>
    </div>
  {:else if loading}
    <div class="loading-state">
      <p>Loading git status...</p>
    </div>
  {:else if !status}
    <div class="empty-state">
      <p>Not a git repository</p>
      <p class="hint">Initialize git or open a git repository</p>
    </div>
  {:else}
    <div class="header">
      <div class="branch-info">
        <span class="branch-icon">⎇</span>
        <span class="branch-name">{branch || 'detached'}</span>
      </div>
      <button class="icon-button" on:click={handleRefresh} title="Refresh">
        <span>↻</span>
      </button>
    </div>

    <div class="changes-container">
      <!-- Staged Changes -->
      {#if combinedFiles.staged.length > 0}
        <div class="section">
          <div class="section-header">
            <span class="section-title">Staged Changes ({combinedFiles.staged.length})</span>
            <button class="text-button" on:click={handleUnstageAll}>Unstage All</button>
          </div>
          <div class="file-list">
            {#each combinedFiles.staged as file}
              <div 
                class="file-item" 
                role="button" 
                tabindex="0"
                on:click={() => handleUnstagePath(file.path)}
                on:keydown={(e) => e.key === 'Enter' && handleUnstagePath(file.path)}
              >
                <span class="status-badge staged" style="color: {getStatusColor(file.stagedStatus || 'Staged')}">
                  {getStatusIcon(file.stagedStatus || 'Staged')}
                </span>
                <span class="file-path">{file.path}</span>
                {#if file.unstagedStatus}
                  <span class="status-indicator unstaged" title="Also has unstaged changes">
                    {getStatusIcon(file.unstagedStatus)}
                  </span>
                {/if}
                <button class="action-button" title="Unstage">−</button>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Changes (Unstaged) -->
      {#if combinedFiles.unstaged.length > 0}
        <div class="section">
          <div class="section-header">
            <span class="section-title">
              Changes ({combinedFiles.unstaged.length})
            </span>
            <button class="text-button" on:click={handleStageAll}>Stage All</button>
          </div>
          <div class="file-list">
            {#each combinedFiles.unstaged as file}
              <div 
                class="file-item" 
                role="button" 
                tabindex="0"
                on:click={() => handleStagePath(file.path)}
                on:keydown={(e) => e.key === 'Enter' && handleStagePath(file.path)}
              >
                <span class="status-badge unstaged" style="color: {getStatusColor(file.unstagedStatus || 'Modified')}">
                  {getStatusIcon(file.unstagedStatus || 'Modified')}
                </span>
                <span class="file-path">{file.path}</span>
                {#if file.stagedStatus}
                  <span class="status-indicator staged" title="Also has staged changes">
                    ✓
                  </span>
                {/if}
                <button class="action-button" title="Stage">+</button>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Conflicted Files -->
      {#if status.conflicted.length > 0}
        <div class="section">
          <div class="section-header">
            <span class="section-title conflict">Conflicts ({status.conflicted.length})</span>
          </div>
          <div class="file-list">
            {#each status.conflicted as file}
              <div class="file-item conflict">
                <span class="status-badge" style="color: {getStatusColor(file.status)}">
                  {getStatusIcon(file.status)}
                </span>
                <span class="file-path">{file.path}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- No Changes -->
      {#if combinedFiles.staged.length === 0 && combinedFiles.unstaged.length === 0 && status.conflicted.length === 0}
        <div class="empty-state">
          <p>No changes</p>
          <p class="hint">Working tree clean</p>
        </div>
      {/if}
    </div>

    <!-- Commit Section -->
    {#if combinedFiles.staged.length > 0}
      <div class="commit-section">
        {#if showCommitInput}
          <textarea
            class="commit-input"
            bind:value={commitMessage}
            placeholder="Commit message (Ctrl+Enter to commit)..."
            rows="3"
            use:focusOnMount
            on:keydown={(e) => {
              if ((e.ctrlKey || e.metaKey) && e.key === 'Enter' && commitMessage.trim()) {
                handleCommit();
              }
            }}
          />
          <div class="commit-actions">
            <button class="commit-button" on:click={handleCommit} disabled={!commitMessage.trim()}>
              Commit ({combinedFiles.staged.length})
            </button>
            <button class="cancel-button" on:click={() => { showCommitInput = false; commitMessage = ''; }}>
              Cancel
            </button>
          </div>
        {:else}
          <button class="show-commit-button" on:click={() => showCommitInput = true}>
            <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M2 8h12M8 2v12"/>
            </svg>
            Commit {combinedFiles.staged.length} {combinedFiles.staged.length === 1 ? 'file' : 'files'}
          </button>
        {/if}
      </div>
    {/if}
  {/if}
</div>

<style>
  .source-control {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-secondary);
  }

  .branch-info {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-sm);
  }

  .branch-icon {
    color: var(--color-accent);
    font-size: 16px;
  }

  .branch-name {
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .icon-button {
    padding: 4px 8px;
    font-size: 16px;
    color: var(--color-text-muted);
    transition: color 0.15s;
  }

  .icon-button:hover {
    color: var(--color-text-primary);
  }

  .changes-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .section {
    border-bottom: 1px solid var(--color-border);
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-xs) var(--space-sm);
    background: var(--color-bg-secondary);
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .section-title {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-muted);
  }

  .section-title.conflict {
    color: #f48771;
  }

  .text-button {
    font-size: var(--font-size-xs);
    color: var(--color-accent);
    padding: 2px 6px;
    transition: opacity 0.15s;
  }

  .text-button:hover {
    opacity: 0.8;
  }

  .file-list {
    display: flex;
    flex-direction: column;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: 4px var(--space-sm);
    cursor: pointer;
    transition: background 0.15s;
    position: relative;
  }

  .file-item:hover {
    background: var(--color-bg-hover);
  }

  .file-item.conflict {
    background: rgba(244, 135, 113, 0.1);
  }

  .status-badge {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 11px;
    font-weight: 600;
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .status-badge.staged {
    opacity: 1;
  }

  .status-badge.unstaged {
    opacity: 0.9;
  }

  .status-indicator {
    font-family: 'SF Mono', 'Monaco', 'Courier New', monospace;
    font-size: 10px;
    font-weight: 600;
    padding: 2px 4px;
    border-radius: 3px;
    margin-left: auto;
    margin-right: 4px;
    flex-shrink: 0;
  }

  .status-indicator.staged {
    background: rgba(115, 201, 145, 0.2);
    color: #73c991;
  }

  .status-indicator.unstaged {
    background: rgba(226, 192, 141, 0.2);
    color: #e2c08d;
  }

  .file-path {
    flex: 1;
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .action-button {
    opacity: 0;
    font-size: 14px;
    padding: 2px 6px;
    color: var(--color-text-muted);
    transition: opacity 0.15s, color 0.15s;
  }

  .file-item:hover .action-button {
    opacity: 1;
  }

  .action-button:hover {
    color: var(--color-accent);
  }

  .commit-section {
    border-top: 1px solid var(--color-border);
    padding: var(--space-sm);
    background: var(--color-bg-secondary);
  }

  .commit-input {
    width: 100%;
    padding: var(--space-xs);
    font-size: var(--font-size-sm);
    font-family: inherit;
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    resize: vertical;
    min-height: 60px;
    line-height: 1.4;
  }

  .commit-input::placeholder {
    color: var(--color-text-muted);
    opacity: 0.6;
  }

  .commit-input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px rgba(0, 212, 255, 0.1);
  }

  .commit-actions {
    display: flex;
    gap: var(--space-xs);
    margin-top: var(--space-xs);
  }

  .commit-button {
    flex: 1;
    padding: var(--space-xs) var(--space-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: var(--radius-sm);
    transition: opacity 0.15s;
  }

  .commit-button:hover:not(:disabled) {
    opacity: 0.9;
  }

  .commit-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .cancel-button {
    padding: var(--space-xs) var(--space-sm);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    transition: background 0.15s;
  }

  .cancel-button:hover {
    background: var(--color-bg-hover);
  }

  .show-commit-button {
    width: 100%;
    padding: var(--space-sm);
    font-size: var(--font-size-sm);
    font-weight: 600;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: var(--radius-sm);
    transition: opacity 0.15s;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-xs);
  }

  .show-commit-button svg {
    width: 14px;
    height: 14px;
  }

  .show-commit-button:hover {
    opacity: 0.9;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-xl);
    text-align: center;
    color: var(--color-text-muted);
  }

  .empty-state p {
    margin: 0;
    font-size: var(--font-size-sm);
  }

  .empty-state .hint {
    margin-top: var(--space-xs);
    font-size: var(--font-size-xs);
    opacity: 0.7;
  }

  .loading-state {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-xl);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
