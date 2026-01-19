<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount } from 'svelte';

  export let filePath: string;
  export let repoPath: string;

  interface DiffLine {
    origin: string;
    content: string;
    old_lineno: number | null;
    new_lineno: number | null;
  }

  interface DiffHunk {
    old_start: number;
    old_lines: number;
    new_start: number;
    new_lines: number;
    lines: DiffLine[];
  }

  interface FileDiff {
    old_path: string | null;
    new_path: string | null;
    hunks: DiffHunk[];
    is_binary: boolean;
  }

  let diff: FileDiff | null = null;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    const startTime = performance.now();
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      
      // Ensure we're using a relative path from the repo root
      let relativePath = filePath;
      if (relativePath.startsWith(repoPath)) {
        relativePath = relativePath.substring(repoPath.length);
        // Remove leading slash
        if (relativePath.startsWith('/')) {
          relativePath = relativePath.substring(1);
        }
      }
      
      console.log('[DiffViewer] Loading diff for:', relativePath, 'in repo:', repoPath);
      
      const result = await invoke<FileDiff | null>('git_file_diff', {
        repoPath,
        filePath: relativePath,
      });
      
      const loadTime = performance.now() - startTime;
      console.log('[DiffViewer] Diff loaded in', loadTime.toFixed(2), 'ms');
      
      // If no diff found, it might be an untracked file - show the whole file as added
      if (!result) {
        try {
          const fullPath = `${repoPath}/${relativePath}`;
          const content = await invoke<string>('read_file', { path: fullPath });
          
          // Create a synthetic diff showing the entire file as added
          const lines = content.split('\n');
          diff = {
            old_path: null,
            new_path: relativePath,
            is_binary: false,
            hunks: [{
              old_start: 0,
              old_lines: 0,
              new_start: 1,
              new_lines: lines.length,
              lines: lines.map((line, idx) => ({
                origin: '+',
                content: line + '\n',
                old_lineno: null,
                new_lineno: idx + 1,
              })),
            }],
          };
          console.log('[DiffViewer] Created synthetic diff for untracked file');
        } catch (readErr) {
          console.error('[DiffViewer] Failed to read untracked file:', readErr);
          diff = null;
        }
      } else {
        diff = result;
      }
      
      loading = false;
      const totalTime = performance.now() - startTime;
      console.log('[DiffViewer] Total render time:', totalTime.toFixed(2), 'ms');
    } catch (err) {
      console.error('[DiffViewer] Error loading diff:', err);
      error = String(err);
      loading = false;
    }
  });

  function getLineClass(origin: string): string {
    switch (origin) {
      case '+': return 'line-added';
      case '-': return 'line-removed';
      default: return 'line-context';
    }
  }

  function getLinePrefix(origin: string): string {
    switch (origin) {
      case '+': return '+';
      case '-': return '-';
      default: return ' ';
    }
  }
</script>

