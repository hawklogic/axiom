<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import EditorSplitView from '$lib/components/EditorSplitView.svelte';
  import Panel from '$lib/components/Panel.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import Splash from '$lib/components/Splash.svelte';
  import FileExplorer from '$lib/components/FileExplorer.svelte';
  import Terminal from '$lib/components/Terminal.svelte';
  import MiniConsole from '$lib/components/MiniConsole.svelte';
  import AboutPanel from '$lib/components/AboutPanel.svelte';
  import SourceControl from '$lib/components/SourceControl.svelte';
  import ArmToolchainPanel from '$lib/components/arm/ArmToolchainPanel.svelte';
  import { APP, PANELS } from '$lib/strings';
  import { editorPanes } from '$lib/stores/editorPanes';
  import { ideStatus } from '$lib/stores/status';
  import { detectLanguage } from '$lib/utils/syntax';

  let ready = false;
  let activePanel = 'files';
  let leftPanelVisible = true;
  let sourceControlRef: any;
  
  // Resizable panel state
  let leftPanelWidth = 300;
  let bottomPanelHeight = 200;
  let terminalFlex = 3; // Terminal takes 3 parts, console takes 1 part
  let bottomSplitEl: HTMLElement;
  let mainContentEl: HTMLElement;
  
  // Svelte action for left panel resize
  function leftPanelResizer(node: HTMLElement) {
    function onMouseDown(e: MouseEvent) {
      e.preventDefault();
      console.log('[Resize] Left panel mousedown');
      
      function onMouseMove(e: MouseEvent) {
        if (!mainContentEl) return;
        const rect = mainContentEl.getBoundingClientRect();
        const newWidth = e.clientX - rect.left - 48; // Subtract sidebar width
        leftPanelWidth = Math.max(150, Math.min(newWidth, 600));
      }
      
      function onMouseUp() {
        document.removeEventListener('mousemove', onMouseMove);
        document.removeEventListener('mouseup', onMouseUp);
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      }
      
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
      document.body.style.cursor = 'ew-resize';
      document.body.style.userSelect = 'none';
    }
    
    node.addEventListener('mousedown', onMouseDown);
    return {
      destroy() {
        node.removeEventListener('mousedown', onMouseDown);
      }
    };
  }
  
  // Svelte action for vertical resize
  function verticalResizer(node: HTMLElement) {
    function onMouseDown(e: MouseEvent) {
      e.preventDefault();
      console.log('[Resize] Vertical mousedown');
      
      function onMouseMove(e: MouseEvent) {
        const rect = document.querySelector('.app-container')?.getBoundingClientRect();
        if (!rect) return;
        const newHeight = rect.bottom - e.clientY - 22; // Subtract status bar height
        bottomPanelHeight = Math.max(100, Math.min(newHeight, rect.height - 100));
      }
      
      function onMouseUp() {
        document.removeEventListener('mousemove', onMouseMove);
        document.removeEventListener('mouseup', onMouseUp);
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      }
      
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
      document.body.style.cursor = 'ns-resize';
      document.body.style.userSelect = 'none';
    }
    
    node.addEventListener('mousedown', onMouseDown);
    return {
      destroy() {
        node.removeEventListener('mousedown', onMouseDown);
      }
    };
  }
  
  // Svelte action for horizontal resize
  function horizontalResizer(node: HTMLElement) {
    function onMouseDown(e: MouseEvent) {
      e.preventDefault();
      console.log('[Resize] Horizontal mousedown');
      
      function onMouseMove(e: MouseEvent) {
        if (!bottomSplitEl) return;
        const rect = bottomSplitEl.getBoundingClientRect();
        const relativeX = e.clientX - rect.left;
        const ratio = Math.max(0.1, Math.min(0.9, relativeX / rect.width));
        terminalFlex = ratio / (1 - ratio);
      }
      
      function onMouseUp() {
        document.removeEventListener('mousemove', onMouseMove);
        document.removeEventListener('mouseup', onMouseUp);
        document.body.style.cursor = '';
        document.body.style.userSelect = '';
      }
      
      document.addEventListener('mousemove', onMouseMove);
      document.addEventListener('mouseup', onMouseUp);
      document.body.style.cursor = 'ew-resize';
      document.body.style.userSelect = 'none';
    }
    
    node.addEventListener('mousedown', onMouseDown);
    return {
      destroy() {
        node.removeEventListener('mousedown', onMouseDown);
      }
    };
  }

  /** Check if running inside Tauri (works with Tauri 2.x) */
  function isTauri(): boolean {
    if (!browser || typeof window === 'undefined') return false;
    // Tauri 2.x uses __TAURI_INTERNALS__, Tauri 1.x uses __TAURI__
    return '__TAURI__' in window || '__TAURI_INTERNALS__' in window;
  }

  onMount(async () => {
    console.log('[Axiom] onMount started');
    try {
      // Brief delay for visual feedback
      await new Promise(resolve => setTimeout(resolve, 300));
      console.log('[Axiom] Setting ready = true');
      ready = true;
      
      // Listen for menu events from Tauri
      if (isTauri()) {
        const { listen } = await import('@tauri-apps/api/event');
        
        // Handle "About" menu item
        await listen('show-about', () => {
          console.log('[Menu] Show about triggered');
          activePanel = 'about';
        });
      }
    } catch (err) {
      console.error('[Axiom] onMount error:', err);
      ready = true; // Still show UI even on error
    }
  });

  async function handleFileSelect(event: CustomEvent<{ path: string; name: string }>) {
    const { path, name } = event.detail;
    console.log('[Editor] File selected:', path, name);
    
    // Check if file is already open in any pane
    for (const pane of $editorPanes.panes) {
      const fileIndex = pane.files.findIndex(f => f.path === path);
      if (fileIndex >= 0) {
        console.log('[Editor] File already open in', pane.id, 'at index', fileIndex);
        // Switch to that file
        editorPanes.setActiveFile(pane.id, fileIndex);
        // Flash the tab
        editorPanes.flashTab(pane.id, path);
        return;
      }
    }
    
    if (!isTauri()) {
      console.warn('[Editor] File reading requires Tauri runtime');
      ideStatus.error('File reading requires Tauri runtime');
      setTimeout(() => ideStatus.ready(), 3000);
      return;
    }

    try {
      ideStatus.loading(`Opening ${name}...`);
      console.log('[Editor] Reading file...');
      const { invoke } = await import('@tauri-apps/api/core');
      const content = await invoke<string>('read_file', { path });
      console.log('[Editor] File read, length:', content.length);
      
      // Open file in the first pane
      const firstPane = $editorPanes.panes[0];
      editorPanes.openFile(firstPane.id, {
        path,
        name,
        content,
        language: detectLanguage(name),
        modified: false,
        cursor: { line: 1, column: 1 },
      });
      console.log('[Editor] File opened in editor');
      ideStatus.ready();
    } catch (err) {
      console.error('[Editor] Failed to read file:', err);
      ideStatus.error(`Failed to open ${name}: ${err}`);
      setTimeout(() => ideStatus.ready(), 3000);
    }
  }

