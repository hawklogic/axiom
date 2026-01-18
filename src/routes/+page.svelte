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
  import { APP, PANELS } from '$lib/strings';
  import { editorPanes } from '$lib/stores/editorPanes';
  import { detectLanguage } from '$lib/utils/syntax';

  let ready = false;
  let activePanel = 'files';
  
  // Resizable panel state
  let bottomPanelHeight = 200;
  let terminalFlex = 3; // Terminal takes 3 parts, console takes 1 part
  let workspaceEl: HTMLElement;
  let bottomSplitEl: HTMLElement;
  
  // Svelte action for vertical resize
  function verticalResizer(node: HTMLElement) {
    function onMouseDown(e: MouseEvent) {
      e.preventDefault();
      console.log('[Resize] Vertical mousedown');
      
      function onMouseMove(e: MouseEvent) {
        if (!workspaceEl) return;
        const rect = workspaceEl.getBoundingClientRect();
        const newHeight = rect.bottom - e.clientY;
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
    } catch (err) {
      console.error('[Axiom] onMount error:', err);
      ready = true; // Still show UI even on error
    }
  });

  async function handleFileSelect(event: CustomEvent<{ path: string; name: string }>) {
    const { path, name } = event.detail;
    console.log('[Editor] File selected:', path, name);
    
    if (!isTauri()) {
      console.warn('[Editor] File reading requires Tauri runtime');
      return;
    }

    try {
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
    } catch (err) {
      console.error('[Editor] Failed to read file:', err);
    }
  }

</script>

{#if !ready}
  <Splash />
{:else}
  <div class="app-container">
    <div class="main-content">
      <Sidebar bind:activePanel />
      
      <div class="workspace" bind:this={workspaceEl}>
        <div class="editor-panels">
          {#if activePanel === 'files'}
            <Panel title={PANELS.fileExplorer}>
              <FileExplorer on:file-select={handleFileSelect} />
            </Panel>
          {:else if activePanel === 'git'}
            <Panel title={PANELS.sourceControl}>
              <div class="panel-placeholder">Source Control</div>
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
          {/if}
          
          <EditorSplitView />
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

  .workspace {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
  }

  .editor-panels {
    display: flex;
    flex: 1;
    overflow: hidden;
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
