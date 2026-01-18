# Axiom Decision Log

## Format

Each decision follows this format:
- **ID**: Sequential number
- **Date**: When decided
- **Decision**: What was decided
- **Rationale**: Why
- **Status**: Active / Superseded / Deprecated

---

## Decisions

### D001: Technology Stack
**Date**: 2024-01-18
**Decision**: Use Rust + Tauri + Svelte + TypeScript
**Rationale**:
- Rust: Safety, performance, single backend language
- Tauri 2.x: Small binary, native performance, Rust IPC
- Svelte: Lightweight, reactive, fast compilation
- TypeScript: Type safety for frontend
**Status**: Active

### D002: Editor Component
**Date**: 2024-01-18
**Decision**: Use CodeMirror 6
**Rationale**:
- Extensible architecture
- No built-in AI features
- Good tree-sitter integration potential
- Active maintenance
**Status**: Active

### D003: Parsing Strategy
**Date**: 2024-01-18
**Decision**: Use tree-sitter for C/C++ parsing
**Rationale**:
- Incremental parsing
- Error recovery
- Battle-tested grammars
- Query language for extraction
**Status**: Active

### D004: Git Integration
**Date**: 2024-01-18
**Decision**: Use git2-rs (libgit2 bindings)
**Rationale**:
- Pure library, no git CLI dependency
- Offline operation
- Full control over operations
**Status**: Active

### D005: Terminal Implementation
**Date**: 2024-01-18
**Decision**: Use portable-pty for PTY, vte for parsing
**Rationale**:
- Cross-platform PTY support
- Standard ANSI parsing
- No external dependencies
**Status**: Active

### D006: Settings Format
**Date**: 2024-01-18
**Decision**: Use TOML with versioned schema
**Rationale**:
- Human-readable
- Well-supported in Rust (toml crate)
- Schema versioning enables migrations
**Status**: Active

### D007: Autocomplete Strategy
**Date**: 2024-01-18
**Decision**: Tab-triggered, symbol-table driven, deterministic
**Rationale**:
- No AI/probabilistic ranking (philosophy)
- Predictable behavior
- Alphabetical within kind for reproducibility
**Status**: Active

### D008: Python Bundling
**Date**: 2024-01-18
**Decision**: Bundle CPython 3.11 via python-build-standalone
**Rationale**:
- No system Python dependency
- Version-pinned for reproducibility
- Used for build scripts, not user code
**Status**: Active

### D009: Theme Default
**Date**: 2024-01-18
**Decision**: Dark theme as default
**Rationale**:
- Standard for development tools
- Reduces eye strain
- Matches ICARUS aesthetic
**Status**: Active

### D010: SPDX Headers
**Date**: 2024-01-18
**Decision**: Require SPDX headers on all source files
**Rationale**:
- Clear license attribution
- Apache-2.0 compliance
- Automated verification via script
**Status**: Active

---

## Pending Decisions

None at this time.

---

*Axiom is a HawkLogic Systems project.*
