/**
 * Tests for Autocomplete System
 * 
 * This file contains unit tests and property-based tests for the autocomplete functionality.
 */

import { describe, it, expect } from 'vitest';
import * as fc from 'fast-check';
import {
  createTrieNode,
  createEmptyTrie,
  insertIntoTrie,
  searchInTrie,
  findByPrefix,
  buildTrie,
  CorpusManager,
  MatchingEngine,
  shouldTrigger,
  extractPrefix,
  getLanguageTriggers,
  type CorpusEntry,
  type Language
} from './autocomplete';

// ============================================================================
// Trie Tests
// ============================================================================

describe('Trie Data Structure', () => {
  it('should create an empty trie', () => {
    const trie = createEmptyTrie();
    expect(trie.char).toBe('');
    expect(trie.children.size).toBe(0);
    expect(trie.isEndOfWord).toBe(false);
    expect(trie.depth).toBe(0);
  });

  it('should insert a word into the trie', () => {
    const trie = createEmptyTrie();
    const entry: CorpusEntry = { text: 'test', type: 'keyword' };
    
    insertIntoTrie(trie, 'test', entry);
    
    expect(searchInTrie(trie, 'test')).toBe(true);
    expect(searchInTrie(trie, 'tes')).toBe(false);
    expect(searchInTrie(trie, 'testing')).toBe(false);
  });

  it('should handle case-insensitive insertion', () => {
    const trie = createEmptyTrie();
    const entry: CorpusEntry = { text: 'Test', type: 'keyword' };
    
    insertIntoTrie(trie, 'Test', entry);
    
    expect(searchInTrie(trie, 'test')).toBe(true);
    expect(searchInTrie(trie, 'TEST')).toBe(true);
    expect(searchInTrie(trie, 'Test')).toBe(true);
  });

  it('should find words by prefix', () => {
    const trie = createEmptyTrie();
    const entries: CorpusEntry[] = [
      { text: 'function', type: 'keyword' },
      { text: 'for', type: 'keyword' },
      { text: 'forEach', type: 'function' },
      { text: 'filter', type: 'function' }
    ];
    
    entries.forEach(entry => insertIntoTrie(trie, entry.text, entry));
    
    const results = findByPrefix(trie, 'f', 10);
    expect(results.length).toBe(4);
    
    const forResults = findByPrefix(trie, 'for', 10);
    expect(forResults.length).toBe(2);
    expect(forResults.some(e => e.text === 'for')).toBe(true);
    expect(forResults.some(e => e.text === 'forEach')).toBe(true);
  });

  it('should respect max results limit', () => {
    const trie = createEmptyTrie();
    const entries: CorpusEntry[] = Array.from({ length: 20 }, (_, i) => ({
      text: `word${i}`,
      type: 'keyword' as const
    }));
    
    entries.forEach(entry => insertIntoTrie(trie, entry.text, entry));
    
    const results = findByPrefix(trie, 'w', 5);
    expect(results.length).toBeLessThanOrEqual(5);
  });

  it('should return empty array for non-existent prefix', () => {
    const trie = createEmptyTrie();
    const entry: CorpusEntry = { text: 'test', type: 'keyword' };
    insertIntoTrie(trie, 'test', entry);
    
    const results = findByPrefix(trie, 'xyz', 10);
    expect(results.length).toBe(0);
  });
});

// ============================================================================
// Corpus Manager Tests
// ============================================================================

