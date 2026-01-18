<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';
  import { consoleStore, type LogLevel } from '$lib/stores/console';

  let consoleContainer: HTMLDivElement;
  let autoScroll = true;

  const { filteredEntries, filter } = consoleStore;

  onMount(() => {
    consoleStore.init();
  });

  afterUpdate(() => {
    if (autoScroll && consoleContainer) {
      consoleContainer.scrollTop = consoleContainer.scrollHeight;
    }
  });

  function handleScroll() {
    if (consoleContainer) {
      const { scrollTop, scrollHeight, clientHeight } = consoleContainer;
      // Disable auto-scroll if user scrolls up
      autoScroll = scrollHeight - scrollTop - clientHeight < 50;
    }
  }

  function formatTime(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleTimeString('en-US', { 
      hour12: false, 
      hour: '2-digit', 
      minute: '2-digit', 
      second: '2-digit',
      fractionalSecondDigits: 3
    }).replace(',', '.');
  }

  function getLevelClass(level: LogLevel): string {
    switch (level) {
      case 'debug': return 'level-debug';
      case 'info': return 'level-info';
      case 'warn': return 'level-warn';
      case 'error': return 'level-error';
      default: return '';
    }
  }

  function handleFilterChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    consoleStore.setFilter(target.value as LogLevel | 'all');
  }
</script>

<div class="mini-console">
  <div class="console-header">
    <span class="console-title">Backend Console</span>
    <div class="console-controls">
      <select 
        class="filter-select" 
        value={$filter} 
        on:change={handleFilterChange}
      >
        <option value="all">All</option>
        <option value="debug">Debug</option>
        <option value="info">Info</option>
        <option value="warn">Warn</option>
        <option value="error">Error</option>
      </select>
      <button 
        class="clear-btn" 
        on:click={() => consoleStore.clear()}
        title="Clear console"
      >
        Clear
      </button>
    </div>
  </div>
  
  <div 
    class="console-output" 
    bind:this={consoleContainer}
    on:scroll={handleScroll}
  >
    {#each $filteredEntries as entry (entry.timestamp + entry.message)}
      <div class="log-entry {getLevelClass(entry.level)}">
        <span class="log-time">{formatTime(entry.timestamp)}</span>
        <span class="log-level">[{entry.level.toUpperCase()}]</span>
        <span class="log-source">{entry.source}:</span>
        <span class="log-message">{entry.message}</span>
      </div>
    {/each}
    {#if $filteredEntries.length === 0}
      <div class="empty-state">No log entries</div>
    {/if}
  </div>
</div>

<style>
  .mini-console {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--color-bg-primary);
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .console-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 8px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    user-select: none;
  }

  .console-title {
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--color-text-muted);
  }

  .console-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .filter-select {
    font-family: var(--font-mono);
    font-size: 10px;
    padding: 2px 4px;
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 2px;
    color: var(--color-text-secondary);
    cursor: pointer;
  }

  .filter-select:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .clear-btn {
    font-family: var(--font-mono);
    font-size: 10px;
    padding: 2px 8px;
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: 2px;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: background 0.1s;
  }

  .clear-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .console-output {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 4px;
  }

  .log-entry {
    display: flex;
    gap: 6px;
    padding: 2px 4px;
    line-height: 1.4;
    border-radius: 2px;
    word-break: break-word;
  }

  .log-entry:hover {
    background: var(--color-bg-hover);
  }

  .log-time {
    color: var(--color-text-muted);
    flex-shrink: 0;
  }

  .log-level {
    flex-shrink: 0;
    font-weight: 600;
    min-width: 50px;
  }

  .log-source {
    color: var(--color-accent);
    flex-shrink: 0;
  }

  .log-message {
    color: var(--color-text-secondary);
    flex: 1;
  }

  /* Level-specific colors */
  .level-debug .log-level {
    color: var(--color-text-muted);
  }

  .level-info .log-level {
    color: var(--color-accent);
  }

  .level-warn .log-level {
    color: var(--color-warning);
  }

  .level-warn .log-message {
    color: var(--color-warning);
  }

  .level-error .log-level {
    color: var(--color-error);
  }

  .level-error .log-message {
    color: var(--color-error);
  }

  .empty-state {
    color: var(--color-text-muted);
    text-align: center;
    padding: 16px;
    font-style: italic;
  }

  /* Scrollbar styling */
  .console-output::-webkit-scrollbar {
    width: 6px;
  }

  .console-output::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
  }

  .console-output::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 3px;
  }

  .console-output::-webkit-scrollbar-thumb:hover {
    background: var(--color-border-focus);
  }
</style>
