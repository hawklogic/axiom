// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Console store - manages backend log messages for the mini-console.
 */

import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
  level: LogLevel;
  message: string;
  source: string;
  timestamp: number;
}

const MAX_ENTRIES = 500;

/** Check if running inside Tauri (works with Tauri 2.x) */
function isTauri(): boolean {
  if (!browser || typeof window === 'undefined') return false;
  // Tauri 2.x uses __TAURI_INTERNALS__, Tauri 1.x uses __TAURI__
  return '__TAURI__' in window || '__TAURI_INTERNALS__' in window;
}

function createConsoleStore() {
  const entries = writable<LogEntry[]>([]);
  const filter = writable<LogLevel | 'all'>('all');
  const initialized = writable(false);

  // Filtered entries based on current filter
  const filteredEntries = derived(
    [entries, filter],
    ([$entries, $filter]) => {
      if ($filter === 'all') return $entries;
      return $entries.filter(e => e.level === $filter);
    }
  );

  return {
    entries,
    filter,
    filteredEntries,

    /**
     * Initialize the console store and start listening for backend events.
     */
    async init() {
      console.log('[Console] init() called');
      if (!browser) {
        console.log('[Console] Not in browser, skipping');
        return;
      }

      // Check if already initialized
      let isInit = false;
      initialized.subscribe(v => isInit = v)();
      if (isInit) {
        console.log('[Console] Already initialized, skipping');
        return;
      }

      // Check if running in Tauri
      if (!isTauri()) {
        console.log('[Console] Not in Tauri, showing warning');
        // Not in Tauri, just show a message
        entries.update(e => [...e, {
          level: 'warn',
          message: 'Backend console requires Tauri runtime',
          source: 'frontend',
          timestamp: Date.now(),
        }]);
        initialized.set(true);
        return;
      }

      console.log('[Console] Tauri detected, setting up event listener...');
      try {
        // Dynamically import Tauri event API
        const { listen } = await import('@tauri-apps/api/event');
        console.log('[Console] Tauri event API imported');
        
        // Listen for backend log events
        await listen<LogEntry>('backend-log', (event) => {
          entries.update(current => {
            const updated = [...current, event.payload];
            // Keep only the last MAX_ENTRIES
            if (updated.length > MAX_ENTRIES) {
              return updated.slice(-MAX_ENTRIES);
            }
            return updated;
          });
        });

        console.log('[Console] Event listener registered');
        initialized.set(true);

        // Add initial entry
        entries.update(e => [...e, {
          level: 'info',
          message: 'Console initialized',
          source: 'frontend',
          timestamp: Date.now(),
        }]);
      } catch (err) {
        console.error('[Console] Failed to initialize:', err);
      }
    },

    /**
     * Add a frontend-side log entry.
     */
    log(level: LogLevel, source: string, message: string) {
      entries.update(current => {
        const updated = [...current, {
          level,
          message,
          source,
          timestamp: Date.now(),
        }];
        if (updated.length > MAX_ENTRIES) {
          return updated.slice(-MAX_ENTRIES);
        }
        return updated;
      });
    },

    /**
     * Clear all log entries.
     */
    clear() {
      entries.set([]);
    },

    /**
     * Set the log level filter.
     */
    setFilter(level: LogLevel | 'all') {
      filter.set(level);
    },
  };
}

export const consoleStore = createConsoleStore();
