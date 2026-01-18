# Axiom Workstreams

## Overview

Axiom development follows a sequential workstream model. Each workstream must complete before the next begins. Backend work precedes frontend work.

## Workstream Definitions

### WS0: Repository Bootstrap + Context Persistence
**Goal**: Establish repo structure, context files, and agent resume capability.

**Deliverables**:
- .cursorrules
- .cursor/context.md, resume.md, axiom_agent_state.json
- docs/philosophy.md, brand.md, voice.md
- docs/PROGRESS.md, WORKSTREAMS.md, DECISIONS.md
- docs/adr/0001-0003
- NOTICE, CONTRIBUTING.md, README.md
- scripts/check_spdx.sh
- src/lib/strings/index.ts (stub)

**Exit Criteria**: All context files present, git branch structure created.

---

### WS1: Settings Schema + Persistence
**Goal**: Implement TOML-based settings with versioned schema and migrations.

**Crate**: `crates/axiom-settings/`

**Deliverables**:
- SettingsSchema struct with serde
- TOML read/write
- Schema versioning
- Migration system
- Validation layer

**Exit Criteria**: Settings can be saved, loaded, migrated, validated.

---

### WS2: Toolchain Detection + Overrides
**Goal**: Detect C/C++ toolchains and Python from known paths.

**Crate**: `crates/axiom-toolchain/`

**Deliverables**:
- ToolchainKind enum (Clang, GCC, ArmGcc, Python)
- DetectedToolchain struct
- Path probing for known locations
- Version parsing
- Override precedence logic
- Bundled Python integration

**Exit Criteria**: Toolchains detected, versions parsed, overrides work.

---

### WS3: Compiler Invocation Adapter
**Goal**: Construct and execute compiler commands deterministically.

**Crate**: `crates/axiom-toolchain/` (extends)

**Deliverables**:
- CompileRequest/CompileResult structs
- Flag construction per toolchain
- Dry-run mode
- Diagnostic parsing

**Exit Criteria**: Compile commands constructed correctly, diagnostics parsed.

---

### WS4: Tree-sitter Parsing + AST
**Goal**: Parse C/C++ using tree-sitter.

**Crate**: `crates/axiom-parser/`

**Deliverables**:
- Parser wrapper with language detection
- AstNode representation
- Incremental parsing
- Query support
- Error detection

**Exit Criteria**: C/C++ files parse correctly, AST accessible.

---

### WS5: Symbol Index + Autocomplete
**Goal**: Build symbol table for deterministic autocomplete.

**Crate**: `crates/axiom-symbols/`

**Deliverables**:
- Symbol struct
- SymbolIndex with prefix search
- Scope tracking
- Deterministic ordering

**Exit Criteria**: Symbols indexed, completions deterministic.

---

### WS6: Git Backend
**Goal**: Git operations via libgit2.

**Crate**: `crates/axiom-git/`

**Deliverables**:
- Repository open/detect
- Status, diff, stage, commit
- Branch info

**Exit Criteria**: Git operations work on real repos.

---

### WS7: Terminal Backend
**Goal**: Integrated terminal via PTY.

**Crate**: `crates/axiom-terminal/`

**Deliverables**:
- PTY spawn
- Read/write streams
- Resize handling
- ANSI parsing
- Multiple sessions

**Exit Criteria**: Terminal spawns, accepts input, renders output.

---

### WS8: Tauri UI Scaffold
**Goal**: Set up Tauri 2.x + Svelte frontend.

**Deliverables**:
- Svelte + Vite setup
- Layout system (split view, docking)
- CodeMirror 6 integration
- Panel component stubs
- Theme system

**Exit Criteria**: UI launches, panels render, themes switch.

---

### WS9: Backend-UI Wiring
**Goal**: Connect Rust crates to Svelte via Tauri commands.

**Deliverables**:
- Tauri command handlers
- Frontend stores
- IPC round-trips

**Exit Criteria**: All backend features accessible from UI.

---

### WS10: Branding + About Panel + Splash
**Goal**: Visual identity assets and About panel.

**Deliverables**:
- SVG logo assets
- App icon bundle
- Splash screen
- About panel
- Strings catalog completion

**Exit Criteria**: Assets present, About panel renders.

---

### WS11: Integration Tests + Final Build
**Goal**: Full test suite, macOS build.

**Deliverables**:
- Integration tests
- Production build scripts
- macOS bundle (.app, .dmg)
- Final verification

**Exit Criteria**: All tests pass, app launches, all features work.

---

## Rules

1. Complete workstreams in order
2. Backend before frontend
3. Tests must pass before merge
4. Update context files at each checkpoint
5. No skipping workstreams

---

*Axiom is a HawkLogic Systems project.*
