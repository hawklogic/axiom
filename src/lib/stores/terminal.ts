// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Terminal store - manages terminal sessions.
 */

import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface TerminalSession {
  id: number;
  title: string;
}

function createTerminalStore() {
  const sessions = writable<TerminalSession[]>([]);
  const activeId = writable<number | null>(null);

  return {
    sessions,
    activeId,

    async create() {
      try {
        const id = await invoke<number>('terminal_create');
        sessions.update(s => [...s, { id, title: `Terminal ${id}` }]);
        activeId.set(id);
        return id;
      } catch (e) {
        console.error('Failed to create terminal:', e);
        throw e;
      }
    },

    async write(id: number, data: string) {
      await invoke('terminal_write', { id, data });
    },

    async resize(id: number, rows: number, cols: number) {
      await invoke('terminal_resize', { id, rows, cols });
    },

    async close(id: number) {
      await invoke('terminal_close', { id });
      sessions.update(s => s.filter(x => x.id !== id));
      activeId.update(current => current === id ? null : current);
    },
  };
}

export const terminalStore = createTerminalStore();
