<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { EMPTY } from '$lib/strings';
  import { editorStore } from '$lib/stores/editor';
  import { highlightCode, detectLanguage, type HighlightedToken } from '$lib/utils/syntax';
  import { onMount, afterUpdate } from 'svelte';
  
  const { files, activeIndex, activeFile } = editorStore;
  
  $: highlightedContent = $activeFile ? highlightCode($activeFile.content, $activeFile.language) : [];
  
  // Debug: log tokens when they change
  $: if (highlightedContent.length > 0) {
    console.log('[Syntax] First 10 tokens:', highlightedContent.slice(0, 10).map(t => ({
      type: t.type,
      value: t.value.replace(/\n/g, '\\n').replace(/\t/g, '\\t').replace(/ /g, '·')
    })));
  }
  
  let editorElement: HTMLTextAreaElement;
  let highlightElement: HTMLElement;
  
  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    if ($activeFile) {
      editorStore.updateContent($activeFile.path, target.value);
    }
  }
  
  function handleScroll(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    if (highlightElement) {
      highlightElement.scrollTop = target.scrollTop;
      highlightElement.scrollLeft = target.scrollLeft;
    }
  }
  
  function handleKeyDown(e: KeyboardEvent) {
    // Handle Tab key
    if (e.key === 'Tab') {
      e.preventDefault();
      const target = e.target as HTMLTextAreaElement;
      const start = target.selectionStart;
      const end = target.selectionEnd;
      const value = target.value;
      
      // Insert tab character
      target.value = value.substring(0, start) + '\t' + value.substring(end);
      target.selectionStart = target.selectionEnd = start + 1;
      
      // Trigger input event to update content
      target.dispatchEvent(new Event('input', { bubbles: true }));
    }
  }
  
  function selectTab(index: number) {
    activeIndex.set(index);
  }
  
  function closeTab(e: MouseEvent, path: string) {
    e.stopPropagation();
    editorStore.closeFile(path);
  }
  
  function getLanguageLabel(lang: string): string {
    switch (lang) {
      case 'c': return 'C';
      case 'cpp': return 'C++';
      case 'h': return 'C Header';
      case 'hpp': return 'C++ Header';
      case 'python': return 'Python';
      case 'assembly': return 'ARM Assembly';
      default: return 'Text';
    }
  }
</script>

<div class="editor-area">
  {#if $files.length > 0}
    <div class="editor-tabs">
      {#each $files as file, i (file.path)}
        <button 
          class="tab" 
          class:active={i === $activeIndex}
          on:click={() => selectTab(i)}
          title={file.path}
        >
          <span class="tab-name">{file.name}</span>
          {#if file.modified}
            <span class="modified-dot">●</span>
          {/if}
          <button class="close-btn" on:click={(e) => closeTab(e, file.path)} title="Close">
            ×
          </button>
        </button>
      {/each}
    </div>
    <div class="editor-content">
      {#if $activeFile}
        <div class="file-info">
          <span class="file-path">{$activeFile.path}</span>
          <span class="file-lang">{getLanguageLabel($activeFile.language)}</span>
        </div>
        <div class="editor-container">
          <pre class="code-highlight" bind:this={highlightElement} aria-hidden="true"><code>{#each highlightedContent as token}<span class="token-{token.type}">{token.value}</span>{/each}</code></pre>
          <textarea
            bind:this={editorElement}
            class="code-editor"
            value={$activeFile.content}
            on:input={handleInput}
            on:scroll={handleScroll}
            on:keydown={handleKeyDown}
            spellcheck="false"
            autocomplete="off"
            autocorrect="off"
            autocapitalize="off"
          ></textarea>
        </div>
      {/if}
    </div>
  {:else}
    <div class="empty-state">
      <p>{EMPTY.noFiles}</p>
      <p class="hint">Open a file from the explorer to begin.</p>
    </div>
  {/if}
</div>

<style>
  .editor-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-bg-primary);
  }

  .editor-tabs {
    display: flex;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    height: 35px;
    align-items: flex-end;
    padding: 0 4px;
    gap: 2px;
    overflow-x: auto;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    font-size: 12px;
    color: var(--color-text-muted);
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    white-space: nowrap;
    max-width: 160px;
  }

  .tab:hover {
    background: var(--color-bg-hover);
  }

  .tab.active {
    color: var(--color-text-primary);
    background: var(--color-bg-primary);
    border-bottom-color: var(--color-bg-primary);
  }

  .tab-name {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .modified-dot {
    color: var(--color-accent);
    font-size: 10px;
  }

  .close-btn {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    font-size: 14px;
    color: var(--color-text-muted);
    opacity: 0;
    transition: opacity 0.1s;
  }

  .tab:hover .close-btn {
    opacity: 1;
  }

  .close-btn:hover {
    background: var(--color-bg-hover);
    color: var(--color-text-primary);
  }

  .editor-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .file-info {
    display: flex;
    justify-content: space-between;
    padding: 4px 12px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
    font-size: 11px;
  }

  .file-path {
    color: var(--color-text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .file-lang {
    color: var(--color-accent);
    font-weight: 500;
  }

  .editor-container {
    position: relative;
    flex: 1;
    overflow: hidden;
  }

  .code-highlight,
  .code-editor {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 12px;
    overflow: auto;
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.5;
    white-space: pre;
    tab-size: 4;
    word-wrap: normal;
  }

  .code-highlight {
    background: var(--color-bg-primary);
    color: var(--color-text-primary);
    pointer-events: none;
    z-index: 1;
  }

  .code-highlight code {
    display: block;
  }

  .code-editor {
    background: transparent;
    color: transparent;
    caret-color: var(--color-text-primary);
    border: none;
    outline: none;
    resize: none;
    z-index: 2;
  }

  .code-editor::selection {
    background: rgba(100, 150, 255, 0.3);
  }

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted);
  }

  .empty-state p {
    margin-bottom: 8px;
  }

  .empty-state .hint {
    font-size: 12px;
  }

  /* Scrollbar styling */
  .code-highlight::-webkit-scrollbar,
  .code-editor::-webkit-scrollbar {
    width: 8px;
    height: 8px;
  }

  .code-highlight::-webkit-scrollbar-track,
  .code-editor::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
  }

  .code-highlight::-webkit-scrollbar-thumb,
  .code-editor::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
  }

  .code-highlight::-webkit-scrollbar-thumb:hover,
  .code-editor::-webkit-scrollbar-thumb:hover {
    background: var(--color-border-focus);
  }

  /* Syntax highlighting */
  .token-keyword {
    color: #569cd6; /* Blue for keywords */
    font-weight: 500;
  }

  .token-string {
    color: #ce9178; /* Orange for strings */
  }

  .token-comment {
    color: #6a9955; /* Green for comments */
    font-style: italic;
  }

  .token-number {
    color: #b5cea8; /* Light green for numbers */
  }

  .token-operator {
    color: #d4d4d4; /* Light gray for operators */
  }

  .token-register {
    color: #4fc1ff; /* Light blue for ARM registers */
    font-weight: 500;
  }

  .token-directive {
    color: #c586c0; /* Purple for assembly directives */
    font-weight: 500;
  }

  .token-function {
    color: #dcdcaa; /* Yellow for functions and labels */
    font-weight: 500;
  }

  .token-type {
    color: #4ec9b0; /* Teal for types and constants */
    font-weight: 500;
  }

  .token-text {
    color: var(--color-text-primary);
  }
</style>
