<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { workspace, hasWorkspace } from '$lib/stores/workspace';
  import TreeNode from './TreeNode.svelte';
  import { BUTTONS, EMPTY } from '$lib/strings';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{
    'file-select': { path: string; name: string };
  }>();

  async function handleOpenFolder() {
    await workspace.openFolder();
  }

  async function handleToggle(event: CustomEvent<{ path: string }>) {
    await workspace.toggleNode(event.detail.path);
  }

  function handleSelect(event: CustomEvent<{ path: string; name: string }>) {
    dispatch('file-select', event.detail);
  }
</script>

<div class="file-explorer">
  {#if !$hasWorkspace}
    <div class="empty-state">
      <p class="empty-text">{EMPTY.noFiles}</p>
      <button class="open-folder-btn" on:click={handleOpenFolder}>
        {BUTTONS.open} Folder
      </button>
      <p class="hint">Or drag a folder here</p>
    </div>
  {:else}
    <div class="workspace-header">
      <span class="workspace-name">{$workspace.name}</span>
      <button class="icon-btn" on:click={() => workspace.refresh()} title="Refresh">
        ↻
      </button>
    </div>
    <div class="tree">
      {#each $workspace.tree as node (node.path)}
        <TreeNode 
          {node} 
          depth={0}
          on:toggle={handleToggle}
          on:select={handleSelect}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .file-explorer {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 24px;
    text-align: center;
    gap: 12px;
  }

  .empty-text {
    color: var(--color-text-muted);
    font-size: 13px;
  }

  .open-folder-btn {
    padding: 8px 24px;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: 6px;
    font-weight: 500;
    transition: background 0.15s;
  }

  .open-folder-btn:hover {
    background: var(--color-accent-hover);
  }

  .hint {
    color: var(--color-text-muted);
    font-size: 11px;
    opacity: 0.7;
  }

  .workspace-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border-bottom: 1px solid var(--color-border);
    background: var(--color-bg-secondary);
  }

  .workspace-name {
    font-weight: 600;
    font-size: 11px;
    color: var(--color-text-primary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .icon-btn {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    color: var(--color-text-muted);
    font-size: 14px;
  }

  .icon-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .tree {
    flex: 1;
    overflow-y: auto;
    padding: 4px;
  }
</style>
