# ADR 0001: Project Scope

## Status
Accepted

## Context
Axiom needs a clearly defined scope to prevent feature creep and maintain focus on its core mission: serving embedded systems engineers with a deterministic, offline IDE.

## Decision

### In Scope
- C/C++ editing with syntax highlighting
- Tree-sitter based parsing and AST visualization
- Deterministic, symbol-table driven autocomplete
- Toolchain detection and management (Clang, GCC, ARM GCC)
- Compiler invocation with diagnostic parsing
- Git integration (status, diff, stage, commit)
- Integrated terminal
- Assembly view (disassembly output)
- Debug panel (stub for v1, breakpoints list, variable watch)
- Settings management with TOML persistence
- Light and dark themes

### Out of Scope (v1)
- Language Server Protocol (LSP) support
- Remote development
- Plugin/extension system
- Refactoring tools
- Code formatting
- Linting beyond compiler output
- Project templates
- Build system integration (CMake, Make, etc.)
- Flash/upload to hardware
- JTAG/SWD debugging

### Explicitly Rejected (Forever)
- Generative AI features
- Cloud services
- Telemetry
- Probabilistic autocomplete
- "Smart" suggestions

## Consequences
- Focused v1 deliverable
- Clear rejection criteria for feature requests
- Potential future expansion via ADRs

## References
- docs/philosophy.md
