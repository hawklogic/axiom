<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { armToolchainStore } from '$lib/stores/armToolchain';
  import { complianceStore } from '$lib/stores/compliance';
  import ToolchainSettings from './ToolchainSettings.svelte';

  type TabId = 'settings' | 'build' | 'visualizer' | 'compliance' | 'memory';

  interface TabDefinition {
    id: TabId;
    label: string;
    icon: string;
    shortcut?: string;
  }

  const tabs: TabDefinition[] = [
    { id: 'settings', label: 'Settings', icon: '⚙️', shortcut: '1' },
    { id: 'build', label: 'Build', icon: '🔨', shortcut: '2' },
    { id: 'visualizer', label: 'Visualizer', icon: '👁️', shortcut: '3' },
    { id: 'compliance', label: 'Compliance', icon: '✓', shortcut: '4' },
    { id: 'memory', label: 'Memory', icon: '📊', shortcut: '5' },
  ];

  let activeTab: TabId = 'settings';

  // Subscribe to loading states from stores
  const { loading: armLoading } = armToolchainStore;
  const { loading: complianceLoading } = complianceStore;
  
  // Compute combined loading state
  $: isLoading = $armLoading || $complianceLoading;

  function handleKeydown(event: KeyboardEvent) {
    const currentIndex = tabs.findIndex(t => t.id === activeTab);
    
    switch (event.key) {
      case 'Tab':
        event.preventDefault();
        if (event.shiftKey) {
          // Previous tab
          const prevIndex = currentIndex > 0 ? currentIndex - 1 : tabs.length - 1;
          activeTab = tabs[prevIndex].id;
        } else {
          // Next tab
          const nextIndex = currentIndex < tabs.length - 1 ? currentIndex + 1 : 0;
          activeTab = tabs[nextIndex].id;
        }
        break;
      case 'ArrowLeft':
        event.preventDefault();
        const prevIndex = currentIndex > 0 ? currentIndex - 1 : tabs.length - 1;
        activeTab = tabs[prevIndex].id;
        break;
      case 'ArrowRight':
        event.preventDefault();
        const nextIndex = currentIndex < tabs.length - 1 ? currentIndex + 1 : 0;
        activeTab = tabs[nextIndex].id;
        break;
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
        const tabIndex = parseInt(event.key) - 1;
        if (tabIndex >= 0 && tabIndex < tabs.length) {
          activeTab = tabs[tabIndex].id;
        }
        break;
    }
  }

  function handleTabClick(tabId: TabId) {
    setActiveTab(tabId);
  }

  function handleTabKeypress(event: KeyboardEvent, tabId: TabId) {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      setActiveTab(tabId);
    }
  }

  function setActiveTab(tabId: TabId) {
    activeTab = tabId;
  }
</script>

<div 
  class="arm-toolchain-panel" 
  on:keydown={handleKeydown}
  role="tabpanel"
  tabindex="0"
>
  <!-- Tab Navigation -->
  <nav class="tab-bar" role="tablist" aria-label="ARM Toolchain tabs">
    {#each tabs as tab}
      <button
        class="tab"
        class:active={activeTab === tab.id}
        role="tab"
        aria-selected={activeTab === tab.id}
        aria-controls="panel-{tab.id}"
        id="tab-{tab.id}"
        tabindex={activeTab === tab.id ? 0 : -1}
        on:click={() => handleTabClick(tab.id)}
        on:keypress={(e) => handleTabKeypress(e, tab.id)}
      >
        <span class="tab-icon">{tab.icon}</span>
        <span class="tab-label">{tab.label}</span>
      </button>
    {/each}
  </nav>

  <!-- Tab Content -->
  <div class="tab-content" id="panel-{activeTab}" role="tabpanel" aria-labelledby="tab-{activeTab}">
    {#if activeTab === 'settings'}
      <ToolchainSettings />
    {:else if activeTab === 'build'}
      <div class="placeholder-content">
        <p>Build Panel</p>
        <p class="hint">Compile, link, and generate binaries</p>
      </div>
    {:else if activeTab === 'visualizer'}
      <div class="placeholder-content">
        <p>Compiler Visualizer</p>
        <p class="hint">View preprocessor, assembly, and disassembly output</p>
      </div>
    {:else if activeTab === 'compliance'}
      <div class="placeholder-content">
        <p>Compliance Panel</p>
        <p class="hint">Manage DO-178C, DO-330, and ARP4754A compliance</p>
      </div>
    {:else if activeTab === 'memory'}
      <div class="placeholder-content">
        <p>Memory Map Viewer</p>
        <p class="hint">Visualize memory layout and usage</p>
      </div>
    {/if}
  </div>

  <!-- Loading Overlay -->
  {#if isLoading}
    <div class="loading-overlay" aria-live="polite" aria-busy="true">
      <div class="loading-spinner"></div>
      <span class="loading-text">Loading...</span>
    </div>
  {/if}
</div>

<style>
  .arm-toolchain-panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--color-bg-secondary);
    position: relative;
  }

  .tab-bar {
    display: flex;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    padding: 0;
    gap: 0;
    overflow-x: auto;
    flex-shrink: 0;
  }

  /* Hide scrollbar but keep functionality */
  .tab-bar::-webkit-scrollbar {
    height: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-lg);
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: color var(--transition-fast), border-color var(--transition-fast), background var(--transition-fast);
    white-space: nowrap;
    position: relative;
  }

  .tab:hover:not(.active) {
    color: var(--color-text-secondary);
    background: var(--color-bg-hover);
  }

  .tab.active {
    color: var(--color-accent);
    border-bottom-color: var(--color-accent);
    background: transparent;
  }

  .tab:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
    z-index: 1;
  }

  .tab-icon {
    font-size: var(--font-size-lg);
    line-height: 1;
    opacity: 0.9;
  }

  .tab.active .tab-icon {
    opacity: 1;
  }

  .tab-label {
    font-weight: 500;
    letter-spacing: 0.01em;
  }

  .tab-content {
    flex: 1;
    overflow: auto;
    padding: var(--space-md);
    background: var(--color-bg-secondary);
  }

  .placeholder-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--color-text-muted);
    text-align: center;
    gap: var(--space-xs);
  }

  .placeholder-content p {
    margin: 0;
    font-size: var(--font-size-base);
    font-weight: 500;
  }

  .placeholder-content .hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    opacity: 0.8;
  }

  .loading-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(26, 26, 26, 0.85);
    backdrop-filter: blur(2px);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-md);
    z-index: 100;
  }

  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .loading-text {
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
    font-weight: 500;
    letter-spacing: 0.02em;
  }
</style>