</script>

{#if !ready}
  <Splash />
{:else}
  <div class="app-container">
    <div class="main-content" bind:this={mainContentEl}>
      <Sidebar bind:activePanel />
      
      {#if leftPanelVisible}
        <div class="left-panel" style="width: {leftPanelWidth}px;">
          {#if activePanel === 'files'}
            <Panel title={PANELS.fileExplorer}>
              <FileExplorer on:file-select={handleFileSelect} />
            </Panel>
          {:else if activePanel === 'git'}
            <Panel title={PANELS.sourceControl}>
              <svelte:fragment slot="header-actions">
                <button class="panel-action-button" on:click={() => sourceControlRef?.refresh()} title="Refresh">
                  ↻
                </button>
              </svelte:fragment>
              <SourceControl bind:this={sourceControlRef} />
            </Panel>
          {:else if activePanel === 'ast'}
            <Panel title={PANELS.astViewer}>
              <div class="panel-placeholder">AST Viewer</div>
            </Panel>
          {:else if activePanel === 'debug'}
            <Panel title={PANELS.debugPanel}>
              <div class="panel-placeholder">Debug Panel</div>
            </Panel>
          {:else if activePanel === 'settings'}
            <Panel title={PANELS.settings}>
              <div class="panel-placeholder">Settings</div>
            </Panel>
          {:else if activePanel === 'about'}
            <Panel title="About Axiom">
              <AboutPanel />
            </Panel>
          {:else if activePanel === 'arm'}
            <ArmToolchainPanel />
          {/if}
        </div>
        
        <!-- Left panel resize handle -->
        <div class="resize-handle-left" use:leftPanelResizer></div>
      {/if}
      
      <div class="editor-area">
        <EditorSplitView bind:leftPanelVisible />
      </div>
    </div>
    
    <!-- Vertical resize handle -->
    <div class="resize-handle-vertical" use:verticalResizer></div>
    
    <div class="bottom-panel" style="height: {bottomPanelHeight}px;">
      <div class="bottom-split" bind:this={bottomSplitEl}>
        <div class="terminal-pane" style="flex: {terminalFlex};">
          <div class="pane-header">Terminal</div>
          <div class="pane-content">
            <Terminal />
          </div>
        </div>
        
        <!-- Horizontal resize handle -->
        <div class="resize-handle-horizontal" use:horizontalResizer></div>
        
        <div class="console-pane" style="flex: 1;">
          <div class="pane-header">Console</div>
          <div class="pane-content">
            <MiniConsole />
          </div>
        </div>
      </div>
    </div>
    
    <StatusBar />
  </div>
{/if}

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }
  
  .editor-area {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .panel-action-button {
    padding: 2px 6px;
    font-size: 14px;
    color: var(--color-text-muted);
    transition: color 0.15s;
    background: transparent;
    border: none;
    cursor: pointer;
  }

  .panel-action-button:hover {
    color: var(--color-text-primary);
  }

  .left-panel {
    min-width: 150px;
    max-width: 600px;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    overflow: hidden;
  }
  
  .resize-handle-left {
    width: 4px;
    background: var(--color-border);
    cursor: ew-resize;
    flex-shrink: 0;
    transition: background 0.15s;
    position: relative;
    z-index: 10;
  }
  
  .resize-handle-left:hover,
  .resize-handle-left:active {
    background: var(--color-accent);
  }

  .resize-handle-vertical {
    height: 8px;
    background: var(--color-border);
    cursor: ns-resize;
    flex-shrink: 0;
    transition: background 0.15s;
    position: relative;
    z-index: 10;
  }

  .resize-handle-vertical:hover,
  .resize-handle-vertical:active {
    background: var(--color-accent);
  }

  .resize-handle-horizontal {
    width: 8px;
    background: var(--color-border);
    cursor: ew-resize;
    flex-shrink: 0;
    transition: background 0.15s;
    position: relative;
    z-index: 10;
  }

  .resize-handle-horizontal:hover,
  .resize-handle-horizontal:active {
    background: var(--color-accent);
  }

  .bottom-panel {
    min-height: 100px;
    max-height: calc(100vh - 150px);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .bottom-split {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .terminal-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 100px;
  }

  .console-pane {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    min-width: 100px;
  }

  .pane-header {
    padding: 4px 8px;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-muted);
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    user-select: none;
  }

  .pane-content {
    flex: 1;
    overflow: hidden;
  }

  .panel-placeholder {
    padding: var(--space-md);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

</style>
