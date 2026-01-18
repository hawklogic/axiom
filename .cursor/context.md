# Axiom IDE — Context Capsule

## Project Identity
- **Name**: Axiom
- **Steward**: HawkLogic Systems
- **License**: Apache-2.0
- **Tagline**: "Deterministic. Inspectable. Offline."

## Current State
- **Branch**: dev
- **Active Workstream**: WS10 (Branding)
- **Build Status**: Code complete, blocked by system LLVM issue

## Philosophy Hash
`0f5e7107568bdd6d6e34f11756ed19263ee0d0c11fabec35bfbdc92294ecd343`

## Architecture
- Backend: 7 Rust crates completed
  - axiom-core: Types, errors
  - axiom-settings: TOML schema, migrations
  - axiom-toolchain: Detection, invocation
  - axiom-parser: tree-sitter C/C++
  - axiom-symbols: Symbol index, autocomplete
  - axiom-git: libgit2 operations
  - axiom-terminal: PTY sessions
- Frontend: Svelte + TypeScript via Tauri 2.x
- Editor: CodeMirror 6 (integration ready)
- All Tauri command handlers implemented
- All Svelte stores implemented

## Workstream Status
| WS | Name | Status |
|----|------|--------|
| 0-9 | Bootstrap through Wiring | COMPLETE |
| 10 | Branding | IN PROGRESS (SVGs pending) |
| 11 | Final Build | PENDING |

## Next Step
1. Create SVG brand assets (logo, icon, splash)
2. Resolve system LLVM issue
3. Run npm install && cargo build
4. Final verification

## Known Issues
- System has LLVM library conflict (homebrew rust vs llvm)
- Fix: brew uninstall llvm or use rustup-installed rust

## Resume Protocol
1. Read this file
2. Read .cursor/axiom_agent_state.json
3. Read docs/PROGRESS.md
4. Verify philosophy hash matches
5. Resume from next_step
