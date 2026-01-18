<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  export let activePanel = 'files';

  const items = [
    { id: 'files', icon: '📁', label: 'Files' },
    { id: 'git', icon: '⎇', label: 'Source Control' },
    { id: 'ast', icon: '🌳', label: 'AST' },
    { id: 'debug', icon: '🔧', label: 'Debug' },
    { id: 'settings', icon: '⚙', label: 'Settings' },
    { id: 'about', icon: 'ℹ', label: 'About' },
  ];
</script>

<aside class="sidebar no-select">
  <div class="logo-section">
    <img src="/assets/axiom-icon.svg" alt="Axiom" class="logo-icon" />
  </div>
  
  <nav class="nav">
    {#each items as item}
      <button
        class="nav-item"
        class:active={activePanel === item.id}
        on:click={() => activePanel = item.id}
        title={item.label}
      >
        <span class="icon">{item.icon}</span>
      </button>
    {/each}
  </nav>
</aside>

<style>
  .sidebar {
    width: 48px;
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
  }
  
  .logo-section {
    padding: var(--space-sm) 0;
    display: flex;
    justify-content: center;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
  }
  
  .logo-icon {
    width: 36px;
    height: 36px;
    transition: all 0.2s ease;
    cursor: default;
    filter: drop-shadow(0 0 4px rgba(0, 212, 255, 0.3));
  }
  
  .logo-icon:hover {
    transform: scale(1.05);
    filter: drop-shadow(0 0 8px rgba(0, 212, 255, 0.5));
  }

  .nav {
    display: flex;
    flex-direction: column;
    padding: var(--space-xs);
    gap: var(--space-xs);
    flex: 1;
  }

  .nav-item {
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
    transition: background var(--transition-fast);
  }

  .nav-item:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .nav-item.active {
    background: var(--color-bg-active);
    color: var(--color-accent);
  }

  .icon {
    font-size: 18px;
  }
</style>
