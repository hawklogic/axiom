// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Terminal store - manages terminal sessions.
 */

import { writable, get } from 'svelte/store';
import { browser } from '$app/environment';

export interface TerminalSession {
  id: number;
  title: string;
}

/** Check if running inside Tauri */
function isTauri(): boolean {
  return browser && typeof window !== 'undefined' && '__TAURI__' in window;
}

/** Lazy import invoke to avoid errors in browser */
async function getInvoke() {
  if (!isTauri()) {
    throw new Error('Not running in Tauri');
  }
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke;
}

function createTerminalStore() {
  const sessions = writable<TerminalSession[]>([]);
  const activeId = writable<number | null>(null);

  return {
    sessions,
    activeId,

    /** Check if Tauri is available */
    isTauriAvailable: isTauri,

    async create(): Promise<number> {
      try {
        const invoke = await getInvoke();
        const id = await invoke<number>('terminal_create');
        sessions.update(s => [...s, { id, title: `Terminal ${id}` }]);
        activeId.set(id);
        return id;
      } catch (e) {
        console.error('Failed to create terminal:', e);
        throw e;
      }
    },

    async write(id: number, data: string): Promise<void> {
      const invoke = await getInvoke();
      await invoke('terminal_write', { id, data });
    },

    async read(id: number): Promise<Uint8Array> {
      const invoke = await getInvoke();
      const bytes = await invoke<number[]>('terminal_read', { id });
      return new Uint8Array(bytes);
    },

    async resize(id: number, rows: number, cols: number): Promise<void> {
      const invoke = await getInvoke();
      await invoke('terminal_resize', { id, rows, cols });
    },

    async close(id: number): Promise<void> {
      const invoke = await getInvoke();
      await invoke('terminal_close', { id });
      sessions.update(s => s.filter(x => x.id !== id));
      activeId.update(current => current === id ? null : current);
    },

    getActive(): number | null {
      return get(activeId);
    },
  };
}

export const terminalStore = createTerminalStore();
