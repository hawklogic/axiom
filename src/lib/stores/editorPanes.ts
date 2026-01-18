// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Editor panes store - manages split view layout and file distribution
 */

import { writable, derived } from 'svelte/store';
import type { OpenFile } from './editor';

export type SplitDirection = 'horizontal' | 'vertical' | null;

export interface EditorPane {
  id: string;
  files: OpenFile[];
  activeIndex: number;
}

export interface PaneLayout {
  panes: EditorPane[];
  splitDirection: SplitDirection;
}

function createEditorPanesStore() {
  const { subscribe, set, update } = writable<PaneLayout>({
    panes: [
      {
        id: 'pane-0',
        files: [],
        activeIndex: -1,
      }
    ],
    splitDirection: null,
  });

  return {
    subscribe,

    /**
     * Open a file in a specific pane
     */
    openFile(paneId: string, file: OpenFile) {
      update(layout => {
        const pane = layout.panes.find(p => p.id === paneId);
        if (!pane) return layout;

        const existing = pane.files.findIndex(f => f.path === file.path);
        if (existing >= 0) {
          pane.activeIndex = existing;
        } else {
          pane.files.push(file);
          pane.activeIndex = pane.files.length - 1;
        }

        return { ...layout };
      });
    },

    /**
     * Close a file in a specific pane
     */
    closeFile(paneId: string, filePath: string) {
      update(layout => {
        const pane = layout.panes.find(p => p.id === paneId);
        if (!pane) return layout;

        const idx = pane.files.findIndex(f => f.path === filePath);
        if (idx >= 0) {
          pane.files.splice(idx, 1);
          if (pane.activeIndex >= pane.files.length) {
            pane.activeIndex = pane.files.length - 1;
          }
        }

        return { ...layout };
      });
    },

    /**
     * Set active file in a pane
     */
    setActiveFile(paneId: string, index: number) {
      update(layout => {
        const pane = layout.panes.find(p => p.id === paneId);
        if (pane && index >= 0 && index < pane.files.length) {
          pane.activeIndex = index;
        }
        return { ...layout };
      });
    },

    /**
     * Update file content
     */
    updateContent(filePath: string, content: string) {
      update(layout => {
        for (const pane of layout.panes) {
          const file = pane.files.find(f => f.path === filePath);
          if (file) {
            file.content = content;
            file.modified = true;
          }
        }
        return { ...layout };
      });
    },

    /**
     * Mark file as saved
     */
    markSaved(filePath: string) {
      update(layout => {
        for (const pane of layout.panes) {
          const file = pane.files.find(f => f.path === filePath);
          if (file) {
            file.modified = false;
          }
        }
        return { ...layout };
      });
    },

    /**
     * Split a pane horizontally or vertically
     */
    splitPane(direction: 'horizontal' | 'vertical') {
      update(layout => {
        if (layout.panes.length >= 4) return layout; // Max 4 panes

        const newPane: EditorPane = {
          id: `pane-${Date.now()}`,
          files: [],
          activeIndex: -1,
        };

        layout.panes.push(newPane);
        layout.splitDirection = direction;

        return { ...layout };
      });
    },

    /**
     * Move a file from one pane to another
     */
    moveFile(fromPaneId: string, toPaneId: string, filePath: string) {
      update(layout => {
        const fromPane = layout.panes.find(p => p.id === fromPaneId);
        const toPane = layout.panes.find(p => p.id === toPaneId);
        
        if (!fromPane || !toPane || fromPaneId === toPaneId) return layout;

        const fileIdx = fromPane.files.findIndex(f => f.path === filePath);
        if (fileIdx < 0) return layout;

        const file = fromPane.files[fileIdx];
        fromPane.files.splice(fileIdx, 1);
        
        // Adjust active index in source pane
        if (fromPane.activeIndex >= fromPane.files.length) {
          fromPane.activeIndex = fromPane.files.length - 1;
        }

        // Add to target pane
        const existingIdx = toPane.files.findIndex(f => f.path === file.path);
        if (existingIdx >= 0) {
          toPane.activeIndex = existingIdx;
        } else {
          toPane.files.push(file);
          toPane.activeIndex = toPane.files.length - 1;
        }

        return { ...layout };
      });
    },

    /**
     * Close a pane (merge its files into another pane)
     */
    closePane(paneId: string) {
      update(layout => {
        if (layout.panes.length <= 1) return layout;

        const paneIdx = layout.panes.findIndex(p => p.id === paneId);
        if (paneIdx < 0) return layout;

        const pane = layout.panes[paneIdx];
        const targetPane = layout.panes[paneIdx === 0 ? 1 : 0];

        // Move all files to target pane
        for (const file of pane.files) {
          if (!targetPane.files.find(f => f.path === file.path)) {
            targetPane.files.push(file);
          }
        }

        layout.panes.splice(paneIdx, 1);

        // Reset split if only one pane left
        if (layout.panes.length === 1) {
          layout.splitDirection = null;
        }

        return { ...layout };
      });
    },

    /**
     * Reset to single pane
     */
    reset() {
      set({
        panes: [
          {
            id: 'pane-0',
            files: [],
            activeIndex: -1,
          }
        ],
        splitDirection: null,
      });
    },
  };
}

export const editorPanes = createEditorPanesStore();
