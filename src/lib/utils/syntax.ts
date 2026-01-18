// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Lightweight syntax highlighter for embedded development languages
 * Supports C, C++, Python, and ARM Assembly
 */

export interface HighlightedToken {
  type: 'keyword' | 'string' | 'comment' | 'number' | 'operator' | 'register' | 'directive' | 'function' | 'type' | 'text';
  value: string;
}

// Language definitions
const C_KEYWORDS = [
  'auto', 'break', 'case', 'char', 'const', 'continue', 'default', 'do',
  'double', 'else', 'enum', 'extern', 'float', 'for', 'goto', 'if',
  'inline', 'int', 'long', 'register', 'restrict', 'return', 'short',
  'signed', 'sizeof', 'static', 'struct', 'switch', 'typedef', 'union',
  'unsigned', 'void', 'volatile', 'while', '_Bool', '_Complex', '_Imaginary'
];

const CPP_KEYWORDS = [
  ...C_KEYWORDS,
  'alignas', 'alignof', 'and', 'and_eq', 'asm', 'bitand', 'bitor',
  'bool', 'catch', 'class', 'compl', 'constexpr', 'const_cast',
  'decltype', 'delete', 'dynamic_cast', 'explicit', 'export', 'false',
  'friend', 'mutable', 'namespace', 'new', 'noexcept', 'not', 'not_eq',
  'nullptr', 'operator', 'or', 'or_eq', 'private', 'protected', 'public',
  'reinterpret_cast', 'static_assert', 'static_cast', 'template', 'this',
  'thread_local', 'throw', 'true', 'try', 'typeid', 'typename', 'using',
  'virtual', 'wchar_t', 'xor', 'xor_eq'
];

const PYTHON_KEYWORDS = [
  'False', 'None', 'True', 'and', 'as', 'assert', 'async', 'await',
  'break', 'class', 'continue', 'def', 'del', 'elif', 'else', 'except',
  'finally', 'for', 'from', 'global', 'if', 'import', 'in', 'is',
  'lambda', 'nonlocal', 'not', 'or', 'pass', 'raise', 'return', 'try',
  'while', 'with', 'yield'
];

// ARM Assembly registers and directives
const ARM_REGISTERS = [
  'r0', 'r1', 'r2', 'r3', 'r4', 'r5', 'r6', 'r7', 'r8', 'r9', 'r10', 'r11', 'r12',
  'sp', 'lr', 'pc', 'cpsr', 'spsr',
  'x0', 'x1', 'x2', 'x3', 'x4', 'x5', 'x6', 'x7', 'x8', 'x9', 'x10', 'x11', 'x12', 'x13', 'x14', 'x15',
  'x16', 'x17', 'x18', 'x19', 'x20', 'x21', 'x22', 'x23', 'x24', 'x25', 'x26', 'x27', 'x28', 'x29', 'x30',
  'w0', 'w1', 'w2', 'w3', 'w4', 'w5', 'w6', 'w7', 'w8', 'w9', 'w10', 'w11', 'w12', 'w13', 'w14', 'w15',
  'w16', 'w17', 'w18', 'w19', 'w20', 'w21', 'w22', 'w23', 'w24', 'w25', 'w26', 'w27', 'w28', 'w29', 'w30'
];

const ARM_DIRECTIVES = [
  '.text', '.data', '.bss', '.section', '.global', '.extern', '.align', '.word', '.byte', '.ascii', '.asciz',
  '.equ', '.set', '.if', '.else', '.endif', '.macro', '.endm', '.include'
];

const ARM_INSTRUCTIONS = [
  'mov', 'add', 'sub', 'mul', 'div', 'and', 'orr', 'eor', 'lsl', 'lsr', 'asr', 'ror',
  'ldr', 'str', 'ldm', 'stm', 'push', 'pop', 'b', 'bl', 'bx', 'blx',
  'cmp', 'tst', 'teq', 'cmn', 'beq', 'bne', 'bcs', 'bcc', 'bmi', 'bpl', 'bvs', 'bvc',
  'bhi', 'bls', 'bge', 'blt', 'bgt', 'ble', 'bal', 'nop'
];

const JS_KEYWORDS = [
  'async', 'await', 'break', 'case', 'catch', 'class', 'const', 'continue',
  'debugger', 'default', 'delete', 'do', 'else', 'export', 'extends', 'false',
  'finally', 'for', 'function', 'if', 'import', 'in', 'instanceof', 'let',
  'new', 'null', 'return', 'static', 'super', 'switch', 'this', 'throw',
  'true', 'try', 'typeof', 'var', 'void', 'while', 'with', 'yield'
];

const TS_KEYWORDS = [
  ...JS_KEYWORDS,
  'abstract', 'as', 'asserts', 'any', 'boolean', 'constructor', 'declare',
  'enum', 'from', 'get', 'implements', 'interface', 'is', 'keyof', 'module',
  'namespace', 'never', 'number', 'object', 'of', 'package', 'private',
  'protected', 'public', 'readonly', 'require', 'set', 'string', 'symbol',
  'type', 'undefined', 'unique', 'unknown'
];