<div class="diff-viewer">
  {#if loading}
    <div class="loading-state">
      <p>Loading diff...</p>
    </div>
  {:else if error}
    <div class="error-state">
      <p>Failed to load diff</p>
      <p class="error-message">{error}</p>
      <p class="hint">File: {filePath}</p>
      <p class="hint">Repo: {repoPath}</p>
    </div>
  {:else if !diff}
    <div class="empty-state">
      <p>No diff available</p>
      <p class="hint">File may be untracked, unchanged, or not in the working directory</p>
      <p class="hint-detail">Path: {filePath}</p>
    </div>
  {:else if diff.is_binary}
    <div class="binary-state">
      <p>Binary file</p>
      <p class="hint">Cannot display diff for binary files</p>
    </div>
  {:else}
    <div class="diff-header">
      <div class="file-paths">
        {#if diff.old_path && diff.new_path && diff.old_path !== diff.new_path}
          <span class="path old">{diff.old_path}</span>
          <span class="arrow">→</span>
          <span class="path new">{diff.new_path}</span>
        {:else}
          <span class="path">{diff.new_path || diff.old_path || filePath}</span>
        {/if}
      </div>
    </div>

    <div class="diff-content">
      {#each diff.hunks as hunk, hunkIdx (hunkIdx)}
        <div class="hunk">
          <div class="hunk-header">
            @@ -{hunk.old_start},{hunk.old_lines} +{hunk.new_start},{hunk.new_lines} @@
          </div>
          <div class="hunk-lines">
            {#each hunk.lines as line, lineIdx (`${hunkIdx}-${lineIdx}`)}
              <div class="diff-line {getLineClass(line.origin)}">
                <span class="line-number old">{line.old_lineno || ''}</span>
                <span class="line-number new">{line.new_lineno || ''}</span>
                <span class="line-prefix">{getLinePrefix(line.origin)}</span>
                <span class="line-content">{line.content}</span>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .diff-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--color-bg-primary);
  }

  .loading-state,
  .error-state,
  .empty-state,
  .binary-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted);
    padding: var(--space-xl);
    text-align: center;
  }

  .loading-state p,
  .empty-state p,
  .binary-state p {
    margin: 0;
    font-size: var(--font-size-sm);
  }

  .error-state p {
    margin: 0;
    font-size: var(--font-size-sm);
  }

  .error-message {
    margin-top: var(--space-xs);
    font-size: var(--font-size-xs);
    color: #f48771;
  }

  .hint {
    margin-top: var(--space-xs);
    font-size: var(--font-size-xs);
    opacity: 0.7;
  }

  .hint-detail {
    margin-top: var(--space-xs);
    font-size: var(--font-size-xs);
    opacity: 0.5;
    font-family: var(--font-mono);
  }

  .diff-header {
    padding: var(--space-sm);
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }

  .file-paths {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-sm);
    font-family: var(--font-mono);
  }

  .path {
    color: var(--color-text-primary);
  }

  .path.old {
    color: #f48771;
    text-decoration: line-through;
  }

  .path.new {
    color: #73c991;
  }

  .arrow {
    color: var(--color-text-muted);
  }

  .diff-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: auto;
  }

  .hunk {
    margin-bottom: var(--space-md);
  }

  .hunk-header {
    padding: 4px 12px;
    background: rgba(0, 212, 255, 0.1);
    color: var(--color-accent);
    font-family: var(--font-mono);
    font-size: 12px;
    font-weight: 600;
    border-top: 1px solid var(--color-border);
    border-bottom: 1px solid var(--color-border);
  }

  .hunk-lines {
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
  }

  .diff-line {
    display: flex;
    align-items: center;
    white-space: pre;
  }

  .diff-line.line-added {
    background: rgba(115, 201, 145, 0.15);
  }

  .diff-line.line-removed {
    background: rgba(244, 135, 113, 0.15);
  }

  .diff-line.line-context {
    background: transparent;
  }

  .line-number {
    display: inline-block;
    width: 50px;
    padding: 0 8px;
    text-align: right;
    color: var(--color-text-muted);
    opacity: 0.6;
    user-select: none;
    flex-shrink: 0;
  }

  .line-number.old {
    border-right: 1px solid var(--color-border);
  }

  .line-number.new {
    border-right: 1px solid var(--color-border);
  }

  .line-prefix {
    display: inline-block;
    width: 20px;
    padding: 0 4px;
    text-align: center;
    font-weight: 600;
    user-select: none;
    flex-shrink: 0;
  }

  .line-added .line-prefix {
    color: #73c991;
  }

  .line-removed .line-prefix {
    color: #f48771;
  }

  .line-context .line-prefix {
    color: var(--color-text-muted);
  }

  .line-content {
    flex: 1;
    padding: 0 8px;
    color: var(--color-text-primary);
  }

  /* Scrollbar styling */
  .diff-content::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .diff-content::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
  }

  .diff-content::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
  }

  .diff-content::-webkit-scrollbar-thumb:hover {
    background: var(--color-border-focus);
  }
</style>
