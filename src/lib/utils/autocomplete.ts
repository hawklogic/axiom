/**
 * Autocomplete System - Core Logic
 * 
 * This module provides the core autocomplete functionality including:
 * - Trie data structure for efficient prefix matching
 * - Corpus management for language-specific keywords
 * - Matching engine for finding relevant suggestions
 * 
 * @module autocomplete
 */

import { consoleStore } from '$lib/stores/console';

// ============================================================================
// Type Definitions
// ============================================================================

/**
 * Supported programming languages for autocomplete
 */
export type Language = 
  | 'c' | 'cpp' | 'python' | 'assembly' | 'javascript' | 'typescript'
  | 'html' | 'css' | 'xml' | 'json' | 'yaml' | 'svelte' | 'astro'
  | 'rust' | 'go' | 'java' | 'sql' | 'bash' | 'makefile' | 'dockerfile'
  | 'markdown' | 'toml' | 'lock' | 'log' | 'cursorrules' | 'gitignore';

/**
 * Type of corpus entry
 */
export type EntryType = 
  | 'keyword'      // Language keywords (if, for, class)
  | 'function'     // Functions and methods (print, map, filter)
  | 'type'         // Types and classes (String, Array, int)
  | 'constant'     // Constants (true, false, null, Math.PI)
  | 'variable';    // Common variable names (i, j, index, result)

/**
 * A single entry in a language corpus
 */
export interface CorpusEntry {
  text: string;
  type: EntryType;
  description?: string;
  category?: string;
  insertText?: string;  // For future snippet support
}

/**
 * A language-specific corpus containing keywords and identifiers
 */
export interface Corpus {
  language: Language;
  entries: CorpusEntry[];
  trie: TrieNode;
}

/**
 * A suggestion returned by the matching engine
 */
export interface Suggestion {
  text: string;
  type: EntryType;
  description?: string;
  score: number;  // Relevance score (0-100)
}

/**
 * Node in a Trie data structure for efficient prefix matching
 */
export interface TrieNode {
  char: string;
  children: Map<string, TrieNode>;
  isEndOfWord: boolean;
  entry?: CorpusEntry;
  depth: number;
}

/**
 * Context information about the cursor position in the editor
 */
export interface CursorContext {
  line: number;
  column: number;
  lineText: string;
  prefix: string;
  language: Language;
  charBefore: string;
  charAfter: string;
}

/**
 * Position for UI rendering
 */
export interface Position {
  x: number;
  y: number;
}

/**
 * State of the autocomplete system
 */
export interface AutocompleteState {
  visible: boolean;
  suggestions: Suggestion[];
  activeIndex: number;
  prefix: string;
  position: Position;
  language: Language | null;
  debounceTimer: number | null;
}

// ============================================================================
// Trie Implementation
// ============================================================================

/**
 * Creates an empty Trie node
 */
export function createTrieNode(char: string = '', depth: number = 0): TrieNode {
  return {
    char,
    children: new Map(),
    isEndOfWord: false,
    depth
  };
}

/**
 * Creates an empty Trie (root node)
 */
export function createEmptyTrie(): TrieNode {
  return createTrieNode('', 0);
}

/**
 * Inserts a word into the Trie
 */
export function insertIntoTrie(root: TrieNode, word: string, entry: CorpusEntry): void {
  let current = root;
  const lowerWord = word.toLowerCase();
  
  for (let i = 0; i < lowerWord.length; i++) {
    const char = lowerWord[i];
    
    if (!current.children.has(char)) {
      current.children.set(char, createTrieNode(char, current.depth + 1));
    }
    
    current = current.children.get(char)!;
  }
  
  current.isEndOfWord = true;
  current.entry = entry;
}

/**
 * Searches for a word in the Trie
 */
