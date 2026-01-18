// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * IDE status store - tracks current IDE state and operations
 */

import { writable } from 'svelte/store';

export type StatusType = 'ready' | 'loading' | 'saving' | 'building' | 'error';

export interface IDEStatus {
  type: StatusType;
  message: string;
}

function createStatusStore() {
  const { subscribe, set, update } = writable<IDEStatus>({
    type: 'ready',
    message: 'Ready',
  });

  return {
    subscribe,

    /**
     * Set status to ready
     */
    ready() {
      set({ type: 'ready', message: 'Ready' });
    },

    /**
     * Set status to loading
     */
    loading(message: string = 'Loading...') {
      set({ type: 'loading', message });
    },

    /**
     * Set status to saving
     */
    saving(filename?: string) {
      set({ 
        type: 'saving', 
        message: filename ? `Saving ${filename}...` : 'Saving...' 
      });
    },

    /**
     * Set status to building
     */
    building(message: string = 'Building...') {
      set({ type: 'building', message });
    },

    /**
     * Set status to error
     */
    error(message: string) {
      set({ type: 'error', message });
    },

    /**
     * Set custom status
     */
    custom(type: StatusType, message: string) {
      set({ type, message });
    },
  };
}

export const ideStatus = createStatusStore();
