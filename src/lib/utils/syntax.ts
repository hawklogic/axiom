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

export function detectLanguage(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase();
  switch (ext) {
    case 'c':
    case 'h':
      return 'c';
    case 'cpp':
    case 'cxx':
    case 'cc':
    case 'hpp':
    case 'hxx':
      return 'cpp';
    case 'py':
    case 'pyw':
      return 'python';
    case 's':
    case 'S':
    case 'asm':
      return 'assembly';
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
    // Skip whitespace but preserve it
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /\s/.test(code[pos])) pos++;
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
    // Skip whitespace but preserve it
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /\s/.test(code[pos])) pos++;
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
    // Skip whitespace but preserve it
    if (/\s/.test(code[pos])) {
      const start = pos;
      while (pos < code.length && /\s/.test(code[pos])) pos++;
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