const HTML_TAGS = [
  'a', 'abbr', 'address', 'area', 'article', 'aside', 'audio', 'b', 'base',
  'bdi', 'bdo', 'blockquote', 'body', 'br', 'button', 'canvas', 'caption',
  'cite', 'code', 'col', 'colgroup', 'data', 'datalist', 'dd', 'del', 'details',
  'dfn', 'dialog', 'div', 'dl', 'dt', 'em', 'embed', 'fieldset', 'figcaption',
  'figure', 'footer', 'form', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'head',
  'header', 'hr', 'html', 'i', 'iframe', 'img', 'input', 'ins', 'kbd', 'label',
  'legend', 'li', 'link', 'main', 'map', 'mark', 'meta', 'meter', 'nav',
  'noscript', 'object', 'ol', 'optgroup', 'option', 'output', 'p', 'param',
  'picture', 'pre', 'progress', 'q', 'rp', 'rt', 'ruby', 's', 'samp', 'script',
  'section', 'select', 'small', 'source', 'span', 'strong', 'style', 'sub',
  'summary', 'sup', 'table', 'tbody', 'td', 'template', 'textarea', 'tfoot',
  'th', 'thead', 'time', 'title', 'tr', 'track', 'u', 'ul', 'var', 'video', 'wbr'
];

const CSS_PROPERTIES = [
  'align', 'animation', 'background', 'border', 'bottom', 'box', 'clear',
  'clip', 'color', 'content', 'cursor', 'display', 'filter', 'flex', 'float',
  'font', 'grid', 'height', 'justify', 'left', 'letter', 'line', 'list',
  'margin', 'max', 'min', 'opacity', 'outline', 'overflow', 'padding',
  'position', 'right', 'text', 'top', 'transform', 'transition', 'vertical',
  'visibility', 'white', 'width', 'word', 'z-index'
];

const RUST_KEYWORDS = [
  'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn', 'else',
  'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in', 'let', 'loop',
  'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return', 'self', 'Self',
  'static', 'struct', 'super', 'trait', 'true', 'type', 'unsafe', 'use',
  'where', 'while', 'abstract', 'become', 'box', 'do', 'final', 'macro',
  'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield'
];

const GO_KEYWORDS = [
  'break', 'case', 'chan', 'const', 'continue', 'default', 'defer', 'else',
  'fallthrough', 'for', 'func', 'go', 'goto', 'if', 'import', 'interface',
  'map', 'package', 'range', 'return', 'select', 'struct', 'switch', 'type',
  'var', 'bool', 'byte', 'complex64', 'complex128', 'error', 'float32',
  'float64', 'int', 'int8', 'int16', 'int32', 'int64', 'rune', 'string',
  'uint', 'uint8', 'uint16', 'uint32', 'uint64', 'uintptr'
];

const JAVA_KEYWORDS = [
  'abstract', 'assert', 'boolean', 'break', 'byte', 'case', 'catch', 'char',
  'class', 'const', 'continue', 'default', 'do', 'double', 'else', 'enum',
  'extends', 'final', 'finally', 'float', 'for', 'goto', 'if', 'implements',
  'import', 'instanceof', 'int', 'interface', 'long', 'native', 'new', 'package',
  'private', 'protected', 'public', 'return', 'short', 'static', 'strictfp',
  'super', 'switch', 'synchronized', 'this', 'throw', 'throws', 'transient',
  'try', 'void', 'volatile', 'while', 'true', 'false', 'null'
];

const SQL_KEYWORDS = [
  'SELECT', 'FROM', 'WHERE', 'INSERT', 'UPDATE', 'DELETE', 'CREATE', 'DROP',
  'ALTER', 'TABLE', 'INDEX', 'VIEW', 'JOIN', 'LEFT', 'RIGHT', 'INNER', 'OUTER',
  'ON', 'AS', 'AND', 'OR', 'NOT', 'NULL', 'IS', 'IN', 'LIKE', 'BETWEEN',
  'ORDER', 'BY', 'GROUP', 'HAVING', 'LIMIT', 'OFFSET', 'UNION', 'ALL',
  'DISTINCT', 'COUNT', 'SUM', 'AVG', 'MIN', 'MAX', 'PRIMARY', 'KEY',
  'FOREIGN', 'REFERENCES', 'CONSTRAINT', 'DEFAULT', 'AUTO_INCREMENT'
];

export type Language = 
  | 'c' | 'cpp' | 'python' | 'assembly' | 'makefile' | 'linker' | 'markdown'
  | 'javascript' | 'typescript' | 'html' | 'css' | 'xml' | 'json' | 'yaml'
  | 'svelte' | 'astro' | 'dockerfile' | 'gitignore' | 'bash' | 'rust'
  | 'go' | 'java' | 'sql' | 'text';

