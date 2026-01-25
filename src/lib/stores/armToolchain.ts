// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems

/**
 * ARM Toolchain store - manages ARM GCC toolchain detection and configuration.
 */

import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// TypeScript interfaces matching the Rust types

export interface ArmToolchainSuite {
  gcc: string;
  gxx: string;
  as_: string;
  ld: string;
  objcopy: string;
  objdump: string;
  size: string;
  gdb: string;
  version: string;
  source: 'Homebrew' | 'STM32CubeIDE' | 'System' | 'Manual';
  completeness: 'Complete' | 'Incomplete';
  missing: string[];
}

export interface ArmMcuConfig {
  cpu: string;
  thumb: boolean;
  fpu: string | null;
  float_abi: 'hard' | 'soft' | 'softfp';
  defines: string[];
  include_paths?: string[];
  linker_script?: string;
}

export interface ArmCompileRequest {
  source: string;
  output: string;
  mcu: ArmMcuConfig;
  include_paths: string[];
  optimization: number;
  debug: boolean;
}

export interface CompileResult {
  exit_code: number;
  stdout: string;
  stderr: string;
  diagnostics: Diagnostic[];
}

export interface Diagnostic {
  file: string;
  line: number;
  column: number;
  severity: 'error' | 'warning' | 'info';
  message: string;
}

export interface ArmLinkRequest {
  objects: string[];
  output: string;
  linker_script: string;
  generate_map: boolean;
  map_path: string | null;
  mcu: ArmMcuConfig;
}

export interface LinkResult {
  exit_code: number;
  stdout: string;
  stderr: string;
  diagnostics: Diagnostic[];
}

export interface BinaryOutputConfig {
  hex: boolean;
  bin: boolean;
  size_report: boolean;
}

export interface BinaryResult {
  hex_path: string | null;
  bin_path: string | null;
  size_stats: SizeStats | null;
}

export interface SizeStats {
  text: number;
  data: number;
  bss: number;
  total: number;
}

export interface MakefileInfo {
  path: string;
  targets: string[];
}

export interface MakeResult {
  exit_code: number;
  stdout: string;
  stderr: string;
}

// Default MCU configurations
export const DEFAULT_MCU_CONFIGS: Record<string, ArmMcuConfig> = {
  'cortex-m0': {
    cpu: 'cortex-m0',
    thumb: true,
    fpu: null,
    float_abi: 'soft',
    defines: [],
    include_paths: [],
  },
  'cortex-m3': {
    cpu: 'cortex-m3',
    thumb: true,
    fpu: null,
    float_abi: 'soft',
    defines: [],
    include_paths: [],
  },
  'cortex-m4': {
    cpu: 'cortex-m4',
    thumb: true,
    fpu: 'fpv4-sp-d16',
    float_abi: 'hard',
    defines: [],
    include_paths: [],
  },
  'cortex-m7': {
    cpu: 'cortex-m7',
    thumb: true,
    fpu: 'fpv5-d16',
    float_abi: 'hard',
    defines: [],
    include_paths: [],
  },
};