export function searchInTrie(root: TrieNode, word: string): boolean {
  let current = root;
  const lowerWord = word.toLowerCase();
  
  for (const char of lowerWord) {
    if (!current.children.has(char)) {
      return false;
    }
    current = current.children.get(char)!;
  }
  
  return current.isEndOfWord;
}

/**
 * Finds all words in the Trie that start with the given prefix
 */
export function findByPrefix(root: TrieNode, prefix: string, maxResults: number = 10): CorpusEntry[] {
  const results: CorpusEntry[] = [];
  const lowerPrefix = prefix.toLowerCase();
  
  // Navigate to the prefix node
  let current = root;
  for (const char of lowerPrefix) {
    if (!current.children.has(char)) {
      return results;  // Prefix not found
    }
    current = current.children.get(char)!;
  }
  
  // Collect all words from this node
  collectWords(current, results, maxResults);
  
  return results;
}

/**
 * Helper function to collect all words from a Trie node (DFS)
 */
function collectWords(node: TrieNode, results: CorpusEntry[], maxResults: number): void {
  if (results.length >= maxResults) {
    return;
  }
  
  if (node.isEndOfWord && node.entry) {
    results.push(node.entry);
  }
  
  for (const child of node.children.values()) {
    collectWords(child, results, maxResults);
    if (results.length >= maxResults) {
      break;
    }
  }
}

/**
 * Builds a Trie from a list of corpus entries
 */
export function buildTrie(entries: CorpusEntry[]): TrieNode {
  const root = createEmptyTrie();
  
  for (const entry of entries) {
    insertIntoTrie(root, entry.text, entry);
  }
  
  return root;
}

// ============================================================================
// Corpus Manager
// ============================================================================

/**
 * Manages loading and caching of language corpuses
 */
export class CorpusManager {
  private corpuses: Map<Language, Corpus> = new Map();
  private loading: Map<Language, Promise<void>> = new Map();
  
  /**
   * Loads a corpus for a specific language (lazy loading)
   */
  async loadCorpus(language: Language): Promise<void> {
    // If already loaded, return immediately
    if (this.corpuses.has(language)) {
      return;
    }
    
    // If currently loading, wait for that promise
    if (this.loading.has(language)) {
      return this.loading.get(language);
    }
    
    // Start loading
    const loadPromise = this.doLoadCorpus(language);
    this.loading.set(language, loadPromise);
    
    try {
      await loadPromise;
    } finally {
      this.loading.delete(language);
    }
  }
  
  /**
   * Internal method to actually load the corpus
   */
  private async doLoadCorpus(language: Language): Promise<void> {
    try {
      const response = await fetch(`/data/corpuses/${language}.json`);
      
      if (!response.ok) {
        console.warn(`Failed to load corpus for ${language}: ${response.status}`);
        this.corpuses.set(language, {
          language,
          entries: [],
          trie: createEmptyTrie()
        });
        return;
      }
      
      const data = await response.json();
      const trie = buildTrie(data.entries);
      
      this.corpuses.set(language, {
        language,
        entries: data.entries,
        trie
      });
      
      console.log(`Loaded corpus for ${language}: ${data.entries.length} entries`);
    } catch (error) {
      console.error(`Error loading corpus for ${language}:`, error);
      this.corpuses.set(language, {
        language,
        entries: [],
        trie: createEmptyTrie()
      });
    }
  }
  
  /**
   * Gets a corpus for a language (returns empty if not loaded)
   */
  getCorpus(language: Language): Corpus {
    return this.corpuses.get(language) || {
      language,
      entries: [],
      trie: createEmptyTrie()
    };
  }
  
  /**
   * Checks if a corpus is loaded
   */
  isLoaded(language: Language): boolean {
    return this.corpuses.has(language);
  }
  
  /**
   * Preloads commonly used languages
   */
  async preloadCommon(): Promise<void> {
    const commonLanguages: Language[] = ['javascript', 'typescript', 'python', 'c', 'cpp'];
    await Promise.all(commonLanguages.map(lang => this.loadCorpus(lang)));
  }
  