export function detectLanguage(filename: string): Language {
  const ext = filename.split('.').pop()?.toLowerCase();
  const basename = filename.split('/').pop()?.toLowerCase() || '';
  
  // Check for special filenames without extensions
  if (basename === 'makefile' || basename.startsWith('makefile.')) {
    return 'makefile';
  }
  if (basename === 'dockerfile' || basename.startsWith('dockerfile.')) {
    return 'dockerfile';
  }
  if (basename === '.gitignore' || basename === '.dockerignore' || basename === '.npmignore') {
    return 'gitignore';
  }
  
  switch (ext) {
    // C/C++
    case 'c':
    case 'h':
      return 'c';
    case 'cpp':
    case 'cxx':
    case 'cc':
    case 'hpp':
    case 'hxx':
      return 'cpp';
    
    // Python
    case 'py':
    case 'pyw':
      return 'python';
    
    // Assembly
    case 's':
    case 'S':
    case 'asm':
      return 'assembly';
    
    // Web languages
    case 'js':
    case 'mjs':
    case 'cjs':
      return 'javascript';
    case 'ts':
    case 'mts':
    case 'cts':
      return 'typescript';
    case 'html':
    case 'htm':
      return 'html';
    case 'css':
    case 'scss':
    case 'sass':
    case 'less':
      return 'css';
    case 'xml':
    case 'svg':
      return 'xml';
    case 'json':
    case 'jsonc':
      return 'json';
    case 'yaml':
    case 'yml':
      return 'yaml';
    
    // Framework-specific
    case 'svelte':
      return 'svelte';
    case 'astro':
      return 'astro';
    
    // Other languages
    case 'rs':
      return 'rust';
    case 'go':
      return 'go';
    case 'java':
      return 'java';
    case 'sql':
      return 'sql';
    case 'sh':
    case 'bash':
    case 'zsh':
      return 'bash';
    
    // Build/config files
    case 'ld':
      return 'linker';
    case 'mk':
      return 'makefile';
    case 'md':
    case 'markdown':
      return 'markdown';
    
    default:
      return 'text';
  }
}

export function highlightCode(code: string, language: string): HighlightedToken[] {
  if (!code.trim()) {
    return [{ type: 'text', value: code }];
  }
  
  switch (language) {
    case 'c':
      return highlightC(code);
    case 'cpp':
      return highlightCpp(code);
    case 'python':
      return highlightPython(code);
    case 'assembly':
      return highlightAssembly(code);
    case 'makefile':
      return highlightMakefile(code);
    case 'linker':
      return highlightLinker(code);
    case 'markdown':
      return highlightMarkdown(code);
    case 'javascript':
      return highlightJavaScript(code);
    case 'typescript':
      return highlightTypeScript(code);
    case 'html':
      return highlightHTML(code);
    case 'css':
      return highlightCSS(code);
    case 'xml':
      return highlightXML(code);
    case 'json':
      return highlightJSON(code);
    case 'yaml':
      return highlightYAML(code);
    case 'svelte':
      return highlightSvelte(code);
    case 'astro':
      return highlightAstro(code);
    case 'dockerfile':
      return highlightDockerfile(code);
    case 'gitignore':
      return highlightGitignore(code);
    case 'bash':
      return highlightBash(code);
    case 'rust':
      return highlightRust(code);
    case 'go':
      return highlightGo(code);
    case 'java':
      return highlightJava(code);
    case 'sql':
      return highlightSQL(code);
    default:
      return [{ type: 'text', value: code }];
  }
}

function highlightC(code: string): HighlightedToken[] {
  return highlightCLike(code, C_KEYWORDS);
}

function highlightCpp(code: string): HighlightedToken[] {
  return highlightCLike(code, CPP_KEYWORDS);
}

