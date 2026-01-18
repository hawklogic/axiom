<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { editorPanes } from '$lib/stores/editorPanes';
  import EditorPane from './EditorPane.svelte';
  
  let draggedFile: { paneId: string; filePath: string } | null = null;
  
  function handleDragStart(paneId: string, filePath: string) {
    draggedFile = { paneId, filePath };
  }
  
  function handleDrop(targetPaneId: string) {
    if (!draggedFile) return;
    
    if (draggedFile.paneId !== targetPaneId) {
      editorPanes.moveFile(draggedFile.paneId, targetPaneId, draggedFile.filePath);
    }
    
    draggedFile = null;
  }
  
  function handleSplitHorizontal() {
    editorPanes.splitPane('horizontal');
  }
  
  function handleSplitVertical() {
    editorPanes.splitPane('vertical');
  }
  
  function handleClosePane(paneId: string) {
    editorPanes.closePane(paneId);
  }
</script>

<div class="editor-split-view">
  <div class="split-controls">
    <button class="split-btn" on:click={handleSplitHorizontal} title="Split Horizontally" disabled={$editorPanes.panes.length >= 4}>
      ⬌
    </button>
    <button class="split-btn" on:click={handleSplitVertical} title="Split Vertically" disabled={$editorPanes.panes.length >= 4}>
      ⬍
    </button>
  </div>
  
  <div class="panes-container" class:horizontal={$editorPanes.splitDirection === 'horizontal'} class:vertical={$editorPanes.splitDirection === 'vertical'}>
    {#each $editorPanes.panes as pane (pane.id)}
      <div class="pane-wrapper">
        {#if $editorPanes.panes.length > 1}
          <button class="close-pane-btn" on:click={() => handleClosePane(pane.id)} title="Close Pane">
            ×
          </button>
        {/if}
        <EditorPane 
          {pane}
          onDragStart={handleDragStart}
          onDrop={handleDrop}
        />
      </div>
    {/each}
  </div>
</div>

<style>
  .editor-split-view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
  
  .split-controls {
    display: flex;
    gap: 4px;
    padding: 4px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }
  
  .split-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    color: var(--color-text-muted);
    font-size: 16px;
    transition: background 0.1s;
  }
  
  .split-btn:hover:not(:disabled) {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }
  
  .split-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
  
  .panes-container {
    flex: 1;
    display: flex;
    overflow: hidden;
  }
  
  .panes-container.horizontal {
    flex-direction: row;
  }
  
  .panes-container.vertical {
    flex-direction: column;
  }
  
  .pane-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    position: relative;
    min-width: 200px;
    min-height: 200px;
  }
  
  .panes-container.horizontal .pane-wrapper:not(:last-child) {
    border-right: 1px solid var(--color-border);
  }
  
  .panes-container.vertical .pane-wrapper:not(:last-child) {
    border-bottom: 1px solid var(--color-border);
  }
  
  .close-pane-btn {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    font-size: 16px;
    color: var(--color-text-muted);
    background: var(--color-bg-secondary);
    z-index: 10;
    opacity: 0;
    transition: opacity 0.2s;
  }
  
  .pane-wrapper:hover .close-pane-btn {
    opacity: 1;
  }
  
  .close-pane-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }
</style>
