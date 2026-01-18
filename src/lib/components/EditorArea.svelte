<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { EMPTY } from '$lib/strings';
  import { editorStore } from '$lib/stores/editor';
  
  const { files, activeIndex, activeFile } = editorStore;
  
  function selectTab(index: number) {
    activeIndex.set(index);
  }
  
  function closeTab(e: MouseEvent, path: string) {
    e.stopPropagation();
    editorStore.closeFile(path);
  }
  
  function getLanguageLabel(lang: string): string {
    switch (lang) {
      case 'c': return 'C';
      case 'cpp': return 'C++';
      case 'h': return 'C Header';
      case 'hpp': return 'C++ Header';
      default: return 'Text';
    }
  }
</script>

<div class="editor-area">
  {#if $files.length > 0}
    <div class="editor-tabs">
      {#each $files as file, i (file.path)}
        <button 
          class="tab" 
          class:active={i === $activeIndex}
          on:click={() => selectTab(i)}
          title={file.path}
        >
          <span class="tab-name">{file.name}</span>
          {#if file.modified}
            <span class="modified-dot">●</span>
          {/if}
          <button class="close-btn" on:click={(e) => closeTab(e, file.path)} title="Close">
            ×
          </button>
        </button>
      {/each}
    </div>
    <div class="editor-content">
      {#if $activeFile}
        <div class="file-info">
          <span class="file-path">{$activeFile.path}</span>
          <span class="file-lang">{getLanguageLabel($activeFile.language)}</span>
        </div>
        <pre class="code-view"><code>{$activeFile.content}</code></pre>
      {/if}
    </div>
  {:else}
    <div class="empty-state">
      <p>{EMPTY.noFiles}</p>
      <p class="hint">Open a file from the explorer to begin.</p>
    </div>
  {/if}
</div>

<style>
  .editor-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-bg-primary);
  }

  .editor-tabs {
    display: flex;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    height: 35px;
    align-items: flex-end;
    padding: 0 4px;
    gap: 2px;
    overflow-x: auto;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    font-size: 12px;
    color: var(--color-text-muted);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    white-space: nowrap;
    max-width: 160px;
  }

  .tab:hover {
    background: var(--color-bg-hover);
  }

  .tab.active {
    color: var(--color-text-primary);
    background: var(--color-bg-primary);
    border-bottom-color: var(--color-bg-primary);
  }

  .tab-name {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .modified-dot {
    color: var(--color-accent);
    font-size: 10px;
  }

  .close-btn {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    font-size: 14px;
    color: var(--color-text-muted);
    opacity: 0;
    transition: opacity 0.1s;
  }

  .tab:hover .close-btn {
    opacity: 1;
  }

  .close-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .editor-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .file-info {
    display: flex;
    justify-content: space-between;
    padding: 4px 12px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    font-size: 11px;
  }

  .file-path {
    color: var(--color-text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-lang {
    color: var(--color-accent);
    font-weight: 500;
  }

  .code-view {
    flex: 1;
    margin: 0;
    padding: 12px;
    overflow: auto;
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    white-space: pre;
    tab-size: 4;
  }

  .code-view code {
    display: block;
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted);
  }

  .empty-state p {
    margin-bottom: 8px;
  }

  .empty-state .hint {
    font-size: 12px;
  }

  /* Scrollbar styling */
  .code-view::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .code-view::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
  }

  .code-view::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
  }

  .code-view::-webkit-scrollbar-thumb:hover {
    background: var(--color-border-focus);
  }
</style>
