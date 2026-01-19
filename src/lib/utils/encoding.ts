// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Simple encoding detection utility
 */

export function detectEncoding(content: string): string {
  // Check for BOM markers
  if (content.charCodeAt(0) === 0xFEFF) {
    return 'UTF-8 with BOM';
  }
  
  // Check for non-ASCII characters
  let hasNonAscii = false;
  for (let i = 0; i < Math.min(content.length, 1000); i++) {
    const code = content.charCodeAt(i);
    if (code > 127) {
      hasNonAscii = true;
      break;
    }
  }
  
  // If all ASCII, it's ASCII
  if (!hasNonAscii) {
    return 'ASCII';
  }
  
  // Otherwise assume UTF-8 (most common)
  return 'UTF-8';
}