function createArmToolchainStore() {
  // Store for detected toolchains
  const toolchains = writable<ArmToolchainSuite[]>([]);
  
  // Store for currently selected toolchain
  const selectedToolchain = writable<ArmToolchainSuite | null>(null);
  
  // Store for MCU configuration
  const mcuConfig = writable<ArmMcuConfig>(DEFAULT_MCU_CONFIGS['cortex-m7']);
  
  // Loading state
  const loading = writable(false);
  
  // Error state
  const error = writable<string | null>(null);

  // Derived store for whether a complete toolchain is selected
  const hasCompleteToolchain = derived(
    selectedToolchain,
    ($selectedToolchain) => $selectedToolchain?.completeness === 'Complete'
  );

  return {
    // Readable stores
    toolchains,
    selectedToolchain,
    mcuConfig,
    loading,
    error,
    hasCompleteToolchain,

    /**
     * Detect all ARM toolchains on the system.
     */
    async detectToolchains(): Promise<ArmToolchainSuite[]> {
      loading.set(true);
      error.set(null);
      
      try {
        const detected = await invoke<ArmToolchainSuite[]>('detect_arm_toolchains_cmd');
        toolchains.set(detected);
        
        // Auto-select the first complete toolchain if none selected
        if (detected.length > 0) {
          const complete = detected.find(t => t.completeness === 'Complete');
          if (complete) {
            selectedToolchain.set(complete);
          } else {
            selectedToolchain.set(detected[0]);
          }
        }
        
        return detected;
      } catch (e) {
        const errorMsg = `Toolchain detection failed: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        return [];
      } finally {
        loading.set(false);
      }
    },

    /**
     * Select a specific toolchain by its GCC path.
     */
    selectToolchain(gccPath: string): void {
      let currentToolchains: ArmToolchainSuite[] = [];
      const unsubscribe = toolchains.subscribe(value => {
        currentToolchains = value;
      });
      unsubscribe();
      
      const toolchain = currentToolchains.find(t => t.gcc === gccPath);
      if (toolchain) {
        selectedToolchain.set(toolchain);
      }
    },

    /**
     * Update the MCU configuration.
     */
    updateMcuConfig(config: Partial<ArmMcuConfig>): void {
      mcuConfig.update(current => ({ ...current, ...config }));
    },

    /**
     * Set MCU configuration to a preset.
     */
    setMcuPreset(preset: keyof typeof DEFAULT_MCU_CONFIGS): void {
      const config = DEFAULT_MCU_CONFIGS[preset];
      if (config) {
        mcuConfig.set({ ...config });
      }
    },

    /**
     * Compile ARM source code.
     */
    async compile(request: ArmCompileRequest): Promise<CompileResult> {
      loading.set(true);
      error.set(null);
      
      try {
        let gccPath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          gccPath = $selectedToolchain?.gcc || null;
        })();
        
        if (!gccPath) {
          throw new Error('No ARM toolchain selected');
        }
        
        const result = await invoke<CompileResult>('compile_arm_cmd', {
          request,
          gccPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Compilation failed: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Link ARM object files.
     */
    async link(request: ArmLinkRequest): Promise<LinkResult> {
      loading.set(true);
      error.set(null);
      
      try {
        let gccPath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          gccPath = $selectedToolchain?.gcc || null;
        })();
        
        if (!gccPath) {
          throw new Error('No ARM toolchain selected');
        }
        
        const result = await invoke<LinkResult>('link_arm_cmd', {
          request,
          gccPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Linking failed: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Generate binary files (HEX, BIN) from ELF.
     */
    async generateBinary(
      elfPath: string,
      config: BinaryOutputConfig
    ): Promise<BinaryResult> {
      loading.set(true);
      error.set(null);
      
      try {
        let objcopyPath: string | null = null;
        let sizePath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          objcopyPath = $selectedToolchain?.objcopy || null;
          sizePath = $selectedToolchain?.size || null;
        })();
        
        if (!objcopyPath || !sizePath) {
          throw new Error('No ARM toolchain selected or toolchain incomplete');
        }
        
        const result = await invoke<BinaryResult>('generate_binary_cmd', {
          elfPath,
          config,
          objcopyPath,
          sizePath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Binary generation failed: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Get preprocessor output for a source file.
     */
    async getPreprocessorOutput(source: string): Promise<string> {
      try {
        let gccPath: string | null = null;
        let mcu: ArmMcuConfig | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          gccPath = $selectedToolchain?.gcc || null;
        })();
        
        mcuConfig.subscribe(($mcuConfig) => {
          mcu = $mcuConfig;
        })();
        
        if (!gccPath || !mcu) {
          throw new Error('No ARM toolchain or MCU configuration');
        }
        
        const result = await invoke<string>('get_preprocessor_output_cmd', {
          source,
          mcu,
          gccPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to get preprocessor output: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Get assembly output for a source file.
     */
    async getAssemblyOutput(source: string, output: string): Promise<string> {
      try {
        let gccPath: string | null = null;
        let mcu: ArmMcuConfig | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          gccPath = $selectedToolchain?.gcc || null;
        })();
        
        mcuConfig.subscribe(($mcuConfig) => {
          mcu = $mcuConfig;
        })();
        
        if (!gccPath || !mcu) {
          throw new Error('No ARM toolchain or MCU configuration');
        }
        
        const result = await invoke<string>('get_assembly_output_cmd', {
          source,
          output,
          mcu,
          gccPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to get assembly output: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Get disassembly of an object file.
     */
    async getDisassembly(objectFile: string): Promise<string> {
      try {
        let objdumpPath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          objdumpPath = $selectedToolchain?.objdump || null;
        })();
        
        if (!objdumpPath) {
          throw new Error('No ARM toolchain selected');
        }
        
        const result = await invoke<string>('get_disassembly_cmd', {
          objectFile,
          objdumpPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to get disassembly: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Get symbol table from an object file.
     */
    async getSymbolTable(objectFile: string): Promise<string> {
      try {
        let objdumpPath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          objdumpPath = $selectedToolchain?.objdump || null;
        })();
        
        if (!objdumpPath) {
          throw new Error('No ARM toolchain selected');
        }
        
        const result = await invoke<string>('get_symbol_table_cmd', {
          objectFile,
          objdumpPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to get symbol table: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Get section headers from an object file.
     */
    async getSectionHeaders(objectFile: string): Promise<string> {
      try {
        let objdumpPath: string | null = null;
        
        selectedToolchain.subscribe(($selectedToolchain) => {
          objdumpPath = $selectedToolchain?.objdump || null;
        })();
        
        if (!objdumpPath) {
          throw new Error('No ARM toolchain selected');
        }
        
        const result = await invoke<string>('get_section_headers_cmd', {
          objectFile,
          objdumpPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to get section headers: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Detect Makefile in a project directory.
     */
    async detectMakefile(projectPath: string): Promise<MakefileInfo | null> {
      try {
        const result = await invoke<MakefileInfo | null>('detect_makefile_cmd', {
          projectPath,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Failed to detect Makefile: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      }
    },

    /**
     * Run make with a specific target.
     */
    async runMake(
      projectPath: string,
      target: string,
      toolchainPrefix?: string
    ): Promise<MakeResult> {
      loading.set(true);
      error.set(null);
      
      try {
        const result = await invoke<MakeResult>('run_make_cmd', {
          projectPath,
          target,
          toolchainPrefix: toolchainPrefix || null,
        });
        
        return result;
      } catch (e) {
        const errorMsg = `Make failed: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },

    /**
     * Clear error state.
     */
    clearError(): void {
      error.set(null);
    },

    /**
     * Save ARM toolchain settings.
     */
    async saveSettings(scope: 'project' | 'global'): Promise<void> {
      loading.set(true);
      error.set(null);

      try {
        // Get current values from stores using get() helper
        let gccPath: string | null = null;
        let currentToolchain: ArmToolchainSuite | null = null;
        let currentMcuConfig: ArmMcuConfig | null = null;

        const unsubscribeToolchain = selectedToolchain.subscribe((value) => {
          currentToolchain = value;
        });
        unsubscribeToolchain();

        const unsubscribeMcu = mcuConfig.subscribe((value) => {
          currentMcuConfig = value;
        });
        unsubscribeMcu();

        gccPath = currentToolchain?.gcc || null;

        if (!currentMcuConfig) {
          throw new Error('No MCU configuration to save');
        }

        // Convert MCU config to the format expected by the backend
        const mcuConfigRequest = {
          cpu: currentMcuConfig.cpu,
          thumb: currentMcuConfig.thumb,
          fpu: currentMcuConfig.fpu,
          float_abi: currentMcuConfig.float_abi,
          defines: currentMcuConfig.defines,
        };

        await invoke('save_arm_toolchain_settings_cmd', {
          gccPath,
          mcuConfig: mcuConfigRequest,
          scope,
        });

        // Show success message (could emit an event or update a success state)
        console.log(`ARM toolchain settings saved to ${scope} configuration`);
      } catch (e) {
        const errorMsg = `Failed to save settings: ${e}`;
        error.set(errorMsg);
        console.error(errorMsg);
        throw e;
      } finally {
        loading.set(false);
      }
    },
  };
}

export const armToolchainStore = createArmToolchainStore();