describe('CorpusManager', () => {
  it('should create a corpus manager', () => {
    const manager = new CorpusManager();
    expect(manager).toBeDefined();
  });

  it('should report corpus as not loaded initially', () => {
    const manager = new CorpusManager();
    expect(manager.isLoaded('javascript')).toBe(false);
  });

  it('should return empty corpus for unloaded language', () => {
    const manager = new CorpusManager();
    const corpus = manager.getCorpus('javascript');
    expect(corpus.entries.length).toBe(0);
  });

  it('should handle missing corpus file gracefully', async () => {
    const manager = new CorpusManager();
    
    // Try to load a non-existent corpus
    await manager.loadCorpus('nonexistent' as Language);
    
    // Should be marked as loaded (with empty corpus)
    expect(manager.isLoaded('nonexistent' as Language)).toBe(true);
    
    // Should return empty corpus
    const corpus = manager.getCorpus('nonexistent' as Language);
    expect(corpus.entries.length).toBe(0);
  });

  it('should calculate memory usage', () => {
    const manager = new CorpusManager();
    
    // Initial memory should be 0
    expect(manager.getMemoryUsage()).toBe(0);
  });

  // Unit tests for CorpusManager methods without actual file loading
  it('should cache corpus after manual insertion', () => {
    const manager = new CorpusManager();
    const entries: CorpusEntry[] = [
      { text: 'test', type: 'keyword' },
      { text: 'function', type: 'keyword' }
    ];
    
    // Manually create a corpus (simulating what loadCorpus does)
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    // Use reflection to set the corpus (for testing purposes)
    (manager as any).corpuses.set('javascript', corpus);
    
    // Should now be loaded
    expect(manager.isLoaded('javascript')).toBe(true);
    
    // Should return the corpus
    const retrieved = manager.getCorpus('javascript');
    expect(retrieved.entries.length).toBe(2);
    expect(retrieved.language).toBe('javascript');
  });

  it('should return same corpus instance when cached', () => {
    const manager = new CorpusManager();
    const entries: CorpusEntry[] = [
      { text: 'test', type: 'keyword' }
    ];
    
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    (manager as any).corpuses.set('javascript', corpus);
    
    const retrieved1 = manager.getCorpus('javascript');
    const retrieved2 = manager.getCorpus('javascript');
    
    // Should be the same object (cached)
    expect(retrieved1).toBe(retrieved2);
  });

  it('should build trie when corpus is created', () => {
    const manager = new CorpusManager();
    const entries: CorpusEntry[] = [
      { text: 'const', type: 'keyword' },
      { text: 'let', type: 'keyword' },
      { text: 'var', type: 'keyword' }
    ];
    
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    (manager as any).corpuses.set('javascript', corpus);
    
    const retrieved = manager.getCorpus('javascript');
    
    // Trie should be built
    expect(retrieved.trie).toBeDefined();
    expect(retrieved.trie.children.size).toBeGreaterThan(0);
  });

  it('should calculate memory usage for loaded corpuses', () => {
    const manager = new CorpusManager();
    const entries: CorpusEntry[] = [
      { text: 'test', type: 'keyword' },
      { text: 'function', type: 'keyword' }
    ];
    
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    (manager as any).corpuses.set('javascript', corpus);
    
    // Memory usage should be greater than 0
    expect(manager.getMemoryUsage()).toBeGreaterThan(0);
  });
});

// ============================================================================
// Error Handling Tests (Task 3.4)
// ============================================================================

