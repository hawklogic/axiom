<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  export let title: string;
  export let collapsible = false;
  export let collapsed = false;

  function toggle() {
    if (collapsible) {
      collapsed = !collapsed;
    }
  }
</script>

<div class="panel" class:collapsed>
  <header class="panel-header no-select" on:click={toggle} role="button" tabindex="0" on:keypress={toggle}>
    <span class="panel-title">{title}</span>
    <div class="panel-header-actions" on:click={(e) => e.stopPropagation()} on:keypress={(e) => e.stopPropagation()} role="toolbar" tabindex="0">
      <slot name="header-actions" />
      {#if collapsible}
        <span class="collapse-icon">{collapsed ? '▶' : '▼'}</span>
      {/if}
    </div>
  </header>
  {#if !collapsed}
    <div class="panel-content">
      <slot />
    </div>
  {/if}
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    min-width: 200px;
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
  }

  .panel.collapsed .panel-content {
    display: none;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border-bottom: 1px solid var(--color-border);
    cursor: default;
  }

  .panel-header[role="button"] {
    cursor: pointer;
  }

  .panel-title {
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-secondary);
  }

  .panel-header-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .collapse-icon {
    font-size: 10px;
    color: var(--color-text-muted);
  }

  .panel-content {
    flex: 1;
    overflow: auto;
  }
</style>
