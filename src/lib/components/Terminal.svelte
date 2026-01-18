<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2024 HawkLogic Systems -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  import { terminalStore } from '$lib/stores/terminal';

  let terminalContainer: HTMLDivElement;
  let terminal: any = null;
  let fitAddon: any = null;
  let sessionId: number | null = null;
  let readInterval: ReturnType<typeof setInterval> | null = null;
  let resizeObserver: ResizeObserver | null = null;

  onMount(async () => {
    if (!browser) return;

    // Dynamically import xterm.js (browser-only)
    const { Terminal } = await import('@xterm/xterm');
    const { FitAddon } = await import('@xterm/addon-fit');
    await import('@xterm/xterm/css/xterm.css');

    // Initialize xterm.js
    terminal = new Terminal({
      fontFamily: '"JetBrains Mono", "SF Mono", Monaco, Menlo, monospace',
      fontSize: 13,
      lineHeight: 1.2,
      cursorBlink: true,
      cursorStyle: 'block',
      theme: {
        background: '#1a1a1a',
        foreground: '#e0e0e0',
        cursor: '#00d4ff',
        cursorAccent: '#1a1a1a',
        selectionBackground: 'rgba(0, 212, 255, 0.3)',
        black: '#1a1a1a',
        red: '#ff5f56',
        green: '#27c93f',
        yellow: '#f5a623',
        blue: '#00d4ff',
        magenta: '#bd93f9',
        cyan: '#00d4ff',
        white: '#e0e0e0',
        brightBlack: '#4a4a4a',
        brightRed: '#ff6e67',
        brightGreen: '#5af78e',
        brightYellow: '#f5d76e',
        brightBlue: '#00e5ff',
        brightMagenta: '#ff92df',
        brightCyan: '#00e5ff',
        brightWhite: '#ffffff',
      },
    });

    fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(terminalContainer);
    
    // Initial fit
    setTimeout(() => {
      fitAddon?.fit();
    }, 0);

    // Check if Tauri is available
    if (!terminalStore.isTauriAvailable()) {
      terminal?.write('\r\n\x1b[33mTerminal requires native Tauri app.\x1b[0m\r\n');
      terminal?.write('\x1b[90mRun with: npm run tauri dev\x1b[0m\r\n');
      return;
    }

    try {
      // Create PTY session
      sessionId = await terminalStore.create();
      
      // Get initial size
      const cols = terminal.cols;
      const rows = terminal.rows;
      await terminalStore.resize(sessionId, rows, cols);

      // Handle input from xterm -> PTY
      terminal.onData(async (data: string) => {
        if (sessionId !== null) {
          try {
            await terminalStore.write(sessionId, data);
          } catch (e) {
            console.error('Write error:', e);
          }
        }
      });

      // Poll for output from PTY -> xterm
      readInterval = setInterval(async () => {
        if (sessionId !== null && terminal) {
          try {
            const data = await terminalStore.read(sessionId);
            if (data.length > 0) {
              const text = new TextDecoder().decode(data);
              terminal.write(text);
            }
          } catch (e) {
            // Ignore read errors (may happen if terminal is busy)
          }
        }
      }, 16); // ~60fps polling

      // Handle resize
      resizeObserver = new ResizeObserver(() => {
        if (fitAddon && terminal && sessionId !== null) {
          fitAddon.fit();
          terminalStore.resize(sessionId, terminal.rows, terminal.cols);
        }
      });
      resizeObserver.observe(terminalContainer);

    } catch (e) {
      console.error('Failed to initialize terminal:', e);
      terminal?.write('\r\n\x1b[31mFailed to start terminal session.\x1b[0m\r\n');
      terminal?.write('\x1b[90mThis feature requires the native Tauri app.\x1b[0m\r\n');
    }
  });

  onDestroy(() => {
    if (readInterval) {
      clearInterval(readInterval);
    }
    if (resizeObserver) {
      resizeObserver.disconnect();
    }
    if (sessionId !== null) {
      terminalStore.close(sessionId).catch(() => {});
    }
    terminal?.dispose();
  });
</script>

<div class="terminal-wrapper" bind:this={terminalContainer}></div>

<style>
  .terminal-wrapper {
    width: 100%;
    height: 100%;
    background: #1a1a1a;
    padding: 4px;
    box-sizing: border-box;
  }

  .terminal-wrapper :global(.xterm) {
    height: 100%;
  }

  .terminal-wrapper :global(.xterm-viewport) {
    background: #1a1a1a !important;
  }

  .terminal-wrapper :global(.xterm-screen) {
    height: 100%;
  }
</style>
