<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { STATUS } from '$lib/strings';
  
  // These would be reactive based on actual state
  let status = STATUS.ready;
  let encoding = 'UTF-8';
  let position = 'Ln 1, Col 1';
  let branch = 'dev';
  let toolchain = 'Clang 15.0.0';
</script>

<footer class="status-bar no-select">
  <div class="status-left">
    <span class="status-item branch">⎇ {branch}</span>
    <span class="status-item">{status}</span>
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
  }

  .status-item {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .branch {
    color: var(--color-accent);
  }
</style>