  /**
   * Gets memory usage statistics (approximate)
   */
  getMemoryUsage(): number {
    let totalBytes = 0;
    
    for (const corpus of this.corpuses.values()) {
      // Rough estimate: each entry ~100 bytes, each trie node ~50 bytes
      totalBytes += corpus.entries.length * 100;
      totalBytes += this.estimateTrieSize(corpus.trie) * 50;
    }
    
    return totalBytes;
  }
  
  /**
   * Estimates the number of nodes in a Trie
   */
  private estimateTrieSize(node: TrieNode): number {
    let count = 1;
    for (const child of node.children.values()) {
      count += this.estimateTrieSize(child);
    }
    return count;
  }
}

// ============================================================================
// Matching Engine
// ============================================================================

/**
 * Performs fast prefix-based matching against a corpus
 */
export class MatchingEngine {
  /**
   * Finds matches for a prefix in a corpus
   */
  match(prefix: string, corpus: Corpus, maxResults: number = 10): Suggestion[] {
    if (!prefix || prefix.length < 1) {
      return [];
    }
    
    const startTime = performance.now();
    
    // Find entries matching the prefix
    const entries = findByPrefix(corpus.trie, prefix, maxResults * 2);
    
    // Convert to suggestions with scores
    const suggestions = entries.map(entry => this.createSuggestion(entry, prefix));
    
    // Sort by score descending
    suggestions.sort((a, b) => b.score - a.score);
    
    // Limit results
    const results = suggestions.slice(0, maxResults);
    
    const elapsed = performance.now() - startTime;
    if (elapsed > 16) {
      console.warn(`Matching took ${elapsed.toFixed(2)}ms (threshold: 16ms)`);
    }
    
    return results;
  }
  
  /**
   * Creates a suggestion from a corpus entry with relevance scoring
   */
  private createSuggestion(entry: CorpusEntry, prefix: string): Suggestion {
    const lowerText = entry.text.toLowerCase();
    const lowerPrefix = prefix.toLowerCase();
    
    let score = 0;
    
    // Exact match (case-insensitive)
    if (lowerText === lowerPrefix) {
      score = 100;
    }
    // Starts with prefix
    else if (lowerText.startsWith(lowerPrefix)) {
      score = 90;
    }
    // Contains prefix
    else if (lowerText.includes(lowerPrefix)) {
      score = 70;
    }
    
    // Bonus for shorter words (more likely to be what user wants)
    score += Math.max(0, 10 - entry.text.length);
    
    return {
      text: entry.text,
      type: entry.type,
      description: entry.description,
      score
    };
  }
}

// ============================================================================
// Trigger Detection
// ============================================================================

/**
 * Gets language-specific trigger characters
 */
export function getLanguageTriggers(language: Language): string[] {
  const triggers: Partial<Record<Language, string[]>> = {
    'javascript': ['.'],
    'typescript': ['.', ':'],
    'cpp': [':', '.', '>'],
    'python': ['.'],
    'css': ['-'],
    'rust': [':', '.'],
    'go': ['.'],
    'java': ['.']
  };
  
  return triggers[language] || [];
}

/**
 * Determines if autocomplete should trigger based on the key event
 */
export function shouldTrigger(event: KeyboardEvent, language: Language | null): boolean {
  // Don't trigger on modifier key combinations
  if (event.ctrlKey || event.metaKey || event.altKey) {
    return false;
  }
  
  // Don't trigger on special keys
  if (['Enter', 'Escape', 'Tab', 'Backspace', 'Delete'].includes(event.key)) {
    return false;
  }
  
  // Don't trigger on arrow keys
  if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) {
    return false;
  }
  
  // Trigger on alphanumeric characters
  if (/[a-zA-Z0-9_]/.test(event.key)) {
    return true;
  }
  
  // Trigger on language-specific characters
  if (language) {
    const triggers = getLanguageTriggers(language);
    if (triggers.includes(event.key)) {
      return true;
    }
  }
  
  return false;
}

