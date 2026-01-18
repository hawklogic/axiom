<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { highlightCode, type HighlightedToken } from '$lib/utils/syntax';
  import { editorPanes, type EditorPane } from '$lib/stores/editorPanes';
  import { consoleStore } from '$lib/stores/console';
  import { ideStatus } from '$lib/stores/status';
  import { settingsStore } from '$lib/stores/settings';
  import { EMPTY } from '$lib/strings';
  import { onMount } from 'svelte';
  
  export let pane: EditorPane;
  export let onDragStart: (paneId: string, filePath: string) => void;
  export let onDragEnd: () => void;
  export let onDrop: (targetPaneId: string) => void;
  
  let editorElement: HTMLTextAreaElement;
  let highlightElement: HTMLElement;
  let lineNumbersElement: HTMLElement;
  let showLineNumbers = true; // Default to true
  
  // Load line numbers preference from settings
  $: if ($settingsStore) {
    showLineNumbers = $settingsStore.editor.line_numbers;
  }
  
  function toggleLineNumbers() {
    showLineNumbers = !showLineNumbers;
    if ($settingsStore) {
      $settingsStore.editor.line_numbers = showLineNumbers;
      settingsStore.save($settingsStore);
    }
  }
  
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
      updateCursorPosition(target);
    }
  }
  
  function updateCursorPosition(target: HTMLTextAreaElement) {
    if (!activeFile) return;
    
    const text = target.value.substring(0, target.selectionStart);
    const lines = text.split('\n');
    const line = lines.length;
    const column = lines[lines.length - 1].length + 1;
    
    editorPanes.updateCursor(activeFile.path, line, column);
  }
  
  function handleClick(e: MouseEvent) {
    const target = e.target as HTMLTextAreaElement;
    if (activeFile) {
      updateCursorPosition(target);
    }
  }
  
  function handleKeyUp(e: KeyboardEvent) {
    const target = e.target as HTMLTextAreaElement;
    if (activeFile) {
      updateCursorPosition(target);
    }
  }
  
  function handleScroll(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    if (highlightElement) {
      highlightElement.scrollTop = target.scrollTop;
      highlightElement.scrollLeft = target.scrollLeft;
    }
    if (lineNumbersElement) {
      lineNumbersElement.scrollTop = target.scrollTop;
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
      const filename = path.split('/').pop() || 'file';
      ideStatus.saving(filename);
      
      const { invoke } = await import('@tauri-apps/api/core');
      await invoke('write_file', { path, contents: content });
      editorPanes.markSaved(path);
      originalContent = content;
      console.log('[Editor] File saved:', path);
      
      // Show success briefly then return to ready
      ideStatus.custom('ready', `Saved ${filename}`);
      setTimeout(() => ideStatus.ready(), 2000);
    } catch (err) {
      console.error('[Editor] Failed to save file:', err);
      ideStatus.error(`Failed to save: ${err}`);
      setTimeout(() => ideStatus.ready(), 3000);
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
  
  function handleTabMouseDown(e: MouseEvent, filePath: string) {
    if (e.button !== 0) return; // Only left click
    
    e.preventDefault(); // Prevent text selection immediately
    
    console.log('[MOUSE] Tab mousedown:', filePath);
    consoleStore.log('info', 'editor', `Mouse down on ${filePath.split('/').pop()}`);
    
    const startX = e.clientX;
    const startY = e.clientY;
    let isDragging = false;
    
    const handleMouseMove = (e: MouseEvent) => {
      e.preventDefault(); // Always prevent default during any mouse move
      
      const dx = Math.abs(e.clientX - startX);
      const dy = Math.abs(e.clientY - startY);
      
      if (!isDragging && (dx > 5 || dy > 5)) {
        isDragging = true;
        console.log('[MOUSE] Started dragging');
        consoleStore.log('info', 'editor', `Dragging ${filePath.split('/').pop()}`);
        onDragStart(pane.id, filePath);
        document.body.style.userSelect = 'none';
        document.body.style.webkitUserSelect = 'none';
        document.body.style.cursor = 'grabbing';
      }
      
      if (isDragging) {
        // Find which pane we're over
        const elements = document.elementsFromPoint(e.clientX, e.clientY);
        const paneWrapper = elements.find(el => el.classList.contains('pane-wrapper'));
        if (paneWrapper) {
          const targetPaneId = paneWrapper.getAttribute('data-pane-id');
          if (targetPaneId) {
            console.log('[MOUSE] Over pane:', targetPaneId);
          }
        }
      }
    };
    
    const handleMouseUp = (e: MouseEvent) => {
      console.log('[MOUSE] Mouse up, isDragging:', isDragging);
      
      if (isDragging) {
        e.preventDefault();
        // Find which pane we dropped on
        const elements = document.elementsFromPoint(e.clientX, e.clientY);
        const paneWrapper = elements.find(el => el.classList.contains('pane-wrapper'));
        if (paneWrapper) {
          const targetPaneId = paneWrapper.getAttribute('data-pane-id');
          if (targetPaneId) {
            console.log('[MOUSE] Dropped on pane:', targetPaneId);
            consoleStore.log('info', 'editor', `Dropped on ${targetPaneId}`);
            onDrop(targetPaneId);
          }
        }
        
        // Restore cursor and text selection
        document.body.style.userSelect = '';
        document.body.style.webkitUserSelect = '';
        document.body.style.cursor = '';
      }
      
      onDragEnd();
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
    
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }
  
  function handleTabDragStart(e: DragEvent, filePath: string) {
    if (!e.dataTransfer) return;
    
    console.log('[TAB DRAG] Starting drag for:', filePath);
    consoleStore.log('info', 'editor', `Tab drag start: ${filePath.split('/').pop()}`);
    
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', filePath);
    e.dataTransfer.setData('application/x-axiom-file', filePath);
    
    // Create a custom drag image to avoid the tab blocking drops
    const dragImage = document.createElement('div');
    dragImage.textContent = filePath.split('/').pop() || '';
    dragImage.style.position = 'absolute';
    dragImage.style.top = '-1000px';
    dragImage.style.padding = '8px 12px';
    dragImage.style.background = '#58a6ff';
    dragImage.style.color = 'white';
    dragImage.style.borderRadius = '4px';
    dragImage.style.fontWeight = '600';
    document.body.appendChild(dragImage);
    e.dataTransfer.setDragImage(dragImage, 0, 0);
    setTimeout(() => document.body.removeChild(dragImage), 0);
    
    console.log('[TAB DRAG] Drag data set, calling onDragStart');
    onDragStart(pane.id, filePath);
  }
  
  function handleTabDragEnd(e: DragEvent) {
    console.log('[DragDrop] Drag ended');
    onDragEnd();
  }
  
  function getLanguageLabel(lang: string): string {
    switch (lang) {
      case 'c': return 'C';
      case 'cpp': return 'C++';
      case 'python': return 'Python';
      case 'assembly': return 'ARM Assembly';
      case 'makefile': return 'Makefile';
      case 'linker': return 'Linker Script';
      case 'markdown': return 'Markdown';
      case 'javascript': return 'JavaScript';
      case 'typescript': return 'TypeScript';
      case 'html': return 'HTML';
      case 'css': return 'CSS';
      case 'xml': return 'XML';
      case 'json': return 'JSON';
      case 'yaml': return 'YAML';
      case 'svelte': return 'Svelte';
      case 'astro': return 'Astro';
      case 'dockerfile': return 'Dockerfile';
      case 'gitignore': return 'Gitignore';
      case 'bash': return 'Shell Script';
      case 'rust': return 'Rust';
      case 'go': return 'Go';
      case 'java': return 'Java';
      case 'sql': return 'SQL';
      case 'text': return 'Text';
      default: return 'Text';
    }
  }
</script>

<div class="editor-pane" 
     role="region"
     aria-label="Editor pane">
  {#if pane.files.length > 0}
    <div class="editor-tabs">
      {#each pane.files as file, i (file.path)}
        <div 
          class="tab" 
          class:active={i === pane.activeIndex}
          class:flashing={$editorPanes.flashingTab?.paneId === pane.id && $editorPanes.flashingTab?.filePath === file.path}
          on:mousedown={(e) => handleTabMouseDown(e, file.path)}
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
          <div class="file-info-right">
            <button 
              class="toggle-line-numbers" 
              on:click={toggleLineNumbers}
              title={showLineNumbers ? 'Hide line numbers' : 'Show line numbers'}
            >
              <svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">
                <line x1="6" y1="3" x2="14" y2="3"/>
                <line x1="6" y1="8" x2="14" y2="8"/>
                <line x1="6" y1="13" x2="14" y2="13"/>
                <text x="2" y="5" font-size="5" fill="currentColor">1</text>
                <text x="2" y="10" font-size="5" fill="currentColor">2</text>
                <text x="2" y="15" font-size="5" fill="currentColor">3</text>
              </svg>
            </button>
            <span class="file-lang">{getLanguageLabel(activeFile.language)}</span>
          </div>
        </div>
        <div class="editor-container" class:show-line-numbers={showLineNumbers}>
          {#if showLineNumbers}
            <div class="line-numbers" bind:this={lineNumbersElement} aria-hidden="true">
              {#each activeFile.content.split('\n') as _, i}
                <div class="line-number">{i + 1}</div>
              {/each}
            </div>
          {/if}
          <div class="editor-wrapper">
            <pre class="code-highlight" bind:this={highlightElement} aria-hidden="true"><code>{#each highlightedContent as token}<span class="token-{token.type}">{token.value}</span>{/each}</code></pre>
            <textarea
              bind:this={editorElement}
              class="code-editor"
              value={activeFile.content}
              on:input={handleInput}
              on:scroll={handleScroll}
              on:keydown={handleKeyDown}
              on:keyup={handleKeyUp}
              on:click={handleClick}
              spellcheck="false"
              autocomplete="off"
              autocorrect="off"
              autocapitalize="off"
            ></textarea>
          </div>
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
    user-select: none;
    -webkit-user-select: none;
  }
  
  .tab:active {
    cursor: grabbing;
  }
  
  .tab.flashing {
    animation: flash-tab 0.6s ease-out;
  }
  
  @keyframes flash-tab {
    0% {
      background: var(--color-accent);
      transform: scale(1);
    }
    50% {
      background: var(--color-accent);
      transform: scale(1.05);
    }
    100% {
      background: var(--color-bg-tertiary);
      transform: scale(1);
    }
  }
  
  .tab.active.flashing {
    animation: flash-tab-active 0.6s ease-out;
  }
  
  @keyframes flash-tab-active {
    0% {
      background: var(--color-accent);
      transform: scale(1);
    }
    50% {
      background: var(--color-accent);
      transform: scale(1.05);
    }
    100% {
      background: var(--color-bg-primary);
      transform: scale(1);
    }
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

  /* Line numbers */
  .file-info-right {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .toggle-line-numbers {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-muted);
    transition: color 0.15s, background 0.15s;
    border-radius: 3px;
  }

  .toggle-line-numbers:hover {
    color: var(--color-text-primary);
    background: var(--color-bg-hover);
  }

  .toggle-line-numbers svg {
    width: 14px;
    height: 14px;
  }

  .editor-container {
    display: flex;
  }

  .line-numbers {
    flex-shrink: 0;
    width: 50px;
    background: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    overflow: hidden;
    user-select: none;
    padding: 12px 0;
  }

  .line-number {
    height: 19.5px; /* Match line-height of 1.5 * 13px font-size */
    padding: 0 8px;
    text-align: right;
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.5;
    color: var(--color-text-muted);
    opacity: 0.6;
  }

  .editor-wrapper {
    position: relative;
    flex: 1;
    overflow: hidden;
  }
</style>
