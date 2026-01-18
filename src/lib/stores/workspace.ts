// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Workspace store - tracks the currently open folder/workspace.
 */

import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export interface DirEntry {
  name: string;
  path: string;
  is_dir: boolean;
  is_file: boolean;
  size: number | null;
}

export interface TreeNode extends DirEntry {
  children?: TreeNode[];
  expanded?: boolean;
  loading?: boolean;
}

function createWorkspaceStore() {
  const { subscribe, set, update } = writable<{
    path: string | null;
    name: string | null;
    tree: TreeNode[];
  }>({
    path: null,
    name: null,
    tree: [],
  });

  return {
    subscribe,

    /**
     * Open folder picker dialog and load workspace.
     */
    async openFolder(): Promise<boolean> {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Open Workspace',
      });

      if (selected && typeof selected === 'string') {
        await this.loadWorkspace(selected);
        return true;
      }
      return false;
    },

    /**
     * Load a workspace from a given path.
     */
    async loadWorkspace(folderPath: string): Promise<void> {
      const name = folderPath.split('/').pop() || folderPath;
      const entries = await invoke<DirEntry[]>('read_dir', { path: folderPath });
      
      const tree: TreeNode[] = entries.map(entry => ({
        ...entry,
        expanded: false,
        children: entry.is_dir ? undefined : undefined,
      }));

      set({ path: folderPath, name, tree });
    },

    /**
     * Toggle expansion of a directory node.
     */
    async toggleNode(nodePath: string): Promise<void> {
      update(state => {
        const toggleInTree = async (nodes: TreeNode[]): Promise<TreeNode[]> => {
          return Promise.all(nodes.map(async node => {
            if (node.path === nodePath && node.is_dir) {
              if (node.expanded) {
                // Collapse
                return { ...node, expanded: false };
              } else {
                // Expand - load children if not loaded
                if (!node.children) {
                  const entries = await invoke<DirEntry[]>('read_dir', { path: node.path });
                  const children: TreeNode[] = entries.map(entry => ({
                    ...entry,
                    expanded: false,
                  }));
                  return { ...node, expanded: true, children };
                }
                return { ...node, expanded: true };
              }
            }
            if (node.children) {
              return { ...node, children: await toggleInTree(node.children) };
            }
            return node;
          }));
        };

        // We need to handle this synchronously for the update
        // So we'll trigger an async update separately
        return state;
      });

      // Handle async tree update
      const state = await new Promise<{ path: string | null; name: string | null; tree: TreeNode[] }>(resolve => {
        subscribe(s => resolve(s))();
      });

      if (!state.tree.length) return;

      const toggleInTree = async (nodes: TreeNode[]): Promise<TreeNode[]> => {
        return Promise.all(nodes.map(async node => {
          if (node.path === nodePath && node.is_dir) {
            if (node.expanded) {
              return { ...node, expanded: false };
            } else {
              if (!node.children) {
                const entries = await invoke<DirEntry[]>('read_dir', { path: node.path });
                const children: TreeNode[] = entries.map(entry => ({
                  ...entry,
                  expanded: false,
                }));
                return { ...node, expanded: true, children };
              }
              return { ...node, expanded: true };
            }
          }
          if (node.children) {
            return { ...node, children: await toggleInTree(node.children) };
          }
          return node;
        }));
      };

      const newTree = await toggleInTree(state.tree);
      set({ ...state, tree: newTree });
    },

    /**
     * Refresh the current workspace.
     */
    async refresh(): Promise<void> {
      const state = await new Promise<{ path: string | null; name: string | null; tree: TreeNode[] }>(resolve => {
        subscribe(s => resolve(s))();
      });

      if (state.path) {
        await this.loadWorkspace(state.path);
      }
    },

    /**
     * Close the current workspace.
     */
    close(): void {
      set({ path: null, name: null, tree: [] });
    },
  };
}

export const workspace = createWorkspaceStore();

// Derived store for checking if workspace is open
export const hasWorkspace = derived(workspace, $ws => $ws.path !== null);