/**
 * Extracts the word prefix before the cursor
 */
export function extractPrefix(text: string, cursorPosition: number): string {
  let start = cursorPosition - 1;
  
  // Move backwards while we have valid identifier characters
  while (start >= 0 && /[a-zA-Z0-9_]/.test(text[start])) {
    start--;
  }
  
  return text.substring(start + 1, cursorPosition);
}

/**
 * Extracts cursor context from editor state
 * 
 * This function parses the editor state to extract relevant context information
 * about the cursor position, including the current line, column, line text,
 * prefix being typed, and surrounding characters.
 * 
 * @param text - The full text content of the editor
 * @param cursorPosition - The absolute cursor position in the text
 * @param language - The current language of the file
 * @returns CursorContext object with extracted information
 */
export function extractCursorContext(
  text: string,
  cursorPosition: number,
  language: Language
): CursorContext {
  // Find the current line number and column
  let line = 0;
  let column = 0;
  let lineStart = 0;
  
  for (let i = 0; i < cursorPosition; i++) {
    if (text[i] === '\n') {
      line++;
      lineStart = i + 1;
      column = 0;
    } else {
      column++;
    }
  }
  
  // Extract the current line text
  let lineEnd = lineStart;
  while (lineEnd < text.length && text[lineEnd] !== '\n') {
    lineEnd++;
  }
  const lineText = text.substring(lineStart, lineEnd);
  
  // Extract the prefix before the cursor
  const prefix = extractPrefix(text, cursorPosition);
  
  // Get character before and after cursor
  const charBefore = cursorPosition > 0 ? text[cursorPosition - 1] : '';
  const charAfter = cursorPosition < text.length ? text[cursorPosition] : '';
  
  return {
    line,
    column,
    lineText,
    prefix,
    language,
    charBefore,
    charAfter
  };
}

/**
 * Detects the predominant case style in assembly code
 */
export function detectAssemblyCaseStyle(text: string): 'upper' | 'lower' | 'mixed' {
  // Extract instruction-like words (at start of lines or after whitespace)
  const instructionPattern = /(?:^|\s+)([a-z]{2,}|[A-Z]{2,})(?:\s|$)/gm;
  const matches = text.matchAll(instructionPattern);
  
  let upperCount = 0;
  let lowerCount = 0;
  
  for (const match of matches) {
    const word = match[1];
    if (word === word.toUpperCase()) {
      upperCount++;
    } else if (word === word.toLowerCase()) {
      lowerCount++;
    }
  }
  
  // Need at least 3 samples to make a determination
  if (upperCount + lowerCount < 3) {
    return 'upper'; // Default to uppercase
  }
  
  // If 80% or more are one case, use that
  const total = upperCount + lowerCount;
  if (upperCount / total >= 0.8) {
    return 'upper';
  } else if (lowerCount / total >= 0.8) {
    return 'lower';
  }
  
  return 'mixed';
}

/**
 * Transforms suggestion text to match the target case style
 */
export function transformCase(text: string, caseStyle: 'upper' | 'lower' | 'mixed'): string {
  if (caseStyle === 'lower') {
    return text.toLowerCase();
  } else if (caseStyle === 'upper') {
    return text.toUpperCase();
  }
  return text; // mixed - keep original
}

// ============================================================================
// Autocomplete Controller
// ============================================================================

/**
 * Orchestrates autocomplete lifecycle, manages state, coordinates between editor and UI
 */
export class AutocompleteController {
  private state: AutocompleteState;
  private editorElement: HTMLTextAreaElement;
  private corpusManager: CorpusManager;
  private matchingEngine: MatchingEngine;
  private assemblyCaseStyle: 'upper' | 'lower' | 'mixed' = 'upper';
  private isInserting: boolean = false;
  
