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
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    while (pos < line.length) {
      // Skip whitespace
      if (/\s/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /\s/.test(line[pos])) pos++;
        tokens.push({ type: 'text', value: line.slice(start, pos) });
        continue;
      }
      
      // Comments
      if (line.slice(pos, pos + 2) === '//') {
        tokens.push({ type: 'comment', value: line.slice(pos) });
        break;
      }
      
      if (line.slice(pos, pos + 2) === '/*') {
        const start = pos;
        pos += 2;
        while (pos < line.length - 1 && line.slice(pos, pos + 2) !== '*/') pos++;
        if (pos < line.length - 1) pos += 2;
        tokens.push({ type: 'comment', value: line.slice(start, pos) });
        continue;
      }
      
      // Strings
      if (line[pos] === '"' || line[pos] === "'") {
        const quote = line[pos];
        const start = pos;
        pos++;
        while (pos < line.length && line[pos] !== quote) {
          if (line[pos] === '\\') pos++; // Skip escaped characters
          pos++;
        }
        if (pos < line.length) pos++; // Include closing quote
        tokens.push({ type: 'string', value: line.slice(start, pos) });
        continue;
      }
      
      // Numbers
      if (/\d/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /[\d.xabcdefABCDEF]/.test(line[pos])) pos++;
        tokens.push({ type: 'number', value: line.slice(start, pos) });
        continue;
      }
      
      // Operators
      if (/[+\-*/%=<>!&|^~(){}[\];,.]/.test(line[pos])) {
        tokens.push({ type: 'operator', value: line[pos] });
        pos++;
        continue;
      }
      
      // Identifiers and keywords
      if (/[a-zA-Z_]/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /[a-zA-Z0-9_]/.test(line[pos])) pos++;
        const word = line.slice(start, pos);
        
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
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    if (i < lines.length - 1) {
      tokens.push({ type: 'text', value: '\n' });
    }
  }
  
  return tokens;
}

function highlightPython(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    while (pos < line.length) {
      // Skip whitespace
      if (/\s/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /\s/.test(line[pos])) pos++;
        tokens.push({ type: 'text', value: line.slice(start, pos) });
        continue;
      }
      
      // Comments
      if (line[pos] === '#') {
        tokens.push({ type: 'comment', value: line.slice(pos) });
        break;
      }
      
      // Strings
      if (line[pos] === '"' || line[pos] === "'") {
        const quote = line[pos];
        const start = pos;
        pos++;
        
        // Handle triple quotes
        if (pos < line.length - 1 && line.slice(pos, pos + 2) === quote + quote) {
          pos += 2;
          while (pos < line.length - 2 && line.slice(pos, pos + 3) !== quote + quote + quote) pos++;
          if (pos < line.length - 2) pos += 3;
        } else {
          while (pos < line.length && line[pos] !== quote) {
            if (line[pos] === '\\') pos++; // Skip escaped characters
            pos++;
          }
          if (pos < line.length) pos++; // Include closing quote
        }
        tokens.push({ type: 'string', value: line.slice(start, pos) });
        continue;
      }
      
      // Numbers
      if (/\d/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /[\d.eE+\-]/.test(line[pos])) pos++;
        tokens.push({ type: 'number', value: line.slice(start, pos) });
        continue;
      }
      
      // Operators
      if (/[+\-*/%=<>!&|^~(){}[\];,.:@]/.test(line[pos])) {
        tokens.push({ type: 'operator', value: line[pos] });
        pos++;
        continue;
      }
      
      // Identifiers and keywords
      if (/[a-zA-Z_]/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /[a-zA-Z0-9_]/.test(line[pos])) pos++;
        const word = line.slice(start, pos);
        
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
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    if (i < lines.length - 1) {
      tokens.push({ type: 'text', value: '\n' });
    }
  }
  
  return tokens;
}

function highlightAssembly(code: string): HighlightedToken[] {
  const tokens: HighlightedToken[] = [];
  const lines = code.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    let pos = 0;
    
    while (pos < line.length) {
      // Skip whitespace
      if (/\s/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /\s/.test(line[pos])) pos++;
        tokens.push({ type: 'text', value: line.slice(start, pos) });
        continue;
      }
      
      // Comments (@ or ; or //)
      if (line[pos] === '@' || line[pos] === ';' || line.slice(pos, pos + 2) === '//') {
        tokens.push({ type: 'comment', value: line.slice(pos) });
        break;
      }
      
      // Directives
      if (line[pos] === '.') {
        const start = pos;
        while (pos < line.length && /[a-zA-Z0-9_.]/.test(line[pos])) pos++;
        const directive = line.slice(start, pos);
        if (ARM_DIRECTIVES.includes(directive)) {
          tokens.push({ type: 'directive', value: directive });
        } else {
          tokens.push({ type: 'text', value: directive });
        }
        continue;
      }
      
      // Labels (word followed by colon)
      if (/[a-zA-Z_]/.test(line[pos])) {
        const start = pos;
        while (pos < line.length && /[a-zA-Z0-9_]/.test(line[pos])) pos++;
        const word = line.slice(start, pos);
        
        // Check if it's followed by a colon (label)
        if (pos < line.length && line[pos] === ':') {
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
      if (/\d/.test(line[pos]) || (line[pos] === '#' && pos + 1 < line.length && /[0-9x]/.test(line[pos + 1]))) {
        const start = pos;
        if (line[pos] === '#') pos++; // Skip immediate prefix
        if (line.slice(pos, pos + 2) === '0x') pos += 2; // Skip hex prefix
        while (pos < line.length && /[0-9a-fA-F]/.test(line[pos])) pos++;
        tokens.push({ type: 'number', value: line.slice(start, pos) });
        continue;
      }
      
      // Operators and punctuation
      if (/[+\-*/%=<>!&|^~(){}[\];,.]/.test(line[pos])) {
        tokens.push({ type: 'operator', value: line[pos] });
        pos++;
        continue;
      }
      
      // Default: single character
      tokens.push({ type: 'text', value: line[pos] });
      pos++;
    }
    
    if (i < lines.length - 1) {
      tokens.push({ type: 'text', value: '\n' });
    }
  }
  
  return tokens;
}