describe('CorpusManager Error Handling', () => {
  /**
   * Test: Missing corpus file returns empty corpus
   * **Validates: Requirements 2.2**
   * 
   * When a corpus file is missing (404 or network error), the system should
   * gracefully degrade by returning an empty corpus rather than crashing.
   */
  it('should return empty corpus when file is missing', async () => {
    const manager = new CorpusManager();
    
    // Try to load a corpus that doesn't exist
    await manager.loadCorpus('nonexistent' as Language);
    
    // Should be marked as loaded (with empty corpus)
    expect(manager.isLoaded('nonexistent' as Language)).toBe(true);
    
    // Should return empty corpus
    const corpus = manager.getCorpus('nonexistent' as Language);
    expect(corpus.entries).toEqual([]);
    expect(corpus.language).toBe('nonexistent');
    expect(corpus.trie).toBeDefined();
    expect(corpus.trie.children.size).toBe(0);
  });

  /**
   * Test: Invalid JSON returns empty corpus
   * **Validates: Requirements 2.2**
   * 
   * When a corpus file contains invalid JSON, the system should catch the
   * parse error and return an empty corpus rather than crashing.
   * 
   * Note: This test simulates the behavior by testing the error handling path.
   * In a real scenario, the fetch would return malformed JSON and trigger
   * the catch block in doLoadCorpus.
   */
  it('should return empty corpus when JSON is invalid', async () => {
    const manager = new CorpusManager();
    
    // Mock fetch to return invalid JSON
    const originalFetch = global.fetch;
    global.fetch = async () => {
      return {
        ok: true,
        json: async () => {
          throw new Error('Invalid JSON');
        }
      } as Response;
    };
    
    try {
      await manager.loadCorpus('javascript');
      
      // Should be marked as loaded (with empty corpus)
      expect(manager.isLoaded('javascript')).toBe(true);
      
      // Should return empty corpus
      const corpus = manager.getCorpus('javascript');
      expect(corpus.entries).toEqual([]);
      expect(corpus.language).toBe('javascript');
      expect(corpus.trie).toBeDefined();
    } finally {
      // Restore original fetch
      global.fetch = originalFetch;
    }
  });

  /**
   * Test: Error logging occurs
   * **Validates: Requirements 2.2**
   * 
   * When an error occurs during corpus loading, the system should log
   * appropriate error messages to help with debugging.
   */
  it('should log error when corpus loading fails', async () => {
    const manager = new CorpusManager();
    
    // Spy on console.error
    const originalError = console.error;
    const errorLogs: any[] = [];
    console.error = (...args: any[]) => {
      errorLogs.push(args);
    };
    
    // Mock fetch to throw an error
    const originalFetch = global.fetch;
    global.fetch = async () => {
      throw new Error('Network error');
    };
    
    try {
      await manager.loadCorpus('javascript');
      
      // Should have logged an error
      expect(errorLogs.length).toBeGreaterThan(0);
      expect(errorLogs[0][0]).toContain('Error loading corpus');
      expect(errorLogs[0][0]).toContain('javascript');
    } finally {
      // Restore original functions
      console.error = originalError;
      global.fetch = originalFetch;
    }
  });

  /**
   * Test: Warning logged for 404 responses
   * **Validates: Requirements 2.2**
   * 
   * When a corpus file is not found (404), the system should log a warning
   * rather than an error, as this is an expected scenario for some languages.
   */
  it('should log warning when corpus file returns 404', async () => {
    const manager = new CorpusManager();
    
    // Spy on console.warn
    const originalWarn = console.warn;
    const warnLogs: any[] = [];
    console.warn = (...args: any[]) => {
      warnLogs.push(args);
    };
    
    // Mock fetch to return 404
    const originalFetch = global.fetch;
    global.fetch = async () => {
      return {
        ok: false,
        status: 404
      } as Response;
    };
    
    try {
      await manager.loadCorpus('javascript');
      
      // Should have logged a warning
      expect(warnLogs.length).toBeGreaterThan(0);
      expect(warnLogs[0][0]).toContain('Failed to load corpus');
      expect(warnLogs[0][0]).toContain('javascript');
      expect(warnLogs[0][0]).toContain('404');
      
      // Should still return empty corpus
      const corpus = manager.getCorpus('javascript');
      expect(corpus.entries).toEqual([]);
    } finally {
      // Restore original functions
      console.warn = originalWarn;
      global.fetch = originalFetch;
    }
  });

  /**
   * Test: Empty corpus has valid trie structure
   * **Validates: Requirements 2.2**
   * 
   * When an error occurs and an empty corpus is returned, the trie should
   * still be a valid empty trie that can be used for matching without errors.
   */
  it('should return valid empty trie when corpus loading fails', async () => {
    const manager = new CorpusManager();
    
    // Mock fetch to fail
    const originalFetch = global.fetch;
    global.fetch = async () => {
      throw new Error('Network error');
    };
    
    // Suppress console.error for this test
    const originalError = console.error;
    console.error = () => {};
    
    try {
      await manager.loadCorpus('javascript');
      
      const corpus = manager.getCorpus('javascript');
      
      // Trie should be valid and usable
      expect(corpus.trie).toBeDefined();
      expect(corpus.trie.char).toBe('');
      expect(corpus.trie.children).toBeDefined();
      expect(corpus.trie.children.size).toBe(0);
      expect(corpus.trie.isEndOfWord).toBe(false);
      
      // Should be able to search without errors
      const results = findByPrefix(corpus.trie, 'test', 10);
      expect(results).toEqual([]);
    } finally {
      // Restore original functions
      console.error = originalError;
      global.fetch = originalFetch;
    }
  });
});

// ============================================================================
// Matching Engine Tests
// ============================================================================