  /**
   * Initialize with editor element reference
   */
  constructor(editorElement: HTMLTextAreaElement) {
    this.editorElement = editorElement;
    this.corpusManager = new CorpusManager();
    this.matchingEngine = new MatchingEngine();
    
    this.state = {
      visible: false,
      suggestions: [],
      activeIndex: 0,
      prefix: '',
      position: { x: 0, y: 0 },
      language: null,
      debounceTimer: null
    };
  }
  
  /**
   * Gets the current autocomplete state
   */
  getState(): AutocompleteState {
    return { ...this.state };
  }
  
  /**
   * Sets the current language for autocomplete
   */
  async setLanguage(language: Language | null): Promise<void> {
    this.state.language = language;
    
    // Detect case style for assembly files
    if (language === 'assembly') {
      this.assemblyCaseStyle = detectAssemblyCaseStyle(this.editorElement.value);
    }
    
    // Load corpus for the language if not already loaded
    if (language && !this.corpusManager.isLoaded(language)) {
      await this.corpusManager.loadCorpus(language);
    }
  }
  
  /**
   * Shows the completion UI
   */
  show(): void {
    this.state.visible = true;
  }
  
  /**
   * Hides the completion UI
   */
  hide(): void {
    this.state.visible = false;
    this.state.suggestions = [];
    this.state.activeIndex = 0;
    this.state.prefix = '';
  }
  
  /**
   * Updates suggestions based on current prefix
   */
  updateSuggestions(prefix: string): void {
    this.state.prefix = prefix;
    
    // Don't show suggestions for empty or very short prefixes
    if (!prefix || prefix.length < 1) {
      this.hide();
      return;
    }
    
    // Get corpus for current language
    if (!this.state.language) {
      this.hide();
      return;
    }
    
    const corpus = this.corpusManager.getCorpus(this.state.language);
    
    // Match suggestions
    let suggestions = this.matchingEngine.match(prefix, corpus, 10);
    
    // Apply case transformation for assembly
    if (this.state.language === 'assembly') {
      suggestions = suggestions.map(s => ({
        ...s,
        text: transformCase(s.text, this.assemblyCaseStyle)
      }));
    }
    
    // Update state
    this.state.suggestions = suggestions;
    this.state.activeIndex = 0;
    
    // Show or hide based on results
    if (suggestions.length > 0) {
      this.show();
    } else {
      this.hide();
    }
  }
  
  /**
   * Gets current suggestions
   */
  getSuggestions(): Suggestion[] {
    return [...this.state.suggestions];
  }
  
  /**
   * Selects the next suggestion (with wrap-around)
   */
  selectNext(): void {
    if (this.state.suggestions.length === 0) {
      return;
    }
    
    this.state.activeIndex = (this.state.activeIndex + 1) % this.state.suggestions.length;
  }
  
  /**
   * Selects the previous suggestion (with wrap-around)
   */
  selectPrevious(): void {
    if (this.state.suggestions.length === 0) {
      return;
    }
    
    this.state.activeIndex = 
      (this.state.activeIndex - 1 + this.state.suggestions.length) % this.state.suggestions.length;
  }
  
  /**
   * Gets the currently active suggestion
   */
  getActiveSuggestion(): Suggestion | null {
    if (this.state.suggestions.length === 0 || this.state.activeIndex < 0) {
      return null;
    }
    
    return this.state.suggestions[this.state.activeIndex];
  }
  
