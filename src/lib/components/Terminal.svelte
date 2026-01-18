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
  let resizeObserver: ResizeObserver | null = null;

  onMount(async () => {
    if (!browser) return;
    console.log('[Terminal] onMount started');

    // Dynamically import xterm.js (browser-only)
    console.log('[Terminal] Importing xterm.js...');
    const { Terminal } = await import('@xterm/xterm');
    const { FitAddon } = await import('@xterm/addon-fit');
    await import('@xterm/xterm/css/xterm.css');
    console.log('[Terminal] xterm.js imported');

    // Initialize xterm.js with full color support
    terminal = new Terminal({
      fontFamily: '"JetBrains Mono", "SF Mono", Monaco, Menlo, monospace',
      fontSize: 13,
      lineHeight: 1.2,
      cursorBlink: true,
      cursorStyle: 'block',
      allowTransparency: true,
      scrollback: 10000,
      theme: {
        // Background and foreground
        background: '#0d1117',
        foreground: '#c9d1d9',
        cursor: '#58a6ff',
        cursorAccent: '#0d1117',
        selectionBackground: 'rgba(56, 139, 253, 0.4)',
        selectionForeground: '#ffffff',
        
        // Standard colors (0-7)
        black: '#484f58',
        red: '#ff7b72',
        green: '#3fb950',
        yellow: '#d29922',
        blue: '#58a6ff',
        magenta: '#bc8cff',
        cyan: '#39c5cf',
        white: '#b1bac4',
        
        // Bright colors (8-15)
        brightBlack: '#6e7681',
        brightRed: '#ffa198',
        brightGreen: '#56d364',
        brightYellow: '#e3b341',
        brightBlue: '#79c0ff',
        brightMagenta: '#d2a8ff',
        brightCyan: '#56d4dd',
        brightWhite: '#f0f6fc',
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
      console.log('[Terminal] Tauri not available, showing message');
      terminal?.write('\r\n\x1b[33mTerminal requires native Tauri app.\x1b[0m\r\n');
      terminal?.write('\x1b[90mRun with: npm run tauri dev\x1b[0m\r\n');
      return;
    }

    console.log('[Terminal] Tauri available, setting up event listener first...');
    try {
      // Set up event listener BEFORE creating PTY session to avoid race condition
      console.log('[Terminal] Importing @tauri-apps/api/event...');
      const { listen } = await import('@tauri-apps/api/event');
      console.log('[Terminal] Event API imported successfully');
      
      // We'll store received data until sessionId is set
      let pendingData: { id: number; data: number[] }[] = [];
      let listenerSessionId: number | null = null;
      
      const unlisten = await listen<{ id: number; data: number[] }>('terminal-output', (event) => {
        console.log('[Terminal] Received event for session', event.payload.id, 'bytes:', event.payload.data.length);
        if (listenerSessionId === null) {
          // Session not created yet, queue the data
          pendingData.push(event.payload);
          return;
        }
        if (event.payload.id === listenerSessionId && terminal) {
          const bytes = new Uint8Array(event.payload.data);
          const text = new TextDecoder().decode(bytes);
          terminal.write(text);
        }
      });
      
      // Store unlisten function for cleanup
      (window as any).__terminalUnlisten = unlisten;
      console.log('[Terminal] Event listener registered');
      
      // Now create PTY session
      console.log('[Terminal] Creating PTY session...');
      sessionId = await terminalStore.create();
      listenerSessionId = sessionId;
      console.log('[Terminal] PTY session created:', sessionId);
      
      // Process any pending data that arrived before sessionId was set
      for (const data of pendingData) {
        if (data.id === sessionId && terminal) {
          const bytes = new Uint8Array(data.data);
          const text = new TextDecoder().decode(bytes);
          terminal.write(text);
        }
      }
      pendingData = [];
      
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

      // Handle resize
      resizeObserver = new ResizeObserver(() => {
        if (fitAddon && terminal && sessionId !== null) {
          fitAddon.fit();
          terminalStore.resize(sessionId, terminal.rows, terminal.cols);
        }
      });
      resizeObserver.observe(terminalContainer);

    } catch (e: any) {
      console.error('Failed to initialize terminal:', e);
      console.error('Error details:', e?.message, e?.stack);
      terminal?.write('\r\n\x1b[31mFailed to start terminal session.\x1b[0m\r\n');
      terminal?.write(`\x1b[90mError: ${e?.message || e}\x1b[0m\r\n`);
    }
  });

  onDestroy(() => {
    // Cleanup event listener
    if ((window as any).__terminalUnlisten) {
      (window as any).__terminalUnlisten();
      delete (window as any).__terminalUnlisten;
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
    background: #0d1117;
    padding: 4px;
    box-sizing: border-box;
  }

  .terminal-wrapper :global(.xterm) {
    height: 100%;
  }

  .terminal-wrapper :global(.xterm-viewport) {
    background: #0d1117 !important;
  }

  .terminal-wrapper :global(.xterm-screen) {
    height: 100%;
  }

  /* Better scrollbar styling */
  .terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar) {
    width: 8px;
  }

  .terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-track) {
    background: #0d1117;
  }

  .terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-thumb) {
    background: #30363d;
    border-radius: 4px;
  }

  .terminal-wrapper :global(.xterm-viewport::-webkit-scrollbar-thumb:hover) {
    background: #484f58;
  }
</style>
