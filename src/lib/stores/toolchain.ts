// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * Toolchain store - manages detected toolchains.
 */

import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface DetectedToolchain {
  kind: 'Clang' | 'Gcc' | 'ArmGcc' | 'Python';
  path: string;
  version: string;
  bundled: boolean;
}

function createToolchainStore() {
  const toolchains = writable<DetectedToolchain[]>([]);
  const loading = writable(false);

  return {
    toolchains,
    loading,

    async detect() {
      loading.set(true);
      try {
        const detected = await invoke<DetectedToolchain[]>('detect_toolchains');
        toolchains.set(detected);
        return detected;
      } catch (e) {
        console.error('Toolchain detection failed:', e);
        return [];
      } finally {
        loading.set(false);
      }
    },

    async get() {
      try {
        const current = await invoke<DetectedToolchain[]>('get_toolchains');
        toolchains.set(current);
        return current;
      } catch (e) {
        console.error('Failed to get toolchains:', e);
        return [];
      }
    },
  };
}

export const toolchainStore = createToolchainStore();
