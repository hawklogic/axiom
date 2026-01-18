<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { highlightCode, type HighlightedToken } from '$lib/utils/syntax';
  import { editorPanes, type EditorPane } from '$lib/stores/editorPanes';
  import { consoleStore } from '$lib/stores/console';
  import { EMPTY } from '$lib/strings';
  import { onMount } from 'svelte';
  
  export let pane: EditorPane;
  export let onDragStart: (paneId: string, filePath: string) => void;
  export let onDragEnd: () => void;
  export let onDrop: (paneId: string) => void;
  
  let editorElement: HTMLTextAreaElement;
  let highlightElement: HTMLElement;
  let isDragOver = false;
  
  onMount(() => {
    console.log('[EditorPane] Mounted, pane:', pane.id, 'files:', pane.files.length);
  });
  
  $: activeFile = pane.activeIndex >= 0 ? pane.files[pane.activeIndex] : null;
  $: highlightedContent = activeFile ? highlightCode(activeFile.content, activeFile.language) : [];
  
  // Undo/Redo state
  interface HistoryState {
    content: string;
    cursorPos: number;
  }
  
  let undoStack: HistoryState[] = [];
  let redoStack: HistoryState[] = [];
  let isUndoRedo = false;
  let originalContent = '';
  let currentFilePath = '';
  
  $: if (activeFile && activeFile.path !== currentFilePath) {
    currentFilePath = activeFile.path;
    originalContent = activeFile.content;
    undoStack = [{ content: activeFile.content, cursorPos: 0 }];
    redoStack = [];
  }
  
  function pushHistory(content: string, cursorPos: number) {
    if (undoStack.length > 0 && undoStack[undoStack.length - 1].content === content) {
      return;
    }
    undoStack.push({ content, cursorPos });
    if (undoStack.length > 100) {
      undoStack.shift();
    }
    redoStack = [];
  }
  
  function undo() {
    if (undoStack.length <= 1 || !activeFile) return;
    
    isUndoRedo = true;
    const current = undoStack.pop()!;
    redoStack.push(current);
    
    const previous = undoStack[undoStack.length - 1];
    editorPanes.updateContent(activeFile.path, previous.content);
    
    if (previous.content === originalContent) {
      editorPanes.markSaved(activeFile.path);
    }
    
    if (editorElement) {
      setTimeout(() => {
        editorElement.selectionStart = editorElement.selectionEnd = previous.cursorPos;
        isUndoRedo = false;
      }, 0);
    } else {
      isUndoRedo = false;
    }
  }
  
  function redo() {
    if (redoStack.length === 0 || !activeFile) return;
    
    isUndoRedo = true;
    const next = redoStack.pop()!;
    undoStack.push(next);
    
    editorPanes.updateContent(activeFile.path, next.content);
    
    if (next.content === originalContent) {
      editorPanes.markSaved(activeFile.path);
    }
    
    if (editorElement) {
      setTimeout(() => {
        editorElement.selectionStart = editorElement.selectionEnd = next.cursorPos;
        isUndoRedo = false;
      }, 0);
    } else {
      isUndoRedo = false;
    }
  }
  
  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    if (activeFile && !isUndoRedo) {
      pushHistory(target.value, target.selectionStart);
      editorPanes.updateContent(activeFile.path, target.value);
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
    if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
      e.preventDefault();
      undo();
      return;
    }
    
    if (((e.ctrlKey && e.key === 'y') || (e.metaKey && e.shiftKey && e.key === 'z'))) {
      e.preventDefault();
      redo();
      return;
    }
    
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      if (activeFile) {
        saveFile(activeFile.path, activeFile.content);
      }
      return;
    }
    
    if (e.key === 'Enter') {
      e.preventDefault();
      const target = e.target as HTMLTextAreaElement;
      const start = target.selectionStart;
      const value = target.value;
      
      let lineStart = start - 1;
      while (lineStart >= 0 && value[lineStart] !== '\n') lineStart--;
      lineStart++;
      
      let lineEnd = start;
      while (lineEnd < value.length && value[lineEnd] !== '\n') lineEnd++;
      const currentLine = value.substring(lineStart, lineEnd);
      
      let indent = '';
      for (let i = 0; i < currentLine.length; i++) {
        if (currentLine[i] === ' ' || currentLine[i] === '\t') {
          indent += currentLine[i];
        } else {
          break;
        }
      }
      
      const trimmedLine = currentLine.trim();
      const needsExtraIndent = trimmedLine.endsWith('{') || 
                                trimmedLine.endsWith('[') || 
                                trimmedLine.endsWith('(') ||
                                trimmedLine.endsWith(':');
      
      let insertion = '\n' + indent;
      if (needsExtraIndent) {
        insertion += '\t';
      }
      
      target.value = value.substring(0, start) + insertion + value.substring(start);
      target.selectionStart = target.selectionEnd = start + insertion.length;
      target.dispatchEvent(new Event('input', { bubbles: true }));
      return;
    }
    
    if (e.key === 'Tab') {
      e.preventDefault();
      const target = e.target as HTMLTextAreaElement;
      const start = target.selectionStart;
      const end = target.selectionEnd;
      const value = target.value;
      
      target.value = value.substring(0, start) + '\t' + value.substring(end);
      target.selectionStart = target.selectionEnd = start + 1;
      target.dispatchEvent(new Event('input', { bubbles: true }));
    }
  }
  
  async function saveFile(path: string, content: string) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('write_file', { path, contents: content });
      editorPanes.markSaved(path);
      originalContent = content;
      console.log('[Editor] File saved:', path);
    } catch (err) {
      console.error('[Editor] Failed to save file:', err);
    }
  }
  
  function selectTab(index: number) {
    editorPanes.setActiveFile(pane.id, index);
  }
  
  function handleTabKeyDown(e: KeyboardEvent, index: number) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      selectTab(index);
    }
  }
  
  function closeTab(e: MouseEvent, path: string) {
    e.stopPropagation();
    editorPanes.closeFile(pane.id, path);
  }
  
  function handleTabDragStart(e: DragEvent, filePath: string) {
    if (!e.dataTransfer) return;
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', filePath);
    console.log('[DragDrop] Drag started:', filePath, 'from pane:', pane.id);
    onDragStart(pane.id, filePath);
  }
  
  function handleTabDragEnd(e: DragEvent) {
    console.log('[DragDrop] Drag ended');
    onDragEnd();
  }
  
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    if (e.dataTransfer) {
      e.dataTransfer.dropEffect = 'move';
    }
    if (!isDragOver) {
      console.log('[DragDrop] Drag over pane:', pane.id);
      consoleStore.log('debug', 'editor', `Drag over ${pane.id}`);
      isDragOver = true;
    }
  }
  
  function handleDragLeave(e: DragEvent) {
    e.stopPropagation();
    // Only set isDragOver to false if we're leaving the pane entirely
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    if (e.clientX < rect.left || e.clientX >= rect.right ||
        e.clientY < rect.top || e.clientY >= rect.bottom) {
      console.log('[DragDrop] Drag leave pane:', pane.id);
      consoleStore.log('debug', 'editor', `Drag leave ${pane.id}`);
      isDragOver = false;
    }
  }
  
  function handleDrop(e: DragEvent) {
    e.preventDefault();
    e.stopPropagation();
    console.log('[DragDrop] Drop on pane:', pane.id);
    consoleStore.log('info', 'editor', `Drop on ${pane.id}`);
    isDragOver = false;
    onDrop(pane.id);
  }
  
  function getLanguageLabel(lang: string): string {
    switch (lang) {
      case 'c': return 'C';
      case 'cpp': return 'C++';
      case 'h': return 'C Header';
      case 'hpp': return 'C++ Header';
      case 'python': return 'Python';
      case 'assembly': return 'ARM Assembly';
      case 'makefile': return 'Makefile';
      case 'linker': return 'Linker Script';
      default: return 'Text';
    }
  }
