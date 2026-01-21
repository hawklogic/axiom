<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { armToolchainStore } from '$lib/stores/armToolchain';

  export let activePanel = 'files';

  let hoveredItem: string | null = null;

  // Subscribe to ARM toolchain loading state for build progress indicator
  const { loading: armLoading } = armToolchainStore;

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
      id: 'arm', 
      label: 'ARM Toolchain',
      description: 'Embedded Development',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="6" width="20" height="12" rx="2"/><circle cx="7" cy="12" r="2"/><circle cx="17" cy="12" r="2"/><path d="M7 10V8M17 10V8M7 14v2M17 14v2"/><line x1="9" y1="12" x2="15" y2="12"/></svg>'
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
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="8" y="6" width="8" height="12" rx="4"/><path d="M8 10h8M8 14h8"/><path d="M12 6V4M9 4.5L7 3M15 4.5l2-1.5"/><path d="M7 9l-2-1M17 9l2-1M7 15l-2 1M17 15l2 1"/></svg>'
    },
    { 
      id: 'settings', 
      label: 'Settings',
      description: 'Preferences',
      svg: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M12 2.69l1.1 2.86 3.02.44-2.19 2.13.52 3.01L12 9.84l-2.45 1.29.52-3.01-2.19-2.13 3.02-.44L12 2.69z"/><path d="M19.14 12.94c.04-.31.06-.63.06-.94 0-.31-.02-.63-.06-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94L14.4 2.81c-.04-.24-.24-.42-.49-.42h-3.84c-.24 0-.45.18-.49.42l-.37 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.69 8.87c-.11.2-.05.47.12.61l2.03 1.58c-.04.31-.07.63-.07.94s.02.63.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.37 2.54c.04.24.24.42.49.42h3.84c.24 0 .45-.18.49-.42l.37-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58z"/></svg>'
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
        class:building={item.id === 'arm' && $armLoading}
        on:click={() => activePanel = item.id}
        on:mouseenter={() => hoveredItem = item.id}
        on:mouseleave={() => hoveredItem = null}
        aria-label={item.label}
      >
        <span class="icon">
          {@html item.svg}
        </span>
        {#if item.id === 'arm' && $armLoading}
          <span class="build-progress-indicator" aria-label="Build in progress">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10" opacity="0.25"/>
              <path d="M12 2a10 10 0 0 1 10 10" stroke-linecap="round"/>
            </svg>
          </span>
        {/if}
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

  .nav-item.building {
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

  .build-progress-indicator {
    position: absolute;
    top: 2px;
    right: 2px;
    width: 12px;
    height: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }

  .build-progress-indicator svg {
    width: 100%;
    height: 100%;
    animation: spin 1s linear infinite;
    color: var(--color-accent);
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
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
