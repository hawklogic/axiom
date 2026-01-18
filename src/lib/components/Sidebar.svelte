<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  export let activePanel = 'files';

  let hoveredItem: string | null = null;

  const items = [
    { 
      id: 'files', 
      label: 'Files',
      description: 'File Explorer',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/></svg>'
    },
    { 
      id: 'git', 
      label: 'Source Control',
      description: 'Git Operations',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M13 6h3a2 2 0 012 2v7"/><line x1="6" y1="9" x2="6" y2="21"/></svg>'
    },
    { 
      id: 'ast', 
      label: 'AST',
      description: 'Syntax Tree',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="5" r="2"/><circle cx="6" cy="12" r="2"/><circle cx="18" cy="12" r="2"/><circle cx="6" cy="19" r="2"/><circle cx="18" cy="19" r="2"/><line x1="12" y1="7" x2="6" y2="10"/><line x1="12" y1="7" x2="18" y2="10"/><line x1="6" y1="14" x2="6" y2="17"/><line x1="18" y1="14" x2="18" y2="17"/></svg>'
    },
    { 
      id: 'debug', 
      label: 'Debug',
      description: 'Debugger',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2v4m0 12v4m8-10h-4M8 12H4m15.07-7.07l-2.83 2.83M9.76 14.24l-2.83 2.83m12.14 0l-2.83-2.83M9.76 9.76L6.93 6.93"/><circle cx="12" cy="12" r="3"/></svg>'
    },
    { 
      id: 'settings', 
      label: 'Settings',
      description: 'Preferences',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 1v6m0 6v10M5.64 5.64l4.24 4.24m4.24 4.24l4.24 4.24M1 12h6m6 0h10M5.64 18.36l4.24-4.24m4.24-4.24l4.24-4.24"/></svg>'
    },
    { 
      id: 'about', 
      label: 'About',
      description: 'Information',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>'
    },
  ];
</script>

<aside class="sidebar no-select">
  <div class="logo-section">
    <a href="https://hawklogicsystems.com/" target="_blank" rel="noopener noreferrer" class="logo-link" title="HawkLogic Systems">
      <img src="/assets/axiom-icon.svg" alt="Axiom" class="logo-icon" />
    </a>
  </div>
  
  <nav class="nav">
    {#each items as item}
      <button
        class="nav-item"
        class:active={activePanel === item.id}
        on:click={() => activePanel = item.id}
        on:mouseenter={() => hoveredItem = item.id}
        on:mouseleave={() => hoveredItem = null}
        aria-label={item.label}
      >
        <span class="icon">
          {@html item.svg}
        </span>
        {#if hoveredItem === item.id}
          <div class="tooltip">
            <div class="tooltip-label">{item.label}</div>
            <div class="tooltip-description">{item.description}</div>
          </div>
        {/if}
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
  
  .logo-link {
    display: flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
    border-radius: 4px;
    padding: 2px;
    transition: background 0.2s ease;
  }
  
  .logo-link:hover {
    background: rgba(0, 212, 255, 0.05);
  }
  
  .logo-icon {
    width: 36px;
    height: 36px;
    transition: filter 0.3s ease;
    cursor: pointer;
    filter: drop-shadow(0 0 2px rgba(0, 212, 255, 0.2));
  }
  
  .logo-link:hover .logo-icon {
    filter: drop-shadow(0 0 6px rgba(0, 212, 255, 0.6));
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
    transition: all var(--transition-fast);
    position: relative;
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
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon :global(svg) {
    width: 100%;
    height: 100%;
  }

  .tooltip {
    position: absolute;
    left: 52px;
    top: 50%;
    transform: translateY(-50%);
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-xs) var(--space-sm);
    white-space: nowrap;
    pointer-events: none;
    z-index: 1000;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    animation: tooltipFadeIn 0.15s ease-out;
  }

  @keyframes tooltipFadeIn {
    from {
      opacity: 0;
      transform: translateY(-50%) translateX(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(-50%) translateX(0);
    }
  }

  .tooltip-label {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--color-text-primary);
    margin-bottom: 2px;
  }

  .tooltip-description {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
