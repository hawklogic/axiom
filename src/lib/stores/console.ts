// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Console store - manages backend log messages for the mini-console.
 */

import { writable, derived } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import { browser } from '$app/environment';

export type LogLevel = 'debug' | 'info' | 'warn' | 'error';

export interface LogEntry {
  level: LogLevel;
  message: string;
  source: string;
  timestamp: number;
}

const MAX_ENTRIES = 500;

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
      if (!browser) return;

      // Check if already initialized
      let isInit = false;
      initialized.subscribe(v => isInit = v)();
      if (isInit) return;

      try {
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

        initialized.set(true);

        // Add initial entry
        entries.update(e => [...e, {
          level: 'info',
          message: 'Console initialized',
          source: 'frontend',
          timestamp: Date.now(),
        }]);
      } catch (err) {
        console.error('Failed to initialize console store:', err);
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
