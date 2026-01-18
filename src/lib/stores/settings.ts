// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Settings store - manages application settings.
 */

import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Settings {
  version: number;
  toolchains: {
    clang_path: string | null;
    gcc_path: string | null;
    arm_gcc_path: string | null;
    auto_detect: boolean;
  };
  build: {
    output_dir: string;
    optimization_level: number;
    debug_symbols: boolean;
  };
  editor: {
    font_size: number;
    tab_size: number;
    font_family: string;
    line_numbers: boolean;
    word_wrap: boolean;
    autocomplete: boolean;
  };
  assembly: {
    syntax: 'intel' | 'att';
    architecture: string | null;
  };
  debug: {
    probe_type: string | null;
    reset_on_connect: boolean;
  };
  ui: {
    theme: 'dark' | 'light';
    font_size: number;
  };
}

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings | null>(null);

  return {
    subscribe,
    
    async load() {
      try {
        const settings = await invoke<Settings>('get_settings');
        set(settings);
        return settings;
      } catch (e) {
        console.error('Failed to load settings:', e);
        return null;
      }
    },

    async save(settings: Settings) {
      try {
        await invoke('set_settings', { settings });
        set(settings);
      } catch (e) {
        console.error('Failed to save settings:', e);
        throw e;
      }
    },

    async reset() {
      try {
        const settings = await invoke<Settings>('reset_settings');
        set(settings);
        return settings;
      } catch (e) {
        console.error('Failed to reset settings:', e);
        throw e;
      }
    },

    updateTheme(theme: 'dark' | 'light') {
      update(s => {
        if (s) {
          s.ui.theme = theme;
        }
        return s;
      });
    },
  };
}

export const settingsStore = createSettingsStore();