</script>

<div class="editor-pane" 
     class:drag-over={isDragOver} 
     on:dragover={handleDragOver} 
     on:dragleave={handleDragLeave} 
     on:drop={handleDrop}
     role="region"
     aria-label="Editor pane">
  {#if isDragOver}
    <div class="drop-indicator">
      <div class="drop-message">Drop file here</div>
    </div>
  {/if}
  {#if pane.files.length > 0}
    <div class="editor-tabs">
      {#each pane.files as file, i (file.path)}
        <div 
          class="tab" 
          class:active={i === pane.activeIndex}
          draggable="true"
          on:dragstart={(e) => handleTabDragStart(e, file.path)}
          on:dragend={handleTabDragEnd}
          on:click={() => selectTab(i)}
          on:keydown={(e) => handleTabKeyDown(e, i)}
          title={file.path}
          role="button"
          tabindex="0"
        >
          <span class="tab-name">{file.name}</span>
          {#if file.modified}
            <span class="modified-dot">●</span>
          {/if}
          <button class="close-btn" on:click={(e) => closeTab(e, file.path)} title="Close">
            ×
          </button>
        </div>
      {/each}
    </div>
    <div class="editor-content">
      {#if activeFile}
        <div class="file-info">
          <span class="file-path">{activeFile.path}</span>
          <span class="file-lang">{getLanguageLabel(activeFile.language)}</span>
        </div>
        <div class="editor-container">
          <pre class="code-highlight" bind:this={highlightElement} aria-hidden="true"><code>{#each highlightedContent as token}<span class="token-{token.type}">{token.value}</span>{/each}</code></pre>
          <textarea
            bind:this={editorElement}
            class="code-editor"
            value={activeFile.content}
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
      <p class="hint">Open a file or drag a tab here</p>
    </div>
  {/if}
</div>

<style>
  .editor-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-bg-primary);
    position: relative;
  }
  
  .editor-pane.drag-over {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
  }
  
  .drop-indicator {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(88, 166, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    pointer-events: none;
  }
  
  .drop-message {
    padding: 16px 32px;
    background: var(--color-accent);
    color: var(--color-bg-primary);
    border-radius: 8px;
    font-weight: 600;
    font-size: 14px;
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
    cursor: grab;
    white-space: nowrap;
    max-width: 160px;
  }
  
  .tab:active {
    cursor: grabbing;
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
    color: #569cd6;
    font-weight: 500;
  }

  .token-string {
    color: #ce9178;
  }

  .token-comment {
    color: #6a9955;
    font-style: italic;
  }

  .token-number {
    color: #b5cea8;
  }

  .token-operator {
    color: #d4d4d4;
  }

  .token-register {
    color: #4fc1ff;
    font-weight: 500;
  }

  .token-directive {
    color: #c586c0;
    font-weight: 500;
  }

  .token-function {
    color: #dcdcaa;
    font-weight: 500;
  }

  .token-type {
    color: #4ec9b0;
    font-weight: 500;
  }

  .token-text {
    color: var(--color-text-primary);
  }
</style>