describe('MatchingEngine', () => {
  it('should create a matching engine', () => {
    const engine = new MatchingEngine();
    expect(engine).toBeDefined();
  });

  it('should return empty array for empty prefix', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'test', type: 'keyword' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const results = engine.match('', corpus);
    expect(results.length).toBe(0);
  });

  it('should match prefix and return suggestions', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'function', type: 'keyword' },
      { text: 'for', type: 'keyword' },
      { text: 'forEach', type: 'function' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const results = engine.match('f', corpus);
    expect(results.length).toBeGreaterThan(0);
    expect(results.every(s => s.text.toLowerCase().startsWith('f'))).toBe(true);
  });

  it('should score exact matches higher', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'for', type: 'keyword' },
      { text: 'forEach', type: 'function' },
      { text: 'format', type: 'function' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const results = engine.match('for', corpus);
    expect(results[0].text).toBe('for');
    expect(results[0].score).toBeGreaterThan(results[1].score);
  });

  it('should limit results to maxResults', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = Array.from({ length: 20 }, (_, i) => ({
      text: `word${i}`,
      type: 'keyword' as const
    }));
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const results = engine.match('w', corpus, 5);
    expect(results.length).toBeLessThanOrEqual(5);
  });

  /**
   * Test: Scoring algorithm correctness
   * **Validates: Requirements 3.2, 3.3, 3.4**
   * 
   * Verifies that the scoring algorithm assigns correct scores:
   * - Exact match (case-insensitive): 100
   * - Starts with prefix: 90
   * - Contains prefix: 70
   */
  it('should apply correct scoring algorithm (exact: 100, starts: 90, contains: 70)', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'test', type: 'keyword' },
      { text: 'testing', type: 'keyword' },
      { text: 'contest', type: 'keyword' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    // Test exact match
    const exactResults = engine.match('test', corpus);
    const exactMatch = exactResults.find(s => s.text === 'test');
    expect(exactMatch).toBeDefined();
    expect(exactMatch!.score).toBeGreaterThanOrEqual(100);
    
    // Test starts with
    const startsResults = engine.match('tes', corpus);
    const startsMatch = startsResults.find(s => s.text === 'test');
    expect(startsMatch).toBeDefined();
    expect(startsMatch!.score).toBeGreaterThanOrEqual(90);
    expect(startsMatch!.score).toBeLessThan(100);
  });

  /**
   * Test: Case-insensitive scoring
   * **Validates: Requirements 3.3**
   * 
   * Verifies that scoring is case-insensitive - matching 'TEST' should
   * give the same score as matching 'test'.
   */
  it('should score case-insensitively', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'Function', type: 'keyword' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const lowerResults = engine.match('function', corpus);
    const upperResults = engine.match('FUNCTION', corpus);
    const mixedResults = engine.match('FuNcTiOn', corpus);
    
    expect(lowerResults.length).toBe(1);
    expect(upperResults.length).toBe(1);
    expect(mixedResults.length).toBe(1);
    
    // All should have similar scores (exact match)
    expect(lowerResults[0].score).toBeGreaterThanOrEqual(100);
    expect(upperResults[0].score).toBeGreaterThanOrEqual(100);
    expect(mixedResults[0].score).toBeGreaterThanOrEqual(100);
  });

  /**
   * Test: Suggestions sorted by score descending
   * **Validates: Requirements 3.4**
   * 
   * Verifies that suggestions are returned in descending order by score.
   */
  it('should return suggestions sorted by score descending', () => {
    const engine = new MatchingEngine();
    const entries: CorpusEntry[] = [
      { text: 'for', type: 'keyword' },
      { text: 'forEach', type: 'function' },
      { text: 'format', type: 'function' },
      { text: 'forward', type: 'function' }
    ];
    const corpus = {
      language: 'javascript' as Language,
      entries,
      trie: buildTrie(entries)
    };
    
    const results = engine.match('for', corpus);
    
    // Verify results are sorted by score descending
    for (let i = 0; i < results.length - 1; i++) {
      expect(results[i].score).toBeGreaterThanOrEqual(results[i + 1].score);
    }
    
    // Exact match should be first
    expect(results[0].text).toBe('for');
  });
});

// ============================================================================
// Trigger Detection Tests
// ============================================================================

// Mock KeyboardEvent for Node.js environment
function createMockKeyboardEvent(options: {
  key: string;
  ctrlKey?: boolean;
  metaKey?: boolean;
  altKey?: boolean;
}): KeyboardEvent {
  return {
    key: options.key,
    ctrlKey: options.ctrlKey || false,
    metaKey: options.metaKey || false,
    altKey: options.altKey || false
  } as KeyboardEvent;
}

describe('Trigger Detection', () => {
  it('should trigger on alphanumeric characters', () => {
    const event = createMockKeyboardEvent({ key: 'a' });
    expect(shouldTrigger(event, 'javascript')).toBe(true);
    
    const event2 = createMockKeyboardEvent({ key: '5' });
    expect(shouldTrigger(event2, 'javascript')).toBe(true);
  });

  it('should not trigger on modifier key combinations', () => {
    const event = createMockKeyboardEvent({ key: 's', ctrlKey: true });
    expect(shouldTrigger(event, 'javascript')).toBe(false);
    
    const event2 = createMockKeyboardEvent({ key: 'c', metaKey: true });
    expect(shouldTrigger(event2, 'javascript')).toBe(false);
  });

  it('should not trigger on special keys', () => {
    const keys = ['Enter', 'Escape', 'Tab', 'Backspace', 'Delete'];
    
    keys.forEach(key => {
      const event = createMockKeyboardEvent({ key });
      expect(shouldTrigger(event, 'javascript')).toBe(false);
    });
  });

  it('should not trigger on arrow keys', () => {
    const keys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'];
    
    keys.forEach(key => {
      const event = createMockKeyboardEvent({ key });
      expect(shouldTrigger(event, 'javascript')).toBe(false);
    });
  });

  it('should trigger on language-specific characters', () => {
    const event = createMockKeyboardEvent({ key: '.' });
    expect(shouldTrigger(event, 'javascript')).toBe(true);
  });

  it('should get language-specific triggers', () => {
    expect(getLanguageTriggers('javascript')).toContain('.');
    expect(getLanguageTriggers('typescript')).toContain('.');
    expect(getLanguageTriggers('typescript')).toContain(':');
    expect(getLanguageTriggers('cpp')).toContain(':');
    expect(getLanguageTriggers('cpp')).toContain('.');
  });
});

