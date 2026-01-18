<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { editorPanes } from '$lib/stores/editorPanes';
  import { consoleStore } from '$lib/stores/console';
  import EditorPane from './EditorPane.svelte';
  import { onMount } from 'svelte';
  
  let draggedFile: { paneId: string; filePath: string } | null = null;
  
  onMount(() => {
    console.log('[EditorSplitView] Mounted, panes:', $editorPanes.panes.length);
    consoleStore.log('info', 'editor', `Split view mounted with ${$editorPanes.panes.length} pane(s)`);
  });
  
  $: {
    console.log('[EditorSplitView] Panes updated:', $editorPanes.panes.length, 'split:', $editorPanes.splitDirection);
    if ($editorPanes.panes.length > 1) {
      consoleStore.log('info', 'editor', `Panes: ${$editorPanes.panes.length}, Split: ${$editorPanes.splitDirection}`);
    }
  }
  
  function handleDragStart(paneId: string, filePath: string) {
    console.log('[EditorSplitView] handleDragStart called:', paneId, filePath);
    consoleStore.log('info', 'editor', `Drag started: ${filePath.split('/').pop()} from ${paneId}`);
    draggedFile = { paneId, filePath };
  }
  
  function handleDragEnd() {
    console.log('[EditorSplitView] handleDragEnd called, clearing draggedFile');
    consoleStore.log('info', 'editor', 'Drag ended');
    draggedFile = null;
  }
  
  function handleDrop(targetPaneId: string) {
    console.log('[EditorSplitView] handleDrop called:', targetPaneId, 'draggedFile:', draggedFile);
    if (!draggedFile) {
      consoleStore.log('warn', 'editor', 'Drop called but no file being dragged');
      return;
    }
    
    if (draggedFile.paneId !== targetPaneId) {
      console.log('[EditorSplitView] Moving file from', draggedFile.paneId, 'to', targetPaneId);
      consoleStore.log('info', 'editor', `Moving ${draggedFile.filePath.split('/').pop()} to ${targetPaneId}`);
      editorPanes.moveFile(draggedFile.paneId, targetPaneId, draggedFile.filePath);
    } else {
      consoleStore.log('info', 'editor', 'Dropped on same pane, no action');
    }
    
    draggedFile = null;
  }
  
  function handleSplitHorizontal() {
    console.log('[EditorSplitView] Split horizontal clicked');
    consoleStore.log('info', 'editor', 'Split horizontal');
    editorPanes.splitPane('horizontal');
  }
  
  function handleSplitVertical() {
    console.log('[EditorSplitView] Split vertical clicked');
    consoleStore.log('info', 'editor', 'Split vertical');
    editorPanes.splitPane('vertical');
  }
  
  function handleClosePane(paneId: string) {
    editorPanes.closePane(paneId);
  }
</script>

<div class="editor-split-view">
  <div class="split-controls">
    <button class="split-btn" on:click={handleSplitHorizontal} title="Split Horizontally" disabled={$editorPanes.panes.length >= 4}>
      ⬌ ({$editorPanes.panes.length})
    </button>
    <button class="split-btn" on:click={handleSplitVertical} title="Split Vertically" disabled={$editorPanes.panes.length >= 4}>
      ⬍ ({$editorPanes.panes.length})
    </button>
    {#if draggedFile}
      <span class="drag-status">Dragging: {draggedFile.filePath.split('/').pop()}</span>
    {/if}
  </div>
  
  <div class="panes-container" class:horizontal={$editorPanes.splitDirection === 'horizontal'} class:vertical={$editorPanes.splitDirection === 'vertical'}>
    {#each $editorPanes.panes as pane (pane.id)}
      <div 
        class="pane-wrapper"
        class:drag-over={draggedFile && draggedFile.paneId !== pane.id}
        on:dragover={(e) => {
          e.preventDefault();
          if (e.dataTransfer) e.dataTransfer.dropEffect = 'move';
          consoleStore.log('debug', 'editor', `Drag over ${pane.id}`);
        }}
        on:drop={(e) => {
          e.preventDefault();
          consoleStore.log('info', 'editor', `Drop on ${pane.id}`);
          handleDrop(pane.id);
        }}
      >
        {#if $editorPanes.panes.length > 1}
          <button class="close-pane-btn" on:click={() => handleClosePane(pane.id)} title="Close Pane">
            ×
          </button>
        {/if}
        <EditorPane 
          {pane}
          onDragStart={handleDragStart}
          onDragEnd={handleDragEnd}
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
  
  .drag-status {
    margin-left: 12px;
    padding: 4px 8px;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
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
    transition: outline 0.2s;
  }
  
  .pane-wrapper.drag-over {
    outline: 3px solid var(--color-accent);
    outline-offset: -3px;
    background: rgba(88, 166, 255, 0.05);
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
