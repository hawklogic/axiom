<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount } from 'svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import EditorArea from '$lib/components/EditorArea.svelte';
  import Panel from '$lib/components/Panel.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import Splash from '$lib/components/Splash.svelte';
  import { APP, PANELS } from '$lib/strings';

  let ready = false;
  let activePanel = 'files';

  onMount(async () => {
    // Simulate backend initialization
    await new Promise(resolve => setTimeout(resolve, 500));
    ready = true;
  });
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
              <div class="panel-placeholder">File Explorer</div>
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
          <Panel title={PANELS.terminal} collapsible>
            <div class="terminal-placeholder">Terminal</div>
          </Panel>
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
  }

  .panel-placeholder {
    padding: var(--space-md);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .terminal-placeholder {
    height: 100%;
    background: var(--color-bg-primary);
    padding: var(--space-sm);
    font-family: var(--font-mono);
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
  }
</style>