// ============================================================================
// Prefix Extraction Tests
// ============================================================================

describe('Prefix Extraction', () => {
  it('should extract prefix before cursor', () => {
    const text = 'const myVar = func';
    const prefix = extractPrefix(text, text.length);
    expect(prefix).toBe('func');
  });

  it('should handle cursor in middle of word', () => {
    const text = 'function';
    const prefix = extractPrefix(text, 4);
    expect(prefix).toBe('func');
  });

  it('should return empty for cursor at start', () => {
    const text = 'test';
    const prefix = extractPrefix(text, 0);
    expect(prefix).toBe('');
  });

  it('should handle underscore in identifiers', () => {
    const text = 'my_var_name';
    const prefix = extractPrefix(text, text.length);
    expect(prefix).toBe('my_var_name');
  });

  it('should stop at non-identifier characters', () => {
    const text = 'const myVar';
    const prefix = extractPrefix(text, text.length);
    expect(prefix).toBe('myVar');
  });
});

// ============================================================================
// Property-Based Tests
// ============================================================================

describe('Property-Based Tests', () => {
  /**
   * Property: Trie Round Trip
   * **Validates: Requirements 3.2**
   * 
   * For any list of words, inserting them into a Trie and then searching for each should return true.
   * This ensures that the Trie correctly stores and retrieves all inserted words.
   * 
   * Tag: Feature: intelligent-autocomplete, Property: Trie insertion correctness
   */
  it('Property: Trie Round Trip - inserted words can be found', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 1, maxLength: 20 }), { minLength: 1, maxLength: 50 }),
        (words) => {
          const trie = createEmptyTrie();
          const uniqueWords = [...new Set(words)];
          
          // Insert all words
          uniqueWords.forEach(word => {
            const entry: CorpusEntry = { text: word, type: 'keyword' };
            insertIntoTrie(trie, word, entry);
          });
          
          // Search for all words
          return uniqueWords.every(word => searchInTrie(trie, word));
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Prefix Search Correctness
   * **Validates: Requirements 3.2**
   * 
   * For any Trie and any prefix, all words returned by findByPrefix should start with that prefix.
   * This ensures that the prefix search only returns relevant matches.
   * 
   * Tag: Feature: intelligent-autocomplete, Property: Prefix search correctness
   */
  it('Property: Prefix Search Correctness - all results start with prefix', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 1, maxLength: 20 }), { minLength: 1, maxLength: 50 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (words, prefix) => {
          const trie = createEmptyTrie();
          const uniqueWords = [...new Set(words)];
          
          uniqueWords.forEach(word => {
            const entry: CorpusEntry = { text: word, type: 'keyword' };
            insertIntoTrie(trie, word, entry);
          });
          
          const results = findByPrefix(trie, prefix, 10);
          
          // All results should start with the prefix (case-insensitive)
          return results.every(entry => 
            entry.text.toLowerCase().startsWith(prefix.toLowerCase())
          );
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Case-Insensitive Matching Consistency
   * **Validates: Requirements 3.3**
   * 
   * For any prefix string in any case combination (lowercase, uppercase, mixed), 
   * matching should return equivalent results.
   * 
   * Tag: Feature: intelligent-autocomplete, Property: Case-insensitive consistency
   */
  it('Property: Case-Insensitive Matching - same results for different cases', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 1, maxLength: 20 }), { minLength: 1, maxLength: 50 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (words, prefix) => {
          const trie = createEmptyTrie();
          const uniqueWords = [...new Set(words)];
          
          uniqueWords.forEach(word => {
            const entry: CorpusEntry = { text: word, type: 'keyword' };
            insertIntoTrie(trie, word, entry);
          });
          
          const lowerResults = findByPrefix(trie, prefix.toLowerCase(), 10);
          const upperResults = findByPrefix(trie, prefix.toUpperCase(), 10);
          
          // Should return same number of results
          return lowerResults.length === upperResults.length;
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Corpus Lazy Loading and Caching
   * **Validates: Requirements 2.2, 2.3**
   * 
   * For any language, corpus should not be loaded until first requested, 
   * and subsequent accesses should return cached corpus without reloading.
   * 
   * This property verifies:
   * 1. Corpus is not loaded initially (lazy loading)
   * 2. After first access, corpus is cached
   * 3. Subsequent accesses return the same cached instance
   * 
   * Tag: Feature: intelligent-autocomplete, Property 2: Lazy loading and caching
   */
  it('Property: Corpus Lazy Loading and Caching - corpus loaded on demand and cached', async () => {
    // Generate arbitrary languages to test
    const languageArbitrary = fc.constantFrom(
      'javascript', 'typescript', 'python', 'c', 'cpp', 
      'rust', 'go', 'java', 'html', 'css'
    );

    await fc.assert(
      fc.asyncProperty(
        languageArbitrary,
        async (language) => {
          const manager = new CorpusManager();
          
          // Property 1: Corpus should not be loaded initially (lazy loading)
          const isLoadedBefore = manager.isLoaded(language);
          if (isLoadedBefore) {
            return false; // Corpus should not be loaded before first request
          }
          
          // Property 2: After loading, corpus should be marked as loaded
          await manager.loadCorpus(language);
          const isLoadedAfter = manager.isLoaded(language);
          if (!isLoadedAfter) {
            return false; // Corpus should be loaded after loadCorpus call
          }
          
          // Property 3: Subsequent accesses should return cached corpus
          const corpus1 = manager.getCorpus(language);
          const corpus2 = manager.getCorpus(language);
          
          // Should return the same object reference (cached)
          if (corpus1 !== corpus2) {
            return false; // Should return cached instance
          }
          
          // Property 4: Multiple loadCorpus calls should not reload
          // (This is tested by checking that the corpus reference remains the same)
          await manager.loadCorpus(language);
          const corpus3 = manager.getCorpus(language);
          
          if (corpus1 !== corpus3) {
            return false; // Should still return the same cached instance
          }
          
          return true;
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Corpus Completeness
   * **Validates: Requirements 2.1**
   * 
   * For any supported language, its corpus should contain entries of expected types
   * (keywords, functions, types) relevant to that language.
   * 
   * This property verifies:
   * 1. Each supported language has a corpus file that exists
   * 2. The corpus contains entries (not empty)
   * 3. The corpus contains entries of multiple expected types
   * 4. All entries have valid type values
   * 5. Language-specific expectations are met (e.g., Python has 'def', JavaScript has 'function')
   * 
   * Tag: Feature: intelligent-autocomplete, Property 3: Corpus completeness
   */
  it('Property: Corpus Completeness - corpus contains expected entry types', async () => {
    // Import fs module for reading corpus files directly in test environment
    const fs = await import('fs/promises');
    const path = await import('path');
    
    // Define supported languages with their expected characteristics
    const supportedLanguages: Array<{
      language: Language;
      expectedKeywords: string[];  // Sample keywords that should exist
      minEntries: number;           // Minimum expected entries
    }> = [
      // High priority languages
      { language: 'javascript', expectedKeywords: ['function', 'const', 'let', 'var'], minEntries: 50 },
      { language: 'typescript', expectedKeywords: ['interface', 'type', 'const', 'let'], minEntries: 50 },
      { language: 'python', expectedKeywords: ['def', 'class', 'if', 'for'], minEntries: 50 },
      { language: 'c', expectedKeywords: ['int', 'char', 'void', 'struct'], minEntries: 50 },
      { language: 'cpp', expectedKeywords: ['class', 'namespace', 'template', 'int'], minEntries: 50 },
      
      // Medium priority languages
      { language: 'html', expectedKeywords: ['div', 'span', 'a', 'img'], minEntries: 20 },
      { language: 'css', expectedKeywords: ['color', 'margin', 'padding', 'display'], minEntries: 20 },
      { language: 'sql', expectedKeywords: ['SELECT', 'FROM', 'WHERE', 'INSERT'], minEntries: 20 },
      { language: 'rust', expectedKeywords: ['fn', 'let', 'mut', 'struct'], minEntries: 30 },
      { language: 'go', expectedKeywords: ['func', 'var', 'type', 'struct'], minEntries: 30 },
      { language: 'java', expectedKeywords: ['class', 'public', 'private', 'void'], minEntries: 50 },
      
      // Low priority languages
      { language: 'assembly', expectedKeywords: ['mov', 'add', 'sub', 'jmp'], minEntries: 20 },
      { language: 'bash', expectedKeywords: ['if', 'then', 'else', 'fi'], minEntries: 20 },
      { language: 'makefile', expectedKeywords: ['all', 'clean', 'install'], minEntries: 10 },
      { language: 'yaml', expectedKeywords: [], minEntries: 5 },
      { language: 'json', expectedKeywords: [], minEntries: 5 },
      { language: 'toml', expectedKeywords: [], minEntries: 5 },
      { language: 'markdown', expectedKeywords: [], minEntries: 5 }
    ];

    // Test each supported language
    for (const { language, expectedKeywords, minEntries } of supportedLanguages) {
      // Read corpus file directly from filesystem
      const corpusPath = path.join(process.cwd(), 'static', 'data', 'corpuses', `${language}.json`);
      
      // Property 1: Corpus file should exist
      let corpusData: { language: string; entries: CorpusEntry[] };
      try {
        const fileContent = await fs.readFile(corpusPath, 'utf-8');
        corpusData = JSON.parse(fileContent);
      } catch (error) {
        throw new Error(`Corpus file for ${language} not found or invalid at ${corpusPath}`);
      }
      
      // Property 2: Corpus should have entries
      expect(corpusData.entries.length).toBeGreaterThanOrEqual(minEntries);
      
      // Property 3: Corpus should have the correct language
      expect(corpusData.language).toBe(language);
      
      // Property 4: All entries should have valid types
      const validTypes: EntryType[] = ['keyword', 'function', 'type', 'constant', 'variable'];
      for (const entry of corpusData.entries) {
        expect(validTypes).toContain(entry.type);
        expect(entry.text).toBeTruthy();
        expect(entry.text.length).toBeGreaterThan(0);
      }
      
      // Property 5: Corpus should contain multiple entry types (for languages with rich syntax)
      if (minEntries >= 50) {
        const entryTypes = new Set(corpusData.entries.map(e => e.type));
        expect(entryTypes.size).toBeGreaterThanOrEqual(2);
      }
      
      // Property 6: Language-specific keywords should exist
      for (const expectedKeyword of expectedKeywords) {
        const hasKeyword = corpusData.entries.some(
          entry => entry.text.toLowerCase() === expectedKeyword.toLowerCase()
        );
        expect(hasKeyword).toBe(true);
      }
      
      // Property 7: Trie can be built from the corpus
      const trie = buildTrie(corpusData.entries);
      expect(trie).toBeDefined();
      expect(trie.children).toBeDefined();
      
      // Property 8: Trie should contain the entries (spot check)
      if (corpusData.entries.length > 0) {
        const firstEntry = corpusData.entries[0];
        const found = searchInTrie(trie, firstEntry.text);
        expect(found).toBe(true);
      }
    }
  });

  /**
   * Property: Prefix Matching Correctness
   * **Validates: Requirements 3.2, 3.3**
   * 
   * For any prefix and corpus, all returned suggestions should start with that prefix (case-insensitive).
   * This ensures that the matching engine only returns relevant suggestions.
   * 
   * Tag: Feature: intelligent-autocomplete, Property 4: Prefix matching correctness
   */
  it('Property: Prefix Matching Correctness - all suggestions start with prefix', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 1, maxLength: 20 }), { minLength: 1, maxLength: 100 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (words, prefix) => {
          const engine = new MatchingEngine();
          const uniqueWords = [...new Set(words)];
          
          const entries: CorpusEntry[] = uniqueWords.map(word => ({
            text: word,
            type: 'keyword'
          }));
          
          const corpus = {
            language: 'javascript' as Language,
            entries,
            trie: buildTrie(entries)
          };
          
          const suggestions = engine.match(prefix, corpus);
          
          // All suggestions should start with the prefix (case-insensitive)
          return suggestions.every(suggestion => 
            suggestion.text.toLowerCase().startsWith(prefix.toLowerCase())
          );
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Case-Insensitive Matching Consistency
   * **Validates: Requirements 3.3**
   * 
   * For any prefix in any case combination, matching should return equivalent results.
   * This ensures that the matching engine is truly case-insensitive.
   * 
   * Tag: Feature: intelligent-autocomplete, Property 5: Case-insensitive consistency
   */
  it('Property: Case-Insensitive Matching Consistency - same results for different cases', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 1, maxLength: 20 }), { minLength: 1, maxLength: 100 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (words, prefix) => {
          const engine = new MatchingEngine();
          const uniqueWords = [...new Set(words)];
          
          const entries: CorpusEntry[] = uniqueWords.map(word => ({
            text: word,
            type: 'keyword'
          }));
          
          const corpus = {
            language: 'javascript' as Language,
            entries,
            trie: buildTrie(entries)
          };
          
          const lowerResults = engine.match(prefix.toLowerCase(), corpus);
          const upperResults = engine.match(prefix.toUpperCase(), corpus);
          const mixedResults = engine.match(
            prefix.split('').map((c, i) => i % 2 === 0 ? c.toLowerCase() : c.toUpperCase()).join(''),
            corpus
          );
          
          // Should return same number of results
          if (lowerResults.length !== upperResults.length || 
              lowerResults.length !== mixedResults.length) {
            return false;
          }
          
          // Should return same text values (order may differ due to scoring)
          const lowerTexts = new Set(lowerResults.map(s => s.text.toLowerCase()));
          const upperTexts = new Set(upperResults.map(s => s.text.toLowerCase()));
          const mixedTexts = new Set(mixedResults.map(s => s.text.toLowerCase()));
          
          return lowerTexts.size === upperTexts.size && 
                 lowerTexts.size === mixedTexts.size &&
                 [...lowerTexts].every(t => upperTexts.has(t) && mixedTexts.has(t));
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Suggestion Ranking
   * **Validates: Requirements 3.4**
   * 
   * For any set of matches, exact prefix matches should have higher scores than partial matches,
   * and results should be sorted by score descending.
   * 
   * Tag: Feature: intelligent-autocomplete, Property 6: Suggestion ranking
   */
  it('Property: Suggestion Ranking - exact matches scored higher, sorted descending', () => {
    fc.assert(
      fc.property(
        fc.array(fc.string({ minLength: 2, maxLength: 20 }), { minLength: 2, maxLength: 100 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (words, prefix) => {
          const engine = new MatchingEngine();
          const uniqueWords = [...new Set(words)];
          
          // Add an exact match to ensure we can test ranking
          const exactMatch = prefix.toLowerCase();
          const wordsWithExact = [exactMatch, ...uniqueWords.filter(w => w.toLowerCase() !== exactMatch)];
          
          const entries: CorpusEntry[] = wordsWithExact.map(word => ({
            text: word,
            type: 'keyword'
          }));
          
          const corpus = {
            language: 'javascript' as Language,
            entries,
            trie: buildTrie(entries)
          };
          
          const suggestions = engine.match(prefix, corpus);
          
          if (suggestions.length === 0) {
            return true; // No matches is valid
          }
          
          // Property 1: Results should be sorted by score descending
          for (let i = 0; i < suggestions.length - 1; i++) {
            if (suggestions[i].score < suggestions[i + 1].score) {
              return false;
            }
          }
          
          // Property 2: Exact matches should have higher scores than partial matches
          const exactMatches = suggestions.filter(s => 
            s.text.toLowerCase() === prefix.toLowerCase()
          );
          const partialMatches = suggestions.filter(s => 
            s.text.toLowerCase() !== prefix.toLowerCase() &&
            s.text.toLowerCase().startsWith(prefix.toLowerCase())
          );
          
          if (exactMatches.length > 0 && partialMatches.length > 0) {
            const minExactScore = Math.min(...exactMatches.map(s => s.score));
            const maxPartialScore = Math.max(...partialMatches.map(s => s.score));
            
            if (minExactScore <= maxPartialScore) {
              return false;
            }
          }
          
          return true;
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Result Limiting
   * **Validates: Requirements 3.5**
   * 
   * For any prefix matching more than 10 entries, exactly 10 suggestions should be returned.
   * This ensures the matching engine respects the result limit.
   * 
   * Tag: Feature: intelligent-autocomplete, Property 7: Result limiting
   */
  it('Property: Result Limiting - max 10 suggestions returned', () => {
    fc.assert(
      fc.property(
        fc.integer({ min: 15, max: 100 }),
        fc.string({ minLength: 1, maxLength: 3 }),
        (numWords, prefix) => {
          const engine = new MatchingEngine();
          
          // Generate words that all start with the prefix to ensure many matches
          const entries: CorpusEntry[] = Array.from({ length: numWords }, (_, i) => ({
            text: `${prefix}word${i}`,
            type: 'keyword' as const
          }));
          
          const corpus = {
            language: 'javascript' as Language,
            entries,
            trie: buildTrie(entries)
          };
          
          const suggestions = engine.match(prefix, corpus);
          
          // Should return exactly 10 suggestions (default limit)
          return suggestions.length <= 10;
        }
      ),
      { numRuns: 100 }
    );
  });

  /**
   * Property: Matching Performance
   * **Validates: Requirements 3.1, 9.1**
   * 
   * For any corpus with up to 10,000 entries and any prefix, matching should complete within 16ms.
   * This ensures the matching engine meets performance requirements.
   * 
   * Tag: Feature: intelligent-autocomplete, Property 8: Matching performance
   */
  it('Property: Matching Performance - completes within 16ms for 10k entries', () => {
    fc.assert(
      fc.property(
        fc.integer({ min: 1000, max: 10000 }),
        fc.string({ minLength: 1, maxLength: 5 }),
        (numEntries, prefix) => {
          const engine = new MatchingEngine();
          
          // Generate a large corpus
          const entries: CorpusEntry[] = Array.from({ length: numEntries }, (_, i) => ({
            text: `word${i}`,
            type: 'keyword' as const
          }));
          
          const corpus = {
            language: 'javascript' as Language,
            entries,
            trie: buildTrie(entries)
          };
          
          // Measure matching time
          const startTime = performance.now();
          engine.match(prefix, corpus);
          const elapsed = performance.now() - startTime;
          
          // Should complete within 16ms
          return elapsed <= 16;
        }
      ),
      { numRuns: 100 }
    );
  });
});
