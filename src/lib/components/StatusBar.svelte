<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { APP } from '$lib/strings';
  import { editorPanes } from '$lib/stores/editorPanes';
  import { ideStatus } from '$lib/stores/status';
  
  let branch = 'dev';
  let toolchain = 'Clang 15.0.0';
  
  const version = '0.1.0';
  
  // Get cursor position from active file in any pane
  $: activeFile = $editorPanes.panes
    .flatMap(pane => pane.activeIndex >= 0 ? [pane.files[pane.activeIndex]] : [])
    [0];
  
  $: position = activeFile?.cursor 
    ? `Ln ${activeFile.cursor.line}, Col ${activeFile.cursor.column}`
    : 'Ln 1, Col 1';
  
  $: encoding = activeFile?.encoding || 'UTF-8';
</script>

<footer class="status-bar no-select">
  <div class="status-left">
    <span class="status-item branch">⎇ {branch}</span>
    <span class="status-item" class:status-error={$ideStatus.type === 'error'} class:status-loading={$ideStatus.type === 'loading' || $ideStatus.type === 'saving' || $ideStatus.type === 'building'}>
      {$ideStatus.message}
    </span>
  </div>
  <div class="status-center">
    <span class="branding">
      {APP.name} v{version} · 
      <a href="https://hawklogicsystems.com/" target="_blank" rel="noopener noreferrer" class="company-link">
        {APP.steward}
      </a>
    </span>
  </div>
  <div class="status-right">
    <span class="status-item">{toolchain}</span>
    <span class="status-item">{encoding}</span>
    <span class="status-item">{position}</span>
  </div>
</footer>

<style>
  .status-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 22px;
    padding: 0 var(--space-md);
    background: var(--color-bg-tertiary);
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .status-left, .status-right {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    flex: 1;
  }

  .status-right {
    justify-content: flex-end;
  }

  .status-center {
    flex-shrink: 0;
  }

  .branding {
    font-size: 10px;
    color: var(--color-text-muted);
    opacity: 0.7;
    letter-spacing: 0.3px;
  }
  
  .company-link {
    color: var(--color-text-muted);
    text-decoration: none;
    transition: color 0.2s;
  }
  
  .company-link:hover {
    color: var(--color-accent);
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .branch {
    color: var(--color-accent);
  }
  
  .status-error {
    color: #f85149;
  }
  
  .status-loading {
    color: #f0ad4e;
  }
</style>
