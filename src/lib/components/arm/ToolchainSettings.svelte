<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { armToolchainStore, DEFAULT_MCU_CONFIGS } from '$lib/stores/armToolchain';
  import type { ArmToolchainSuite, ArmMcuConfig } from '$lib/stores/armToolchain';

  // Subscribe to stores - destructure to get individual store references
  const { toolchains, selectedToolchain, mcuConfig, loading, error } = armToolchainStore;
  
  // These are now Svelte stores that can be accessed with $ prefix in template
  // e.g., $toolchains, $selectedToolchain, $mcuConfig, $loading, $error

  // Local state
  let customPath = '';
  let validationResult: { valid: boolean; message: string; details?: string } | null = null;
  let validating = false;
  let settingsScope: 'project' | 'global' = 'project';
  let showCustomPathInput = false;

  // MCU preset options
  const cpuPresets = ['cortex-m0', 'cortex-m3', 'cortex-m4', 'cortex-m7'];
  const fpuTypes = ['none', 'fpv4-sp-d16', 'fpv5-d16', 'fpv5-sp-d16'];
  const floatAbiOptions: Array<'soft' | 'softfp' | 'hard'> = ['soft', 'softfp', 'hard'];

  // New define/path input
  let newDefine = '';
  let newIncludePath = '';

  // Detect toolchains on mount
  import { onMount } from 'svelte';
  onMount(() => {
    armToolchainStore.detectToolchains();
  });

  function handleRefresh() {
    armToolchainStore.detectToolchains();
  }

  function handleToolchainSelect(toolchain: ArmToolchainSuite) {
    armToolchainStore.selectToolchain(toolchain.gcc);
  }

  function handleAddCustomPath() {
    showCustomPathInput = !showCustomPathInput;
    if (!showCustomPathInput) {
      // Reset validation when closing
      customPath = '';
      validationResult = null;
      validating = false;
    }
  }

  async function handleValidateCustomPath() {
    // Clear previous validation result
    validationResult = null;
    
    // Basic client-side validation
    if (!customPath.trim()) {
      validationResult = { 
        valid: false, 
        message: 'Path cannot be empty' 
      };
      return;
    }

    // Check if path looks like it could be a toolchain
    const pathLower = customPath.toLowerCase();
    if (!pathLower.includes('arm-none-eabi') && !pathLower.includes('gcc')) {
      validationResult = { 
        valid: false, 
        message: 'Path should point to an ARM toolchain (e.g., arm-none-eabi-gcc)',
        details: 'Expected path to contain "arm-none-eabi" or "gcc"'
      };
      return;
    }

    // Perform backend validation
    validating = true;
    try {
      const result = await invoke<ArmToolchainSuite>('validate_toolchain_path', {
        path: customPath.trim()
      });
      
      // Validation successful
      if (result.completeness === 'Complete') {
        validationResult = { 
          valid: true, 
          message: `✓ Valid ARM GCC ${result.version} toolchain found`,
          details: `Source: ${result.source} - All required tools detected`
        };
      } else {
        validationResult = { 
          valid: false, 
          message: `⚠ Incomplete toolchain (ARM GCC ${result.version})`,
          details: `Missing tools: ${result.missing.join(', ')}`
        };
      }
    } catch (error) {
      // Validation failed
      const errorMsg = String(error);
      
      if (errorMsg.includes('does not exist')) {
        validationResult = { 
          valid: false, 
          message: '✗ Path does not exist',
          details: 'Please check the path and try again'
        };
      } else if (errorMsg.includes('No valid ARM toolchain')) {
        validationResult = { 
          valid: false, 
          message: '✗ No valid ARM toolchain found at this path',
          details: 'The path exists but does not contain a valid ARM GCC toolchain'
        };
      } else {
        validationResult = { 
          valid: false, 
          message: '✗ Validation failed',
          details: errorMsg
        };
      }
    } finally {
      validating = false;
    }
  }

  // Debounce validation to avoid excessive calls
  let validationTimeout: ReturnType<typeof setTimeout> | null = null;
  function handleCustomPathInput() {
    if (validationTimeout) {
      clearTimeout(validationTimeout);
    }
    
    // Clear validation result immediately on input
    validationResult = null;
    
    // Debounce validation by 500ms
    validationTimeout = setTimeout(() => {
      if (customPath.trim()) {
        handleValidateCustomPath();
      }
    }, 500);
  }

  function handleCpuPresetChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const preset = target.value as keyof typeof DEFAULT_MCU_CONFIGS;
    
    if (DEFAULT_MCU_CONFIGS[preset]) {
      // setMcuPreset will auto-populate FPU and float ABI based on the preset
      armToolchainStore.setMcuPreset(preset);
    }
  }

  function handleFpuChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const fpu = target.value === 'none' ? null : target.value;
    armToolchainStore.updateMcuConfig({ fpu });
  }

  function handleFloatAbiChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    const float_abi = target.value as 'soft' | 'softfp' | 'hard';
    armToolchainStore.updateMcuConfig({ float_abi });
  }

  function handleThumbChange(event: Event) {
    const target = event.target as HTMLInputElement;
    armToolchainStore.updateMcuConfig({ thumb: target.checked });
  }

  function handleAddDefine() {
    if (newDefine.trim()) {
      const currentDefines = $mcuConfig.defines;
      if (!currentDefines.includes(newDefine.trim())) {
        armToolchainStore.updateMcuConfig({ 
          defines: [...currentDefines, newDefine.trim()] 
        });
      }
      newDefine = '';
    }
  }

  function handleRemoveDefine(define: string) {
    const currentDefines = $mcuConfig.defines;
    armToolchainStore.updateMcuConfig({ 
      defines: currentDefines.filter(d => d !== define) 
    });
  }

  function handleAddIncludePath() {
    // Include paths will be handled in a future task
    // For now, just clear the input
    if (newIncludePath.trim()) {
      // TODO: Add to include paths list
      newIncludePath = '';
    }
  }

  function handleSave() {
    armToolchainStore.saveSettings(settingsScope)
      .then(() => {
        // Success - could show a toast notification here
        console.log(`Settings saved successfully to ${settingsScope}`);
      })
      .catch((error) => {
        // Error is already set in the store
        console.error('Failed to save settings:', error);
      });
  }

  function getInstallationInstructions(): string {
    const platform = navigator.platform.toLowerCase();
    
    if (platform.includes('mac')) {
      return `Install ARM GCC via Homebrew:
brew install arm-none-eabi-gcc

Or download from ARM Developer:
https://developer.arm.com/downloads/-/gnu-rm`;
    } else if (platform.includes('linux')) {
      return `Install ARM GCC via package manager:

Ubuntu/Debian:
sudo apt install gcc-arm-none-eabi

Fedora:
sudo dnf install arm-none-eabi-gcc-cs

Or download from ARM Developer:
https://developer.arm.com/downloads/-/gnu-rm`;
    } else {
      return `Download ARM GCC from ARM Developer:
https://developer.arm.com/downloads/-/gnu-rm

Or install via STM32CubeIDE which includes the toolchain.`;
    }
  }