  /**
   * Inserts the selected suggestion into the editor
   */
  insertSuggestion(suggestion: Suggestion): void {
    this.isInserting = true;
    
    const cursorPosition = this.editorElement.selectionStart;
    const text = this.editorElement.value;
    
    // Re-extract the current prefix to ensure accuracy
    const actualPrefix = extractPrefix(text, cursorPosition);
    
    // Find the start of the prefix
    const prefixStart = cursorPosition - actualPrefix.length;
    
    // Replace the prefix with the suggestion
    const newText = 
      text.substring(0, prefixStart) + 
      suggestion.text + 
      text.substring(cursorPosition);
    
    // Update editor value
    this.editorElement.value = newText;
    
    // Position cursor after inserted text
    const newCursorPosition = prefixStart + suggestion.text.length;
    this.editorElement.selectionStart = newCursorPosition;
    this.editorElement.selectionEnd = newCursorPosition;
    
    // Hide the UI BEFORE triggering input event
    this.hide();
    
    // Trigger input event for undo/redo integration
    const inputEvent = new Event('input', { bubbles: true });
    this.editorElement.dispatchEvent(inputEvent);
    
    // Reset flag after a short delay to allow input event to process
    setTimeout(() => {
      this.isInserting = false;
    }, 10);
  }
  
