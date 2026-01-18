# Axiom IDE — Context Capsule

## Project Identity
- **Name**: Axiom
- **Steward**: HawkLogic Systems
- **License**: Apache-2.0
- **Tagline**: "Deterministic. Inspectable. Offline."

## Current State
- **Branch**: dev
- **Active Workstream**: WS0 (Bootstrap)
- **Build Status**: Not yet buildable

## Philosophy (Immutable)
Axiom is an instrument panel for systems engineers. It reveals ground truth:
- Compiler output (not suggestions)
- AST structure (not guesses)
- ABI rules (not heuristics)
- ISA semantics (not approximations)
- Debugger state (not simulations)

**Axiom is NOT:**
- A productivity accelerator
- An AI assistant
- A cloud-connected service
- A telemetry collector

## Architecture
- Backend: Rust crates (axiom-core, axiom-settings, axiom-toolchain, axiom-parser, axiom-symbols, axiom-git, axiom-terminal)
- Frontend: Svelte + TypeScript via Tauri 2.x
- Editor: CodeMirror 6
- Parsing: tree-sitter (C/C++)
- Git: libgit2 via git2-rs
- Terminal: portable-pty

## Workstream Status
| WS | Name | Status |
|----|------|--------|
| 0 | Bootstrap | IN PROGRESS |
| 1 | Settings | Pending |
| 2 | Toolchain | Pending |
| 3 | Compiler | Pending |
| 4 | Parser | Pending |
| 5 | Symbols | Pending |
| 6 | Git | Pending |
| 7 | Terminal | Pending |
| 8 | UI Scaffold | Pending |
| 9 | Wiring | Pending |
| 10 | Branding | Pending |
| 11 | Final Build | Pending |

## Next Step
Complete WS0: Create all context files, docs, scripts, and initial structure.

## Resume Protocol
1. Read this file
2. Read .cursor/axiom_agent_state.json
3. Read docs/PROGRESS.md
4. Verify philosophy hash matches
5. Resume from next_step
