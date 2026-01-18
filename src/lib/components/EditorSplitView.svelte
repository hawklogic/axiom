<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { editorPanes } from '$lib/stores/editorPanes';
  import { consoleStore } from '$lib/stores/console';
  import { workspace } from '$lib/stores/workspace';
  import EditorPane from './EditorPane.svelte';
  import { onMount, onDestroy } from 'svelte';
  
  export let leftPanelVisible: boolean;
  
  let draggedFile: { paneId: string; filePath: string } | null = null;
  let dragOverPaneId: string | null = null;
  
  async function handleOpenFolder() {
    const success = await workspace.openFolder();
    if (success) {
      consoleStore.log('info', 'workspace', 'Folder opened successfully');
    } else {
      consoleStore.log('warn', 'workspace', 'No folder selected');
    }
  }

  async function handleSaveAndCloseAll() {
    // Collect all files first (before any modifications)
    const allFilesToClose: Array<{ paneId: string; filePath: string; file: any }> = [];
    const modifiedFiles: any[] = [];
    
    for (const pane of $editorPanes.panes) {
      for (const file of pane.files) {
        allFilesToClose.push({ paneId: pane.id, filePath: file.path, file });
        if (file.modified) {
          modifiedFiles.push(file);
        }
      }
    }
    
    if (modifiedFiles.length > 0) {
      consoleStore.log('info', 'editor', `Saving ${modifiedFiles.length} modified files...`);
      // Save all modified files
      for (const file of modifiedFiles) {
        try {
          const { invoke } = await import('@tauri-apps/api/core');
          await invoke('write_file', { path: file.path, content: file.content });
          editorPanes.markSaved(file.path);
        } catch (err) {
          consoleStore.log('error', 'editor', `Failed to save ${file.path}: ${err}`);
        }
      }
    }
    
    // Close all files (now safe since we collected them first)
    consoleStore.log('info', 'editor', `Closing ${allFilesToClose.length} tabs...`);
    for (const { paneId, filePath } of allFilesToClose) {
      editorPanes.closeFile(paneId, filePath);
    }
    consoleStore.log('info', 'editor', 'All tabs closed');
  }
  
  onMount(() => {
    console.log('[EditorSplitView] Mounted, panes:', $editorPanes.panes.length);
    consoleStore.log('info', 'editor', `Split view mounted with ${$editorPanes.panes.length} pane(s)`);
  });
  
  onDestroy(() => {
    // Cleanup if needed
  });
  
  function setupDropHandlers(node: HTMLElement) {
    const paneId = node.getAttribute('data-drop-target');
    console.log('[SETUP] Setting up drop handlers for overlay, pane:', paneId);
    consoleStore.log('info', 'editor', `Setup drop handlers for ${paneId}`);
    
    const handleDragEnter = (e: DragEvent) => {
      console.log('[NATIVE DRAGENTER] Overlay');
      consoleStore.log('info', 'editor', 'NATIVE DRAGENTER');
      e.preventDefault();
    };
    
    const handleDragOver = (e: DragEvent) => {
      console.log('[NATIVE DRAGOVER] Overlay');
      consoleStore.log('info', 'editor', 'NATIVE DRAGOVER');
      e.preventDefault();
      if (e.dataTransfer) {
        e.dataTransfer.dropEffect = 'move';
      }
    };
    
    const handleDropEvent = (e: DragEvent) => {
      console.log('[NATIVE DROP] Overlay, pane:', paneId);
      consoleStore.log('info', 'editor', `NATIVE DROP on ${paneId}`);
      e.preventDefault();
      e.stopPropagation();
      
      if (paneId && draggedFile) {
        console.log('[NATIVE DROP] Calling handleDrop with:', paneId);
        handleDrop(paneId);
      }
    };
    
    node.addEventListener('dragenter', handleDragEnter);
    node.addEventListener('dragover', handleDragOver);
    node.addEventListener('drop', handleDropEvent);
    
    console.log('[SETUP] Listeners attached to overlay');
    
    return {
      destroy() {
        node.removeEventListener('dragenter', handleDragEnter);
        node.removeEventListener('dragover', handleDragOver);
        node.removeEventListener('drop', handleDropEvent);
      }
    };
  }
  
  function handleGlobalDragOver(e: DragEvent) {
    console.log('[DRAGOVER] Event on overlay');
    consoleStore.log('info', 'editor', 'DRAGOVER on overlay');
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'move';
    }
  }
  
  function handleGlobalDrop(e: DragEvent) {
    console.log('[DROP] Event on overlay, draggedFile:', draggedFile);
    consoleStore.log('info', 'editor', 'DROP on overlay!');
    e.preventDefault();
    e.stopPropagation();
    
    const target = e.target as HTMLElement;
    const paneWrapper = target.closest('.pane-wrapper');
    if (paneWrapper) {
      const paneId = paneWrapper.getAttribute('data-pane-id');
      if (paneId && draggedFile) {
        console.log('[DROP] Moving to pane:', paneId);
        consoleStore.log('info', 'editor', `Moving to ${paneId}`);
        handleDrop(paneId);
      }
    }
  }
  
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
    console.log('[EditorSplitView] draggedFile set to:', draggedFile);
    consoleStore.log('info', 'editor', `Overlay should appear on other panes`);
  }
  
  function handleDragEnd() {
    console.log('[EditorSplitView] handleDragEnd called, clearing draggedFile');
    consoleStore.log('info', 'editor', 'Drag ended');
    draggedFile = null;
    dragOverPaneId = null;
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
    dragOverPaneId = null;
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
    <button 
      class="toggle-panel-btn" 
      on:click={() => leftPanelVisible = !leftPanelVisible}
      title={leftPanelVisible ? 'Hide sidebar panel' : 'Show sidebar panel'}
    >
      <span class="toggle-icon">{leftPanelVisible ? '◀' : '▶'}</span>
      <span class="toggle-label">{leftPanelVisible ? 'Hide Panel' : 'Show Panel'}</span>
    </button>
    
    <button 
      class="open-folder-btn" 
      on:click={handleOpenFolder}
      title="Open Folder"
    >
      <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M1 3.5C1 2.67157 1.67157 2 2.5 2H5.5L7 3.5H13.5C14.3284 3.5 15 4.17157 15 5V12.5C15 13.3284 14.3284 14 13.5 14H2.5C1.67157 14 1 13.3284 1 12.5V3.5Z" stroke="currentColor" stroke-width="1.5" fill="none"/>
      </svg>
      <span class="btn-label">Open Folder</span>
    </button>

    {#if $editorPanes.panes.some(p => p.files.length > 0)}
      <button 
        class="save-close-all-btn"
        on:click={handleSaveAndCloseAll}
        title="Save and close all open tabs"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M3 3L13 13M13 3L3 13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <span class="btn-label">Save & Close All</span>
      </button>
    {/if}
    
    <div class="split-buttons">
      <button class="split-btn" on:click={handleSplitHorizontal} title="Split Editor Side by Side" disabled={$editorPanes.panes.length >= 4}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="1" y="2" width="6" height="12" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="9" y="2" width="6" height="12" stroke="currentColor" stroke-width="1.5" fill="none"/>
        </svg>
        <span class="btn-label">Split Right</span>
      </button>
      <button class="split-btn" on:click={handleSplitVertical} title="Split Editor Top and Bottom" disabled={$editorPanes.panes.length >= 4}>
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
          <rect x="2" y="1" width="12" height="6" stroke="currentColor" stroke-width="1.5" fill="none"/>
          <rect x="2" y="9" width="12" height="6" stroke="currentColor" stroke-width="1.5" fill="none"/>
        </svg>
        <span class="btn-label">Split Down</span>
      </button>
    </div>
    {#if $editorPanes.panes.length > 1}
      <span class="pane-count">{$editorPanes.panes.length} panes</span>
    {/if}
    {#if draggedFile}
      <span class="drag-status">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
          <circle cx="6" cy="6" r="5"/>
        </svg>
        Dragging: {draggedFile.filePath.split('/').pop()}
      </span>
    {/if}
  </div>
  
  <div class="panes-container" class:horizontal={$editorPanes.splitDirection === 'horizontal'} class:vertical={$editorPanes.splitDirection === 'vertical'}>
    {#each $editorPanes.panes as pane (pane.id)}
      <div 
        class="pane-wrapper"
        data-pane-id={pane.id}
        role="region"
        aria-label="Editor pane drop zone"
        class:drag-over={draggedFile && draggedFile.paneId !== pane.id}
      >
        {#if draggedFile && draggedFile.paneId !== pane.id}
          <div 
            class="drop-overlay"
            data-drop-target={pane.id}
            role="region"
            aria-label="Drop zone"
            use:setupDropHandlers
          >
            <div class="drop-message">Drop file here</div>
          </div>
        {/if}
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
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
  }
  
  .toggle-panel-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    border-radius: 4px;
    color: var(--color-text-secondary);
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s;
    border: 1px solid var(--color-border);
    margin-right: auto;
  }
  
  .toggle-panel-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
    border-color: var(--color-accent);
  }
  
  .toggle-icon {
    font-size: 10px;
  }
  
  .toggle-label {
    white-space: nowrap;
  }
  
  .open-folder-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    border-radius: 4px;
    color: var(--color-text-secondary);
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s;
    border: 1px solid var(--color-border);
  }
  
  .open-folder-btn svg {
    flex-shrink: 0;
  }
  
  .open-folder-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
    border-color: var(--color-accent);
  }

  .save-close-all-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    border-radius: 4px;
    color: var(--color-text-secondary);
    font-size: 12px;
    font-weight: 500;
    transition: all 0.15s;
    border: 1px solid var(--color-border);
  }
  
  .save-close-all-btn svg {
    flex-shrink: 0;
  }
  
  .save-close-all-btn:hover {
    background: rgba(244, 135, 113, 0.15);
    color: #f48771;
    border-color: #f48771;
  }
  
  .split-buttons {
    display: flex;
    gap: 6px;
    margin-left: auto;
  }
  
  .split-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    border-radius: 4px;
    color: var(--color-text-muted);
    font-size: 12px;
    transition: all 0.15s;
    border: 1px solid transparent;
  }
  
  .split-btn svg {
    flex-shrink: 0;
  }
  
  .btn-label {
    font-weight: 500;
    white-space: nowrap;
  }
  
  .split-btn:hover:not(:disabled) {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
    border-color: var(--color-border);
  }
  
  .split-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  
  .pane-count {
    padding: 4px 10px;
    background: var(--color-bg-tertiary);
    color: var(--color-text-muted);
    border-radius: 4px;
    font-size: 11px;
    font-weight: 500;
    border: 1px solid var(--color-border);
  }
  
  .drag-status {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-left: auto;
    padding: 5px 10px;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
  }
  
  .drag-status svg {
    flex-shrink: 0;
    animation: pulse 1s ease-in-out infinite;
  }
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
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
  
  .drop-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 9999;
    background: rgba(88, 166, 255, 0.08);
    border: 2px solid var(--color-accent);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: all !important;
    cursor: copy;
    transition: background 0.2s ease;
  }
  
  .drop-overlay:hover {
    background: rgba(88, 166, 255, 0.12);
  }
  
  @keyframes pulse-border {
    0%, 100% { 
      background: rgba(88, 166, 255, 0.15);
      outline-offset: -8px;
    }
    50% { 
      background: rgba(88, 166, 255, 0.25);
      outline-offset: -6px;
    }
  }
  
  .drop-message {
    padding: 12px 24px;
    background: var(--color-bg-secondary);
    color: var(--color-text-primary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-weight: 500;
    font-size: 13px;
    pointer-events: none;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    opacity: 0.95;
  }
  
  .pane-wrapper.drag-over::before {
    pointer-events: auto;
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