function highlightCLike(code: string, keywords: string[]): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    // Handle newlines separately from other whitespace
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // Handle other whitespace (spaces, tabs)
    if (/[ \t\r]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      const whitespace = code.slice(start, pos);
      if (whitespace.length > 0) {
        tokens.push({ type: 'text', value: whitespace });
      }
      continue;
    }
    
    // Comments
    if (code.slice(pos, pos + 2) === '//') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    if (code.slice(pos, pos + 2) === '/*') {
      const start = pos;
      pos += 2;
      while (pos < code.length - 1 && code.slice(pos, pos + 2) !== '*/') pos++;
      if (pos < code.length - 1) pos += 2;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Strings
    if (code[pos] === '"' || code[pos] === "'") {
      const quote = code[pos];
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== quote) {
        if (code[pos] === '\\') pos++; // Skip escaped characters
        pos++;
      }
      if (pos < code.length) pos++; // Include closing quote
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Numbers
    if (/\d/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[\d.xabcdefABCDEF]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators
    if (/[+\-*/%=<>!&|^~(){}[\];,.]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    // Identifiers and keywords
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      if (keywords.includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else if (/^[A-Z_][A-Z0-9_]*$/.test(word)) {
        tokens.push({ type: 'type', value: word }); // Constants/macros
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Default: single character
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightPython(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    // Handle newlines separately from other whitespace
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // Handle other whitespace (spaces, tabs)
    if (/[ \t\r]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      const whitespace = code.slice(start, pos);
      if (whitespace.length > 0) {
        tokens.push({ type: 'text', value: whitespace });
      }
      continue;
    }
    
    // Comments
    if (code[pos] === '#') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Strings
    if (code[pos] === '"' || code[pos] === "'") {
      const quote = code[pos];
      const start = pos;
      pos++;
      
      // Handle triple quotes
      if (pos < code.length - 1 && code.slice(pos, pos + 2) === quote + quote) {
        pos += 2;
        while (pos < code.length - 2 && code.slice(pos, pos + 3) !== quote + quote + quote) pos++;
        if (pos < code.length - 2) pos += 3;
      } else {
        while (pos < code.length && code[pos] !== quote) {
          if (code[pos] === '\\') pos++; // Skip escaped characters
          pos++;
        }
        if (pos < code.length) pos++; // Include closing quote
      }
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Numbers
    if (/\d/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[\d.eE+\-]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators
    if (/[+\-*/%=<>!&|^~(){}[\];,.:@]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    // Identifiers and keywords
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      if (PYTHON_KEYWORDS.includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else if (word.endsWith('()') || /^[a-z_][a-z0-9_]*$/.test(word)) {
        tokens.push({ type: 'function', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Default: single character
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightAssembly(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    // Handle newlines separately from other whitespace
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // Handle other whitespace (spaces, tabs)
    if (/[ \t\r]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      const whitespace = code.slice(start, pos);
      if (whitespace.length > 0) {
        tokens.push({ type: 'text', value: whitespace });
      }
      continue;
    }
    
    // Comments (@ or ; or //)
    if (code[pos] === '@' || code[pos] === ';' || code.slice(pos, pos + 2) === '//') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Directives
    if (code[pos] === '.') {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_.]/.test(code[pos])) pos++;
      const directive = code.slice(start, pos);
      if (ARM_DIRECTIVES.includes(directive)) {
        tokens.push({ type: 'directive', value: directive });
      } else {
        tokens.push({ type: 'text', value: directive });
      }
      continue;
    }
    
    // Labels (word followed by colon)
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      // Check if it's followed by a colon (label)
      if (pos < code.length && code[pos] === ':') {
        tokens.push({ type: 'function', value: word });
        tokens.push({ type: 'operator', value: ':' });
        pos++;
        continue;
      }
      
      // Check for registers
      if (ARM_REGISTERS.includes(word.toLowerCase())) {
        tokens.push({ type: 'register', value: word });
      } else if (ARM_INSTRUCTIONS.includes(word.toLowerCase())) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Numbers and hex values
    if (/\d/.test(code[pos]) || (code[pos] === '#' && pos + 1 < code.length && /[0-9x]/.test(code[pos + 1]))) {
      const start = pos;
      if (code[pos] === '#') pos++; // Skip immediate prefix
      if (code.slice(pos, pos + 2) === '0x') pos += 2; // Skip hex prefix
      while (pos < code.length && /[0-9a-fA-F]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators and punctuation
    if (/[+\-*/%=<>!&|^~(){}[\];,.]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    // Default: single character
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}


function highlightMakefile(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  // Makefile keywords and built-in functions
  const MAKEFILE_KEYWORDS = [
    'ifeq', 'ifneq', 'ifdef', 'ifndef', 'else', 'endif',
    'include', 'sinclude', '-include', 'define', 'endef',
    'export', 'unexport', 'override', 'private'
  ];
  
  const MAKEFILE_FUNCTIONS = [
    'subst', 'patsubst', 'strip', 'findstring', 'filter', 'filter-out',
    'sort', 'word', 'wordlist', 'words', 'firstword', 'lastword',
    'dir', 'notdir', 'suffix', 'basename', 'addsuffix', 'addprefix',
    'join', 'wildcard', 'realpath', 'abspath', 'error', 'warning',
    'shell', 'origin', 'flavor', 'foreach', 'if', 'or', 'and', 'call', 'eval'
  ];
  
  while (pos < code.length) {
    // Handle newlines separately
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // Handle other whitespace
    if (/[ \t\r]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Comments
    if (code[pos] === '#') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Variable references $(VAR) or ${VAR}
    if (code[pos] === '$' && pos + 1 < code.length && (code[pos + 1] === '(' || code[pos + 1] === '{')) {
      const start = pos;
      const closer = code[pos + 1] === '(' ? ')' : '}';
      pos += 2;
      
      // Check for function call
      let funcName = '';
      const funcStart = pos;
      while (pos < code.length && /[a-zA-Z_-]/.test(code[pos])) {
        funcName += code[pos];
        pos++;
      }
      
      if (MAKEFILE_FUNCTIONS.includes(funcName)) {
        tokens.push({ type: 'text', value: code.slice(start, funcStart) });
        tokens.push({ type: 'function', value: funcName });
        // Continue to closing bracket
        while (pos < code.length && code[pos] !== closer) pos++;
        if (pos < code.length) {
          tokens.push({ type: 'text', value: code.slice(funcStart + funcName.length, pos + 1) });
          pos++;
        }
      } else {
        // Regular variable
        while (pos < code.length && code[pos] !== closer) pos++;
        if (pos < code.length) pos++;
        tokens.push({ type: 'type', value: code.slice(start, pos) });
      }
      continue;
    }
    
    // Automatic variables ($@, $<, $^, etc.)
    if (code[pos] === '$' && pos + 1 < code.length && /[@<^+?*%|]/.test(code[pos + 1])) {
      tokens.push({ type: 'type', value: code.slice(pos, pos + 2) });
      pos += 2;
      continue;
    }
    
    // Target: dependency pattern (at start of line or after whitespace)
    if (/[a-zA-Z0-9_.\-\/]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_.\-\/]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      // Check if followed by colon (target)
      let checkPos = pos;
      while (checkPos < code.length && /[ \t]/.test(code[checkPos])) checkPos++;
      if (checkPos < code.length && code[checkPos] === ':') {
        tokens.push({ type: 'function', value: word });
        continue;
      }
      
      // Check for keywords
      if (MAKEFILE_KEYWORDS.includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Strings
    if (code[pos] === '"' || code[pos] === "'") {
      const quote = code[pos];
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== quote) {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators and special characters
    if (/[:=+?!]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    // Default
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightLinker(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  // Linker script keywords and commands
  const LINKER_KEYWORDS = [
    'MEMORY', 'SECTIONS', 'ENTRY', 'OUTPUT_FORMAT', 'OUTPUT_ARCH',
    'STARTUP', 'SEARCH_DIR', 'INPUT', 'GROUP', 'AS_NEEDED',
    'PROVIDE', 'PROVIDE_HIDDEN', 'KEEP', 'ALIGN', 'ALIGNOF',
    'SIZEOF', 'ADDR', 'LOADADDR', 'ORIGIN', 'LENGTH',
    'CREATE_OBJECT_SYMBOLS', 'CONSTRUCTORS', 'SORT', 'SORT_BY_NAME',
    'SORT_BY_ALIGNMENT', 'SORT_BY_INIT_PRIORITY', 'COMMON',
    'NOCROSSREFS', 'OUTPUT', 'ASSERT', 'EXTERN', 'FORCE_COMMON_ALLOCATION',
    'INHIBIT_COMMON_ALLOCATION', 'INSERT', 'AFTER', 'BEFORE',
    'INCLUDE', 'PHDRS', 'FILEHDR', 'AT', 'SUBALIGN', 'HIDDEN'
  ];
  
  // Memory region attributes
  const MEMORY_ATTRS = ['r', 'w', 'x', 'a', 'i', 'l', '!'];
  
  while (pos < code.length) {
    // Handle newlines separately
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // Handle other whitespace
    if (/[ \t\r]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Comments (C-style)
    if (code.slice(pos, pos + 2) === '/*') {
      const start = pos;
      pos += 2;
      while (pos < code.length - 1 && code.slice(pos, pos + 2) !== '*/') pos++;
      if (pos < code.length - 1) pos += 2;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Hex numbers (0x...)
    if (code.slice(pos, pos + 2) === '0x' || code.slice(pos, pos + 2) === '0X') {
      const start = pos;
      pos += 2;
      while (pos < code.length && /[0-9a-fA-F]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Numbers
    if (/\d/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[\dKkMm]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Section names (starts with .)
    if (code[pos] === '.' && pos + 1 < code.length && /[a-zA-Z_]/.test(code[pos + 1])) {
      const start = pos;
      pos++;
      while (pos < code.length && /[a-zA-Z0-9_.]/.test(code[pos])) pos++;
      tokens.push({ type: 'directive', value: code.slice(start, pos) });
      continue;
    }
    
    // Identifiers and keywords
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      if (LINKER_KEYWORDS.includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Strings
    if (code[pos] === '"') {
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== '"') {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators and special characters
    if (/[=+\-*/><!&|(){}[\];:,]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    // Default
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightMarkdown(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    // Headers (# ## ###)
    if (line.trimStart().startsWith('#')) {
      const leadingSpace = line.match(/^\s*/)?.[0] || '';
      if (leadingSpace) {
        tokens.push({ type: 'text', value: leadingSpace });
        pos += leadingSpace.length;
      }
      
      const headerMatch = line.slice(pos).match(/^(#{1,6})\s+(.*)$/);
      if (headerMatch) {
        tokens.push({ type: 'keyword', value: headerMatch[1] + ' ' });
        tokens.push({ type: 'function', value: headerMatch[2] });
        if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
        continue;
      }
    }
    
    // Code blocks (```)
    if (line.trimStart().startsWith('```')) {
      tokens.push({ type: 'directive', value: line });
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // Lists (- * +)
    const listMatch = line.match(/^(\s*)([-*+])\s+(.*)$/);
    if (listMatch) {
      tokens.push({ type: 'text', value: listMatch[1] });
      tokens.push({ type: 'operator', value: listMatch[2] + ' ' });
      tokens.push({ type: 'text', value: listMatch[3] });
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // Numbered lists (1. 2. etc)
    const numberedListMatch = line.match(/^(\s*)(\d+\.)\s+(.*)$/);
    if (numberedListMatch) {
      tokens.push({ type: 'text', value: numberedListMatch[1] });
      tokens.push({ type: 'number', value: numberedListMatch[2] + ' ' });
      tokens.push({ type: 'text', value: numberedListMatch[3] });
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // Process inline markdown
    while (pos < line.length) {
      // Bold (**text**)
      if (line.slice(pos, pos + 2) === '**') {
        const endPos = line.indexOf('**', pos + 2);
        if (endPos !== -1) {
          tokens.push({ type: 'keyword', value: line.slice(pos, endPos + 2) });
          pos = endPos + 2;
          continue;
        }
      }
      
      // Italic (*text* or _text_)
      if (line[pos] === '*' || line[pos] === '_') {
        const char = line[pos];
        const endPos = line.indexOf(char, pos + 1);
        if (endPos !== -1 && endPos !== pos + 1) {
          tokens.push({ type: 'type', value: line.slice(pos, endPos + 1) });
          pos = endPos + 1;
          continue;
        }
      }
      
      // Inline code (`code`)
      if (line[pos] === '`') {
        const endPos = line.indexOf('`', pos + 1);
        if (endPos !== -1) {
          tokens.push({ type: 'string', value: line.slice(pos, endPos + 1) });
          pos = endPos + 1;
          continue;
        }
      }
      
      // Links [text](url)
      if (line[pos] === '[') {
        const closeBracket = line.indexOf(']', pos + 1);
        if (closeBracket !== -1 && line[closeBracket + 1] === '(') {
          const closeParen = line.indexOf(')', closeBracket + 2);
          if (closeParen !== -1) {
            tokens.push({ type: 'function', value: line.slice(pos, closeParen + 1) });
            pos = closeParen + 1;
            continue;
          }
        }
      }
      
      // Default
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    if (i < lines.length - 1) {
      tokens.push({ type: 'text', value: '\n' });
    }
  }
  
  return tokens;
}


function highlightJavaScript(code: string): HighlightedToken[] {
  return highlightCLike(code, JS_KEYWORDS);
}

function highlightTypeScript(code: string): HighlightedToken[] {
  return highlightCLike(code, TS_KEYWORDS);
}

function highlightHTML(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    // HTML comments
    if (code.slice(pos, pos + 4) === '<!--') {
      const end = code.indexOf('-->', pos + 4);
      if (end !== -1) {
        tokens.push({ type: 'comment', value: code.slice(pos, end + 3) });
        pos = end + 3;
      } else {
        tokens.push({ type: 'comment', value: code.slice(pos) });
        break;
      }
      continue;
    }
    
    // Tags
    if (code[pos] === '<') {
      const tagEnd = code.indexOf('>', pos);
      if (tagEnd !== -1) {
        const tagContent = code.slice(pos + 1, tagEnd);
        tokens.push({ type: 'operator', value: '<' });
        
        // Parse tag name
        const spaceIdx = tagContent.search(/[\s/>]/);
        const tagName = spaceIdx !== -1 ? tagContent.slice(0, spaceIdx) : tagContent;
        const cleanTagName = tagName.replace('/', '');
        
        if (tagName.startsWith('/')) {
          tokens.push({ type: 'operator', value: '/' });
        }
        
        if (HTML_TAGS.includes(cleanTagName.toLowerCase())) {
          tokens.push({ type: 'keyword', value: cleanTagName });
        } else {
          tokens.push({ type: 'function', value: cleanTagName });
        }
        
        // Parse attributes
        if (spaceIdx !== -1) {
          const attrs = tagContent.slice(spaceIdx);
          let attrPos = 0;
          while (attrPos < attrs.length) {
            if (/\s/.test(attrs[attrPos])) {
              tokens.push({ type: 'text', value: attrs[attrPos] });
              attrPos++;
              continue;
            }
            
            if (attrs[attrPos] === '/' || attrs[attrPos] === '>') {
              attrPos++;
              continue;
            }
            
            // Attribute name
            const attrStart = attrPos;
            while (attrPos < attrs.length && /[a-zA-Z0-9\-:]/.test(attrs[attrPos])) attrPos++;
            if (attrPos > attrStart) {
              tokens.push({ type: 'type', value: attrs.slice(attrStart, attrPos) });
            }
            
            // Skip whitespace
            while (attrPos < attrs.length && /\s/.test(attrs[attrPos])) {
              tokens.push({ type: 'text', value: attrs[attrPos] });
              attrPos++;
            }
            
            // Equals sign
            if (attrPos < attrs.length && attrs[attrPos] === '=') {
              tokens.push({ type: 'operator', value: '=' });
              attrPos++;
              
              // Skip whitespace
              while (attrPos < attrs.length && /\s/.test(attrs[attrPos])) {
                tokens.push({ type: 'text', value: attrs[attrPos] });
                attrPos++;
              }
              
              // Attribute value
              if (attrPos < attrs.length && (attrs[attrPos] === '"' || attrs[attrPos] === "'")) {
                const quote = attrs[attrPos];
                const valueStart = attrPos;
                attrPos++;
                while (attrPos < attrs.length && attrs[attrPos] !== quote) attrPos++;
                if (attrPos < attrs.length) attrPos++;
                tokens.push({ type: 'string', value: attrs.slice(valueStart, attrPos) });
              }
            }
          }
        }
        
        if (tagContent.endsWith('/')) {
          tokens.push({ type: 'operator', value: '/' });
        }
        
        tokens.push({ type: 'operator', value: '>' });
        pos = tagEnd + 1;
        continue;
      }
    }
    
    // Text content
    const textStart = pos;
    while (pos < code.length && code[pos] !== '<' && code[pos] !== '\n') pos++;
    if (pos > textStart) {
      tokens.push({ type: 'text', value: code.slice(textStart, pos) });
    }
  }
  
  return tokens;
}

function highlightXML(code: string): HighlightedToken[] {
  return highlightHTML(code); // XML uses similar syntax to HTML
}

function highlightCSS(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Comments
    if (code.slice(pos, pos + 2) === '/*') {
      const end = code.indexOf('*/', pos + 2);
      if (end !== -1) {
        tokens.push({ type: 'comment', value: code.slice(pos, end + 2) });
        pos = end + 2;
      } else {
        tokens.push({ type: 'comment', value: code.slice(pos) });
        break;
      }
      continue;
    }
    
    // Strings
    if (code[pos] === '"' || code[pos] === "'") {
      const quote = code[pos];
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== quote) {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Selectors (before {)
    if (code[pos] === '.' || code[pos] === '#') {
      const start = pos;
      pos++;
      while (pos < code.length && /[a-zA-Z0-9_-]/.test(code[pos])) pos++;
      tokens.push({ type: 'function', value: code.slice(start, pos) });
      continue;
    }
    
    // Property names (before :)
    if (/[a-zA-Z-]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9-]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      // Check if it's a property
      const nextNonSpace = code.slice(pos).search(/\S/);
      if (nextNonSpace !== -1 && code[pos + nextNonSpace] === ':') {
        tokens.push({ type: 'type', value: word });
      } else {
        tokens.push({ type: 'keyword', value: word });
      }
      continue;
    }
    
    // Numbers and units
    if (/\d/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[\d.]/.test(code[pos])) pos++;
      // Check for units
      if (pos < code.length && /[a-z%]/.test(code[pos])) {
        while (pos < code.length && /[a-z%]/.test(code[pos])) pos++;
      }
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Operators and punctuation
    if (/[{}:;,()[\]>+~*]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightJSON(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Strings (keys and values)
    if (code[pos] === '"') {
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== '"') {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      
      // Check if it's a key (followed by :)
      const nextNonSpace = code.slice(pos).search(/\S/);
      if (nextNonSpace !== -1 && code[pos + nextNonSpace] === ':') {
        tokens.push({ type: 'type', value: code.slice(start, pos) });
      } else {
        tokens.push({ type: 'string', value: code.slice(start, pos) });
      }
      continue;
    }
    
    // Numbers
    if (/[\d-]/.test(code[pos])) {
      const start = pos;
      if (code[pos] === '-') pos++;
      while (pos < code.length && /[\d.eE+\-]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Keywords (true, false, null)
    if (/[a-z]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-z]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      if (['true', 'false', 'null'].includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Operators
    if (/[{}[\]:,]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightYAML(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    // Leading whitespace
    while (pos < line.length && /[ \t]/.test(line[pos])) {
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    // Comments
    if (line[pos] === '#') {
      tokens.push({ type: 'comment', value: line.slice(pos) });
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // List items
    if (line[pos] === '-' && (pos + 1 >= line.length || /\s/.test(line[pos + 1]))) {
      tokens.push({ type: 'operator', value: '-' });
      pos++;
      if (pos < line.length) {
        tokens.push({ type: 'text', value: line.slice(pos) });
      }
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // Key-value pairs
    const colonIdx = line.indexOf(':', pos);
    if (colonIdx !== -1) {
      const key = line.slice(pos, colonIdx).trim();
      if (key) {
        tokens.push({ type: 'type', value: line.slice(pos, colonIdx) });
        tokens.push({ type: 'operator', value: ':' });
        
        const value = line.slice(colonIdx + 1);
        if (value.trim()) {
          // Check for strings
          if (value.trim().startsWith('"') || value.trim().startsWith("'")) {
            tokens.push({ type: 'string', value });
          } else if (/^\s*[\d.-]/.test(value)) {
            tokens.push({ type: 'number', value });
          } else if (/^\s*(true|false|null|yes|no|on|off)\s*$/.test(value)) {
            tokens.push({ type: 'keyword', value });
          } else {
            tokens.push({ type: 'text', value });
          }
        }
      } else {
        tokens.push({ type: 'text', value: line.slice(pos) });
      }
    } else {
      tokens.push({ type: 'text', value: line.slice(pos) });
    }
    
    if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
  }
  
  return tokens;
}

function highlightSvelte(code: string): HighlightedToken[] {
  // Svelte is HTML + JS/TS, use HTML highlighter as base
  return highlightHTML(code);
}

function highlightAstro(code: string): HighlightedToken[] {
  // Astro is similar to HTML with frontmatter
  return highlightHTML(code);
}

function highlightDockerfile(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  const DOCKER_KEYWORDS = [
    'FROM', 'RUN', 'CMD', 'LABEL', 'EXPOSE', 'ENV', 'ADD', 'COPY',
    'ENTRYPOINT', 'VOLUME', 'USER', 'WORKDIR', 'ARG', 'ONBUILD',
    'STOPSIGNAL', 'HEALTHCHECK', 'SHELL', 'MAINTAINER'
  ];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    // Leading whitespace
    while (pos < line.length && /\s/.test(line[pos])) {
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    // Comments
    if (line[pos] === '#') {
      tokens.push({ type: 'comment', value: line.slice(pos) });
      if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
      continue;
    }
    
    // Docker keywords
    const wordMatch = line.slice(pos).match(/^[A-Z]+/);
    if (wordMatch && DOCKER_KEYWORDS.includes(wordMatch[0])) {
      tokens.push({ type: 'keyword', value: wordMatch[0] });
      pos += wordMatch[0].length;
      tokens.push({ type: 'text', value: line.slice(pos) });
    } else {
      tokens.push({ type: 'text', value: line.slice(pos) });
    }
    
    if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
  }
  
  return tokens;
}

function highlightGitignore(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    
    if (line.trim().startsWith('#')) {
      tokens.push({ type: 'comment', value: line });
    } else if (line.trim().startsWith('!')) {
      tokens.push({ type: 'keyword', value: line });
    } else if (line.trim()) {
      tokens.push({ type: 'string', value: line });
    } else {
      tokens.push({ type: 'text', value: line });
    }
    
    if (i < lines.length - 1) tokens.push({ type: 'text', value: '\n' });
  }
  
  return tokens;
}

function highlightBash(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  const BASH_KEYWORDS = [
    'if', 'then', 'else', 'elif', 'fi', 'case', 'esac', 'for', 'while',
    'until', 'do', 'done', 'in', 'function', 'select', 'time', 'coproc',
    'echo', 'export', 'source', 'alias', 'unalias', 'set', 'unset',
    'readonly', 'local', 'declare', 'typeset', 'return', 'exit', 'break',
    'continue', 'shift', 'test', 'true', 'false'
  ];
  
  while (pos < code.length) {
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Comments
    if (code[pos] === '#') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Strings
    if (code[pos] === '"' || code[pos] === "'" || code[pos] === '`') {
      const quote = code[pos];
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== quote) {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Variables
    if (code[pos] === '$') {
      const start = pos;
      pos++;
      if (code[pos] === '{') {
        pos++;
        while (pos < code.length && code[pos] !== '}') pos++;
        if (pos < code.length) pos++;
      } else {
        while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      }
      tokens.push({ type: 'type', value: code.slice(start, pos) });
      continue;
    }
    
    // Keywords
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      if (BASH_KEYWORDS.includes(word)) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Operators
    if (/[|&;<>()[\]{}!]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}

function highlightRust(code: string): HighlightedToken[] {
  return highlightCLike(code, RUST_KEYWORDS);
}

function highlightGo(code: string): HighlightedToken[] {
  return highlightCLike(code, GO_KEYWORDS);
}

function highlightJava(code: string): HighlightedToken[] {
  return highlightCLike(code, JAVA_KEYWORDS);
}

function highlightSQL(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  let pos = 0;
  
  while (pos < code.length) {
    if (code[pos] === '\n') {
      tokens.push({ type: 'text', value: '\n' });
      pos++;
      continue;
    }
    
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[ \t\r]/.test(code[pos])) pos++;
      tokens.push({ type: 'text', value: code.slice(start, pos) });
      continue;
    }
    
    // Comments
    if (code.slice(pos, pos + 2) === '--') {
      const start = pos;
      while (pos < code.length && code[pos] !== '\n') pos++;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    if (code.slice(pos, pos + 2) === '/*') {
      const start = pos;
      pos += 2;
      while (pos < code.length - 1 && code.slice(pos, pos + 2) !== '*/') pos++;
      if (pos < code.length - 1) pos += 2;
      tokens.push({ type: 'comment', value: code.slice(start, pos) });
      continue;
    }
    
    // Strings
    if (code[pos] === "'" || code[pos] === '"') {
      const quote = code[pos];
      const start = pos;
      pos++;
      while (pos < code.length && code[pos] !== quote) {
        if (code[pos] === '\\') pos++;
        pos++;
      }
      if (pos < code.length) pos++;
      tokens.push({ type: 'string', value: code.slice(start, pos) });
      continue;
    }
    
    // Numbers
    if (/\d/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[\d.]/.test(code[pos])) pos++;
      tokens.push({ type: 'number', value: code.slice(start, pos) });
      continue;
    }
    
    // Keywords
    if (/[a-zA-Z_]/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /[a-zA-Z0-9_]/.test(code[pos])) pos++;
      const word = code.slice(start, pos);
      
      if (SQL_KEYWORDS.includes(word.toUpperCase())) {
        tokens.push({ type: 'keyword', value: word });
      } else {
        tokens.push({ type: 'text', value: word });
      }
      continue;
    }
    
    // Operators
    if (/[=<>!+\-*/%(),.;]/.test(code[pos])) {
      tokens.push({ type: 'operator', value: code[pos] });
      pos++;
      continue;
    }
    
    tokens.push({ type: 'text', value: code[pos] });
    pos++;
  }
  
  return tokens;
}
