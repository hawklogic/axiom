// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Editor store - manages open files and editor state.
 */

import { writable, derived } from 'svelte/store';
import type { Language } from '$lib/utils/syntax';

export interface OpenFile {
  path: string;
  name: string;
  content: string;
  language: Language;
  modified: boolean;
  cursor: { line: number; column: number };
  type?: 'file' | 'diff';
  diffContext?: {
    repoPath: string;
    filePath: string;
    commitId?: string; // If set, show diff for this commit vs its parent
  };
}

function createEditorStore() {
  const files = writable<OpenFile[]>([]);
  const activeIndex = writable<number>(-1);

  const activeFile = derived(
    [files, activeIndex],
    ([$files, $activeIndex]) => $activeIndex >= 0 ? $files[$activeIndex] : null
  );

  return {
    files,
    activeIndex,
    activeFile,

    openFile(file: OpenFile) {
      files.update(f => {
        const existing = f.findIndex(x => x.path === file.path);
        if (existing >= 0) {
          activeIndex.set(existing);
          return f;
        }
        f.push(file);
        activeIndex.set(f.length - 1);
        return f;
      });
    },

    closeFile(path: string) {
      files.update(f => {
        const idx = f.findIndex(x => x.path === path);
        if (idx >= 0) {
          f.splice(idx, 1);
          activeIndex.update(i => {
            if (i >= f.length) return f.length - 1;
            return i;
          });
        }
        return f;
      });
    },

    updateContent(path: string, content: string) {
      files.update(f => {
        const file = f.find(x => x.path === path);
        if (file) {
          file.content = content;
          file.modified = true;
        }
        return f;
      });
    },

    markSaved(path: string) {
      files.update(f => {
        const file = f.find(x => x.path === path);
        if (file) {
          file.modified = false;
        }
        return f;
      });
    },

    updateCursor(path: string, line: number, column: number) {
      files.update(f => {
        const file = f.find(x => x.path === path);
        if (file) {
          file.cursor = { line, column };
        }
        return f;
      });
    },
  };
}

export const editorStore = createEditorStore();
