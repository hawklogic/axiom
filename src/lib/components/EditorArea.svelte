<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { EMPTY } from '$lib/strings';
  
  // Placeholder for CodeMirror integration
  let hasOpenFile = false;
</script>

<div class="editor-area">
  {#if hasOpenFile}
    <div class="editor-tabs">
      <div class="tab active">main.c</div>
    </div>
    <div class="editor-content">
      <!-- CodeMirror will be mounted here -->
    </div>
  {:else}
    <div class="empty-state">
      <p>{EMPTY.noFiles}</p>
      <p class="hint">Open a file to begin.</p>
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
    padding: 0 var(--space-sm);
  }

  .tab {
    padding: var(--space-xs) var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-bottom: none;
    border-radius: var(--radius-sm) var(--radius-sm) 0 0;
    cursor: pointer;
  }

  .tab.active {
    color: var(--color-text-primary);
    background: var(--color-bg-primary);
  }

  .editor-content {
    flex: 1;
    overflow: auto;
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
    margin-bottom: var(--space-sm);
  }

  .empty-state .hint {
    font-size: var(--font-size-sm);
  }
</style>