  /**
   * Handles keyboard events from the editor
   */
  handleKeyDown(event: KeyboardEvent): void {
    // If UI is visible, handle navigation and completion keys
    if (this.state.visible) {
      // Arrow Down - select next
      if (event.key === 'ArrowDown') {
        event.preventDefault();
        this.selectNext();
        return;
      }
      
      // Arrow Up - select previous
      if (event.key === 'ArrowUp') {
        event.preventDefault();
        this.selectPrevious();
        return;
      }
      
      // Tab - accept suggestion
      if (event.key === 'Tab') {
        event.preventDefault();
        const activeSuggestion = this.getActiveSuggestion();
        if (activeSuggestion) {
          this.insertSuggestion(activeSuggestion);
        }
        return;
      }
      
      // Escape - hide UI
      if (event.key === 'Escape') {
        event.preventDefault();
        this.hide();
        return;
      }
      
      // Enter - hide UI without inserting
      if (event.key === 'Enter') {
        this.hide();
        return;
      }
    }
    
    // NEVER trigger on arrow keys
    if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) {
      return;
    }
  }
  
  /**
   * Handles input events from the editor (after text has been inserted)
   */
  handleInput(): void {
    // Don't trigger if we're currently inserting a suggestion
    if (this.isInserting) {
      return;
    }
    
    // Extract prefix at cursor with the FULL current text
    const cursorPosition = this.editorElement.selectionStart;
    const text = this.editorElement.value;
    const prefix = extractPrefix(text, cursorPosition);
    
    consoleStore.log('info', 'autocomplete', `handleInput - cursor:${cursorPosition} prefix:"${prefix}" char:"${text[cursorPosition - 1]}"`);
    
    // Only trigger if we have a valid prefix
    if (prefix.length >= 1) {
      // Update immediately without debounce to avoid lag
      this.performUpdate();
    } else {
      this.hide();
    }
  }
  
  /**
   * Debounces suggestion updates to avoid excessive matching
   */
  private debounceUpdate(): void {
    // Clear existing timer
    if (this.state.debounceTimer !== null) {
      clearTimeout(this.state.debounceTimer);
    }
    
    // Set new timer
    this.state.debounceTimer = window.setTimeout(() => {
      this.performUpdate();
      this.state.debounceTimer = null;
    }, 50);
  }
  
  /**
   * Performs the actual suggestion update
   */
  private performUpdate(): void {
    const cursorPosition = this.editorElement.selectionStart;
    const text = this.editorElement.value;
    
    // Extract prefix at cursor
    const prefix = extractPrefix(text, cursorPosition);
    
    consoleStore.log('info', 'autocomplete', `performUpdate - cursor:${cursorPosition} prefix:"${prefix}"`);
    
    // Update suggestions
    this.updateSuggestions(prefix);
    
    // Update position
    this.updatePosition();
    
    // Log to console for debugging
    if (this.state.visible && this.state.suggestions.length > 0) {
      consoleStore.log('info', 'autocomplete', `Showing ${this.state.suggestions.length} suggestions for "${prefix}"`);
    }
  }
  
  /**
   * Updates the UI position based on cursor location
   * Uses a more accurate method with a hidden span element to measure actual cursor position
   */
  private updatePosition(): void {
    if (!this.editorElement) return;
    
    const rect = this.editorElement.getBoundingClientRect();
    const style = window.getComputedStyle(this.editorElement);
    
    const cursorPos = this.editorElement.selectionStart;
    const textBeforeCursor = this.editorElement.value.substring(0, cursorPos);
    const lines = textBeforeCursor.split('\n');
    const lineNumber = lines.length - 1;
    const columnNumber = lines[lines.length - 1].length;
    
    const fontSize = parseFloat(style.fontSize) || 13;
    // Parse lineHeight - it might be "normal" or a pixel value
    let lineHeight: number;
    const lineHeightStyle = style.lineHeight;
    if (lineHeightStyle === 'normal' || !lineHeightStyle) {
      lineHeight = fontSize * 1.5;
    } else {
      lineHeight = parseFloat(lineHeightStyle);
      // If lineHeight is a unitless number, multiply by fontSize
      if (!lineHeightStyle.includes('px')) {
        lineHeight = lineHeight * fontSize;
      }
    }
    
    const paddingLeft = parseFloat(style.paddingLeft) || 12;
    const paddingTop = parseFloat(style.paddingTop) || 12;
    
    // Use a more accurate character width measurement
    // Create a temporary span to measure the actual width of the text before cursor
    const measureSpan = document.createElement('span');
    measureSpan.style.font = style.font;
    measureSpan.style.fontSize = style.fontSize;
    measureSpan.style.fontFamily = style.fontFamily;
    measureSpan.style.fontWeight = style.fontWeight;
    measureSpan.style.letterSpacing = style.letterSpacing;
    measureSpan.style.whiteSpace = 'pre';
    measureSpan.style.visibility = 'hidden';
    measureSpan.style.position = 'absolute';
    measureSpan.textContent = lines[lineNumber];
    document.body.appendChild(measureSpan);
    const textWidth = measureSpan.offsetWidth;
    document.body.removeChild(measureSpan);
    
    // Account for scroll position
    const scrollTop = this.editorElement.scrollTop;
    const scrollLeft = this.editorElement.scrollLeft;
    
    // Calculate position
    let x = rect.left + paddingLeft + textWidth - scrollLeft;
    let y = rect.top + paddingTop + (lineNumber * lineHeight) - scrollTop;
    
    // Viewport boundary checks
    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;
    const dropdownWidth = 300; // Approximate dropdown width
    const dropdownHeight = 300; // Max dropdown height
    
    // Adjust X if dropdown would go off-screen to the right
    if (x + dropdownWidth > viewportWidth) {
      x = Math.max(10, viewportWidth - dropdownWidth - 10);
    }
    
    // Adjust Y if dropdown would go off-screen at the bottom
    // Show above cursor instead of below
    if (y + dropdownHeight > viewportHeight) {
      y = rect.top + paddingTop + (lineNumber * lineHeight) - scrollTop - dropdownHeight;
      // If still off-screen at top, clamp to top of viewport
      if (y < 10) {
        y = 10;
      }
    }
    
    this.state.position = { x, y };
  }
  
  /**
   * Handles editor blur event
   */
  handleBlur(): void {
    // Hide UI when editor loses focus
    this.hide();
  }
  
  /**
   * Handles editor scroll event
   */
  handleScroll(): void {
    // Reposition UI when editor scrolls
    if (this.state.visible) {
      this.updatePosition();
    }
  }
  
  /**
   * Cleanup resources
   */
  destroy(): void {
    // Clear any pending timers
    if (this.state.debounceTimer !== null) {
      clearTimeout(this.state.debounceTimer);
      this.state.debounceTimer = null;
    }
    
    // Hide UI
    this.hide();
  }
}
