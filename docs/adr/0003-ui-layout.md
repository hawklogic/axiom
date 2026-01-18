# ADR 0003: UI Layout Architecture

## Status
Accepted

## Context
Axiom needs a flexible, professional UI layout that supports the workflows of embedded systems engineers while maintaining the ICARUS aesthetic.

## Decision

### Layout System
- **Split views**: Horizontal and vertical splitting
- **Dockable panels**: Panels can be docked to edges or floated
- **Drag-and-drop tabs**: Files can be moved between editor groups
- **Persistent layout**: Layout saved to settings, restored on launch

### Panel Types
1. **File Explorer**: Tree view of project files
2. **Source Control**: Git status, staging, diff view
3. **AST Viewer**: Tree-sitter node display, click-to-navigate
4. **Assembly View**: Disassembly output, linked to source lines
5. **Debug Panel**: Breakpoints, watch, call stack, registers (stub v1)
6. **Terminal**: Integrated PTY terminal
7. **Settings**: Categorized settings UI

### Default Layout
```
+------------------+------------------------+
|                  |                        |
|  File Explorer   |      Editor Tabs       |
|                  |                        |
|                  +------------------------+
+------------------+                        |
|                  |      Editor Area       |
|  Source Control  |                        |
|                  +------------------------+
+------------------+       Terminal         |
|   AST Viewer     |                        |
+------------------+------------------------+
|                   Status Bar              |
+-------------------------------------------+
```

### Status Bar Content
- File encoding (UTF-8, etc.)
- Line/column position
- Git branch name
- Active toolchain
- Build status indicator

### Theme System
- Dark theme (default)
- Light theme
- Theme persisted in settings
- CSS custom properties for easy theming

### Implementation
- Svelte components with CSS Grid/Flexbox
- Panel state managed in Svelte stores
- Layout serialized to settings on change
- Smooth transitions disabled (ICARUS aesthetic: no fluff)

## Consequences
- Familiar IDE layout for users
- Flexible for different workflows
- Consistent with ICARUS aesthetic
- Layout persistence across sessions

## References
- docs/brand.md (ICARUS Aesthetic section)
