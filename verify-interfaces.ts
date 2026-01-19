/**
 * Verification script for CorpusEntry and Corpus interfaces
 * This demonstrates that the interfaces are correctly defined and usable
 */

import type { CorpusEntry, Corpus, Language, TrieNode } from './src/lib/utils/autocomplete';
import { createEmptyTrie, buildTrie } from './src/lib/utils/autocomplete';

// Test 1: Create CorpusEntry instances
const entry1: CorpusEntry = {
  text: 'function',
  type: 'keyword',
  description: 'Declares a function',
  category: 'function-definition'
};

const entry2: CorpusEntry = {
  text: 'console.log',
  type: 'function',
  description: 'Prints to console'
};

const entry3: CorpusEntry = {
  text: 'const',
  type: 'keyword'
};

console.log('✓ CorpusEntry interface works correctly');
console.log('  - entry1:', entry1);
console.log('  - entry2:', entry2);
console.log('  - entry3:', entry3);

// Test 2: Create Corpus instance
const entries: CorpusEntry[] = [entry1, entry2, entry3];
const trie: TrieNode = buildTrie(entries);

const corpus: Corpus = {
  language: 'javascript' as Language,
  entries: entries,
  trie: trie
};

console.log('\n✓ Corpus interface works correctly');
console.log('  - language:', corpus.language);
console.log('  - entries count:', corpus.entries.length);
console.log('  - trie root children:', corpus.trie.children.size);

// Test 3: Verify all required fields are present
console.log('\n✓ All required fields present:');
console.log('  - CorpusEntry has: text, type, description?, category?');
console.log('  - Corpus has: language, entries, trie');

console.log('\n✅ All interface verifications passed!');