</script>

<div class="toolchain-settings">
  <!-- Header with Refresh button -->
  <div class="settings-header">
    <h2>Toolchain Settings</h2>
    <button 
      class="btn-refresh" 
      on:click={handleRefresh}
      disabled={$loading}
      aria-label="Refresh toolchains"
    >
      Refresh
    </button>
  </div>

  <!-- Error display -->
  {#if $error}
    <div class="error-banner" role="alert">
      <span class="error-icon">⚠️</span>
      <span class="error-message">{$error}</span>
    </div>
  {/if}

  <!-- Detected Toolchains -->
  <section class="section">
    <h3>Detected Toolchains</h3>
    
    {#if $toolchains.length === 0 && !$loading}
      <div class="empty-state">
        <p class="empty-message">No ARM toolchains detected on your system.</p>
        <pre class="installation-instructions">{getInstallationInstructions()}</pre>
      </div>
    {:else}
      <div class="toolchain-list">
        {#each $toolchains as toolchain}
          <button
            class="toolchain-card"
            class:selected={$selectedToolchain?.gcc === toolchain.gcc}
            on:click={() => handleToolchainSelect(toolchain)}
          >
            <div class="toolchain-header">
              <span class="toolchain-radio">
                {$selectedToolchain?.gcc === toolchain.gcc ? '●' : '○'}
              </span>
              <span class="toolchain-name">
                ARM GCC {toolchain.version}
              </span>
              <span class="toolchain-source">({toolchain.source})</span>
              <span 
                class="toolchain-status"
                class:complete={toolchain.completeness === 'Complete'}
                class:incomplete={toolchain.completeness === 'Incomplete'}
              >
                {toolchain.completeness}
              </span>
            </div>
            <div class="toolchain-path">{toolchain.gcc}</div>
            {#if toolchain.missing.length > 0}
              <div class="toolchain-missing">
                Missing: {toolchain.missing.join(', ')}
              </div>
            {/if}
          </button>
        {/each}
      </div>
    {/if}

    <!-- Add Custom Path -->
    <button 
      class="btn-add-custom" 
      on:click={handleAddCustomPath}
      aria-expanded={showCustomPathInput}
    >
      + Add Custom Path
    </button>

    {#if showCustomPathInput}
      <div class="custom-path-input">
        <div class="input-with-validation">
          <input
            type="text"
            bind:value={customPath}
            placeholder="/path/to/arm-none-eabi-gcc"
            on:input={handleCustomPathInput}
            disabled={validating}
            aria-label="Custom toolchain path"
            aria-describedby={validationResult ? 'validation-message' : undefined}
          />
          {#if validating}
            <span class="validation-spinner" aria-label="Validating...">⏳</span>
          {/if}
        </div>
        {#if validationResult}
          <div 
            id="validation-message"
            class="validation-result"
            class:valid={validationResult.valid}
            class:invalid={!validationResult.valid}
            role="alert"
          >
            <div class="validation-message">{validationResult.message}</div>
            {#if validationResult.details}
              <div class="validation-details">{validationResult.details}</div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  </section>

  <!-- Tool Paths for Selected Toolchain -->
  {#if $selectedToolchain}
    <section class="section">
      <h3>Tool Paths</h3>
      
      <div class="tool-paths-list">
        <div class="tool-path-item">
          <span class="tool-label">GCC (C Compiler):</span>
          <span class="tool-path">{$selectedToolchain.gcc}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">G++ (C++ Compiler):</span>
          <span class="tool-path">{$selectedToolchain.gxx}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">AS (Assembler):</span>
          <span class="tool-path">{$selectedToolchain.as_}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">LD (Linker):</span>
          <span class="tool-path">{$selectedToolchain.ld}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">OBJCOPY (Binary Converter):</span>
          <span class="tool-path">{$selectedToolchain.objcopy}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">OBJDUMP (Disassembler):</span>
          <span class="tool-path">{$selectedToolchain.objdump}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">SIZE (Size Utility):</span>
          <span class="tool-path">{$selectedToolchain.size}</span>
        </div>
        
        <div class="tool-path-item">
          <span class="tool-label">GDB (Debugger):</span>
          <span class="tool-path">{$selectedToolchain.gdb}</span>
        </div>
      </div>
    </section>
  {/if}

  <!-- MCU Configuration -->
  <section class="section">
    <h3>MCU Configuration</h3>
    
    <div class="config-grid">
      <div class="config-field">
        <label for="cpu-preset">CPU Preset:</label>
        <select 
          id="cpu-preset"
          value={$mcuConfig.cpu}
          on:change={handleCpuPresetChange}
        >
          {#each cpuPresets as preset}
            <option value={preset}>{preset}</option>
          {/each}
        </select>
      </div>

      <div class="config-field">
        <label for="fpu-type">FPU Type:</label>
        <select 
          id="fpu-type"
          value={$mcuConfig.fpu || 'none'}
          on:change={handleFpuChange}
        >
          {#each fpuTypes as fpu}
            <option value={fpu}>{fpu}</option>
          {/each}
        </select>
      </div>

      <div class="config-field">
        <label for="float-abi">Float ABI:</label>
        <select 
          id="float-abi"
          value={$mcuConfig.float_abi}
          on:change={handleFloatAbiChange}
        >
          {#each floatAbiOptions as abi}
            <option value={abi}>{abi}</option>
          {/each}
        </select>
      </div>

      <div class="config-field checkbox-field">
        <label for="thumb-mode">
          <input
            id="thumb-mode"
            type="checkbox"
            checked={$mcuConfig.thumb}
            on:change={handleThumbChange}
          />
          Thumb Mode
        </label>
      </div>
    </div>
  </section>

  <!-- Preprocessor Defines -->
  <section class="section">
    <h3>Preprocessor Defines</h3>
    
    <div class="list-container">
      {#each $mcuConfig.defines as define}
        <div class="list-item">
          <span class="list-item-text">{define}</span>
          <button 
            class="btn-remove"
            on:click={() => handleRemoveDefine(define)}
            aria-label="Remove {define}"
          >
            ×
          </button>
        </div>
      {/each}
      
      <div class="add-item">
        <input
          type="text"
          bind:value={newDefine}
          placeholder="Add define (e.g., STM32H750xx)"
          on:keypress={(e) => e.key === 'Enter' && handleAddDefine()}
          aria-label="New preprocessor define"
        />
        <button 
          class="btn-add"
          on:click={handleAddDefine}
          disabled={!newDefine.trim()}
        >
          + Add Define
        </button>
      </div>
    </div>
  </section>

  <!-- Include Paths -->
  <section class="section">
    <h3>Include Paths</h3>
    
    <div class="list-container">
      <!-- Include paths will be implemented in task 4.7 -->
      <div class="placeholder-text">
        Include paths configuration will be available in the next update.
      </div>
      
      <div class="add-item">
        <input
          type="text"
          bind:value={newIncludePath}
          placeholder="Add include path"
          disabled
          aria-label="New include path"
        />
        <button 
          class="btn-add"
          on:click={handleAddIncludePath}
          disabled
        >
          + Add Path
        </button>
      </div>
    </div>
  </section>

  <!-- Linker Script -->
  <section class="section">
    <h3>Linker Script</h3>
    
    <div class="linker-script-field">
      <input
        type="text"
        placeholder="STM32H750VBTX_FLASH.ld"
        disabled
        aria-label="Linker script path"
      />
      <button class="btn-browse" disabled>Browse</button>
    </div>
    <p class="field-hint">Linker script selection will be available in task 5.8</p>
  </section>

  <!-- Settings Scope and Save -->
  <section class="section settings-footer">
    <div class="scope-selector">
      <span class="scope-label">Save to:</span>
      <label class="radio-label">
        <input
          type="radio"
          name="scope"
          value="project"
          bind:group={settingsScope}
        />
        Project
      </label>
      <label class="radio-label">
        <input
          type="radio"
          name="scope"
          value="global"
          bind:group={settingsScope}
        />
        Global
      </label>
      <span 
        class="scope-indicator"
        class:project-scope={settingsScope === 'project'}
        class:global-scope={settingsScope === 'global'}
        aria-label="Current settings scope"
      >
        {settingsScope === 'project' ? '📁 Project' : '🌐 Global'}
      </span>
    </div>
    
    <button 
      class="btn-save"
      on:click={handleSave}
      disabled={$loading}
    >
      Save
    </button>
  </section>
</div>

<style>
  .toolchain-settings {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
    padding: var(--space-md);
    height: 100%;
    overflow-y: auto;
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: var(--space-sm);
    border-bottom: 1px solid var(--color-border);
  }

  .settings-header h2 {
    margin: 0;
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .btn-refresh {
    padding: var(--space-xs) var(--space-md);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: background var(--transition-fast);
  }

  .btn-refresh:hover:not(:disabled) {
    background: var(--color-bg-hover);
  }

  .btn-refresh:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error-banner {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: rgba(244, 67, 54, 0.1);
    border: 1px solid rgba(244, 67, 54, 0.3);
    border-radius: var(--radius-sm);
    color: #f44336;
  }

  .error-icon {
    font-size: var(--font-size-lg);
  }

  .error-message {
    font-size: var(--font-size-sm);
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .section h3 {
    margin: 0;
    font-size: var(--font-size-base);
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .empty-state {
    padding: var(--space-lg);
    text-align: center;
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .empty-message {
    margin: 0 0 var(--space-md) 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
  }

  .installation-instructions {
    margin: 0;
    padding: var(--space-md);
    background: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-xs);
    text-align: left;
    white-space: pre-wrap;
    overflow-x: auto;
  }

  .toolchain-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .toolchain-card {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding: var(--space-md);
    background: var(--color-bg-tertiary);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: border-color var(--transition-fast), background var(--transition-fast), box-shadow var(--transition-fast);
    text-align: left;
    position: relative;
  }

  .toolchain-card:hover {
    background: var(--color-bg-hover);
    border-color: var(--color-accent-muted, rgba(33, 150, 243, 0.5));
  }

  .toolchain-card.selected {
    border-color: var(--color-accent);
    background: rgba(33, 150, 243, 0.08);
    box-shadow: 0 0 0 1px var(--color-accent);
  }

  .toolchain-card.selected::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 4px;
    background: var(--color-accent);
    border-radius: var(--radius-md) 0 0 var(--radius-md);
  }

  .toolchain-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .toolchain-radio {
    font-size: var(--font-size-lg);
    color: var(--color-text-muted);
    transition: color var(--transition-fast);
  }

  .toolchain-card.selected .toolchain-radio {
    color: var(--color-accent);
    font-weight: bold;
  }

  .toolchain-name {
    font-weight: 600;
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
  }

  .toolchain-source {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .toolchain-status {
    margin-left: auto;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 500;
  }

  .toolchain-status.complete {
    background: rgba(76, 175, 80, 0.2);
    color: #4caf50;
  }

  .toolchain-status.incomplete {
    background: rgba(255, 152, 0, 0.2);
    color: #ff9800;
  }

  .toolchain-path {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-family: monospace;
  }

  .toolchain-missing {
    font-size: var(--font-size-xs);
    color: #ff9800;
  }

  .btn-add-custom {
    padding: var(--space-sm) var(--space-md);
    background: transparent;
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-md);
    color: var(--color-text-secondary);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: border-color var(--transition-fast), color var(--transition-fast);
  }

  .btn-add-custom:hover {
    border-color: var(--color-accent);
    color: var(--color-accent);
  }

  .custom-path-input {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .input-with-validation {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input-with-validation input {
    flex: 1;
    padding: var(--space-sm);
    padding-right: var(--space-xl);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    font-family: monospace;
    transition: border-color var(--transition-fast);
  }

  .input-with-validation input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .input-with-validation input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .validation-spinner {
    position: absolute;
    right: var(--space-sm);
    font-size: var(--font-size-base);
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .validation-result {
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    border: 1px solid transparent;
  }

  .validation-result.valid {
    background: rgba(76, 175, 80, 0.1);
    border-color: rgba(76, 175, 80, 0.3);
    color: #4caf50;
  }

  .validation-result.invalid {
    background: rgba(244, 67, 54, 0.1);
    border-color: rgba(244, 67, 54, 0.3);
    color: #f44336;
  }

  .validation-message {
    font-weight: 600;
    margin-bottom: var(--space-xs);
  }

  .validation-details {
    font-size: var(--font-size-xs);
    opacity: 0.9;
    line-height: 1.4;
  }

  .config-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--space-md);
  }

  .config-field {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .config-field label {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
  }

  .config-field select {
    padding: var(--space-sm);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
  }

  .checkbox-field {
    flex-direction: row;
    align-items: center;
  }

  .checkbox-field label {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    cursor: pointer;
  }

  .checkbox-field input[type="checkbox"] {
    width: 16px;
    height: 16px;
    cursor: pointer;
  }

  .list-container {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .list-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
  }

  .list-item-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    font-family: monospace;
  }

  .btn-remove {
    padding: 0;
    width: 24px;
    height: 24px;
    background: transparent;
    border: none;
    color: var(--color-text-muted);
    font-size: var(--font-size-lg);
    cursor: pointer;
    transition: color var(--transition-fast);
  }

  .btn-remove:hover {
    color: #f44336;
  }

  .add-item {
    display: flex;
    gap: var(--space-sm);
  }

  .add-item input {
    flex: 1;
    padding: var(--space-sm);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
  }

  .add-item input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-add {
    padding: var(--space-sm) var(--space-md);
    background: var(--color-accent);
    border: none;
    border-radius: var(--radius-sm);
    color: white;
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: opacity var(--transition-fast);
    white-space: nowrap;
  }

  .btn-add:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-add:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .placeholder-text {
    padding: var(--space-md);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
    text-align: center;
  }

  .linker-script-field {
    display: flex;
    gap: var(--space-sm);
  }

  .linker-script-field input {
    flex: 1;
    padding: var(--space-sm);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
  }

  .linker-script-field input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-browse {
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: background var(--transition-fast);
    white-space: nowrap;
  }

  .btn-browse:hover:not(:disabled) {
    background: var(--color-bg-hover);
  }

  .btn-browse:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .field-hint {
    margin: 0;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    font-style: italic;
  }

  .settings-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  .scope-selector {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .scope-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-secondary);
    font-weight: 500;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    cursor: pointer;
  }

  .radio-label input[type="radio"] {
    cursor: pointer;
  }

  .scope-indicator {
    display: inline-flex;
    align-items: center;
    gap: var(--space-xs);
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-xs);
    font-weight: 600;
    margin-left: var(--space-sm);
    transition: background var(--transition-fast), color var(--transition-fast);
  }

  .scope-indicator.project-scope {
    background: rgba(33, 150, 243, 0.15);
    color: #2196f3;
    border: 1px solid rgba(33, 150, 243, 0.3);
  }

  .scope-indicator.global-scope {
    background: rgba(156, 39, 176, 0.15);
    color: #9c27b0;
    border: 1px solid rgba(156, 39, 176, 0.3);
  }

  .btn-save {
    padding: var(--space-sm) var(--space-xl);
    background: var(--color-accent);
    border: none;
    border-radius: var(--radius-sm);
    color: white;
    font-size: var(--font-size-sm);
    font-weight: 600;
    cursor: pointer;
    transition: opacity var(--transition-fast);
  }

  .btn-save:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .tool-paths-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .tool-path-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-md);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
  }

  .tool-label {
    font-size: var(--font-size-xs);
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .tool-path {
    font-size: var(--font-size-sm);
    color: var(--color-text-primary);
    font-family: monospace;
    word-break: break-all;
  }
</style>
