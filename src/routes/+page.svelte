<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import EditorArea from '$lib/components/EditorArea.svelte';
  import Panel from '$lib/components/Panel.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import Splash from '$lib/components/Splash.svelte';
  import FileExplorer from '$lib/components/FileExplorer.svelte';
  import Terminal from '$lib/components/Terminal.svelte';
  import MiniConsole from '$lib/components/MiniConsole.svelte';
  import { APP, PANELS } from '$lib/strings';
  import { editorStore } from '$lib/stores';
  import { invoke } from '@tauri-apps/api/core';

  let ready = false;
  let activePanel = 'files';
  let activeBottomTab: 'terminal' | 'console' = 'terminal';

  onMount(async () => {
    // Simulate backend initialization
    await new Promise(resolve => setTimeout(resolve, 500));
    ready = true;
  });

  function getLanguage(name: string): 'c' | 'cpp' | 'h' | 'hpp' | 'unknown' {
    const ext = name.split('.').pop()?.toLowerCase() || '';
    switch (ext) {
      case 'c': return 'c';
      case 'cpp':
      case 'cc':
      case 'cxx': return 'cpp';
      case 'h': return 'h';
      case 'hpp':
      case 'hxx': return 'hpp';
      default: return 'unknown';
    }
  }

  async function handleFileSelect(event: CustomEvent<{ path: string; name: string }>) {
    const { path, name } = event.detail;
    try {
      const content = await invoke<string>('read_file', { path });
      editorStore.openFile({
        path,
        name,
        content,
        language: getLanguage(name),
        modified: false,
        cursor: { line: 1, column: 1 },
      });
    } catch (err) {
      console.error('Failed to read file:', err);
    }
  }
</script>

{#if !ready}
  <Splash />
{:else}
  <div class="app-container">
    <div class="main-content">
      <Sidebar bind:activePanel />
      
      <div class="workspace">
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
          
          <EditorArea />
        </div>
        
        <div class="bottom-panel">
          <div class="bottom-tabs">
            <button 
              class="bottom-tab" 
              class:active={activeBottomTab === 'terminal'}
              on:click={() => activeBottomTab = 'terminal'}
            >
              Terminal
            </button>
            <button 
              class="bottom-tab" 
              class:active={activeBottomTab === 'console'}
              on:click={() => activeBottomTab = 'console'}
            >
              Console
            </button>
          </div>
          <div class="bottom-content">
            {#if activeBottomTab === 'terminal'}
              <Terminal />
            {:else}
              <MiniConsole />
            {/if}
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

  .bottom-panel {
    height: 200px;
    border-top: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
  }

  .bottom-tabs {
    display: flex;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    padding: 0 4px;
    gap: 2px;
  }

  .bottom-tab {
    padding: 6px 12px;
    font-size: 11px;
    font-weight: 500;
    color: var(--color-text-muted);
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: color 0.1s, border-color 0.1s;
  }

  .bottom-tab:hover {
    color: var(--color-text-secondary);
  }

  .bottom-tab.active {
    color: var(--color-text-primary);
    border-bottom-color: var(--color-accent);
  }

  .bottom-content {
    flex: 1;
    overflow: hidden;
  }

  .panel-placeholder {
    padding: var(--space-md);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

</style>
