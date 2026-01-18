<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { TreeNode } from '$lib/stores/workspace';

  export let node: TreeNode;
  export let depth: number = 0;

  const dispatch = createEventDispatcher<{
    'toggle': { path: string };
    'select': { path: string; name: string };
  }>();

  function handleClick() {
    if (node.is_dir) {
      dispatch('toggle', { path: node.path });
    } else {
      dispatch('select', { path: node.path, name: node.name });
    }
  }

  function getFileIcon(n: TreeNode): string {
    if (n.is_dir) {
      return n.expanded ? '▼' : '▶';
    }
    const ext = n.name.split('.').pop()?.toLowerCase() || '';
    switch (ext) {
      case 'c': return '◇';
      case 'h': return '◆';
      case 'cpp':
      case 'cc':
      case 'cxx': return '◇';
      case 'hpp':
      case 'hxx': return '◆';
      case 'rs': return '⬡';
      case 'py': return '◎';
      case 'js':
      case 'ts': return '◉';
      case 'json': return '{}';
      case 'toml':
      case 'yaml':
      case 'yml': return '≡';
      case 'md': return '¶';
      case 'txt': return '≡';
      case 'sh': return '$';
      default: return '○';
    }
  }

  function forwardToggle(e: CustomEvent<{ path: string }>) {
    dispatch('toggle', e.detail);
  }

  function forwardSelect(e: CustomEvent<{ path: string; name: string }>) {
    dispatch('select', e.detail);
  }
</script>

<div class="tree-node" style="padding-left: {depth * 12}px">
  <button 
    class="node-row" 
    class:directory={node.is_dir}
    on:click={handleClick}
  >
    <span class="node-icon">{getFileIcon(node)}</span>
    <span class="node-name">{node.name}</span>
  </button>
</div>

{#if node.is_dir && node.expanded && node.children}
  {#each node.children as child (child.path)}
    <svelte:self 
      node={child} 
      depth={depth + 1} 
      on:toggle={forwardToggle}
      on:select={forwardSelect}
    />
  {/each}
{/if}

<style>
  .tree-node {
    /* padding is set via inline style */
  }

  .node-row {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 2px 4px;
    border-radius: 4px;
    width: 100%;
    text-align: left;
    font-size: 13px;
    color: var(--color-text-secondary);
    transition: background 0.1s;
  }

  .node-row:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .node-row.directory {
    color: var(--color-text-primary);
  }

  .node-icon {
    font-size: 10px;
    width: 14px;
    text-align: center;
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .node-row.directory .node-icon {
    color: var(--color-accent);
  }

  .node-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
