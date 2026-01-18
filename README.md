# Axiom

**Deterministic. Inspectable. Offline.**

Axiom is an avionics-grade IDE for embedded systems engineers. Built for those who work close to the metal.

*A HawkLogic Systems project.*

---

## What is Axiom?

Axiom is an instrument panel, not a productivity tool. It reveals ground truth:

- **Compiler output** — What your compiler actually said
- **AST structure** — What the parser actually parsed
- **ABI rules** — What the binary interface actually is
- **ISA semantics** — What instructions actually do
- **Debugger state** — What the hardware actually shows

## What Axiom is NOT

- Not an AI assistant
- Not a cloud service
- Not a telemetry collector
- Not a "smart" IDE

## Philosophy

Axiom is built on immutable principles:

1. **No generative AI** — Autocomplete is symbol-table driven, deterministic
2. **No cloud services** — Your code never leaves your machine
3. **No telemetry** — Zero bytes transmitted
4. **No speculation** — Only ground truth
5. **No probabilistic behavior** — Same input → same output

See [docs/philosophy.md](docs/philosophy.md) for the full philosophy.

## Target Audience

- Embedded C/C++ engineers
- RTOS and kernel developers
- Bare-metal ARM Cortex-M engineers
- Safety-critical and regulated-domain engineers

## Features

- **Editor**: CodeMirror 6 with C/C++ syntax highlighting
- **Parsing**: Tree-sitter based AST with visualization
- **Autocomplete**: Tab-triggered, deterministic, alphabetically ordered
- **Toolchains**: Clang, GCC, ARM GCC detection and management
- **Git**: Status, diff, staging, commit (via libgit2)
- **Terminal**: Integrated PTY terminal
- **Themes**: Dark (default) and light

## Platform

- **Primary**: macOS (Apple Silicon)
- **Future**: Linux, Windows

## Installation

### Prerequisites

- macOS 12+ (Apple Silicon or Intel)
- Rust 1.75+ (for building from source)
- Node.js 20+ (for building from source)

### From Source

```bash
git clone https://github.com/hawklogic/axiom.git
cd axiom
npm install
scripts/build.sh
```

The built application will be in `src-tauri/target/release/bundle/`.

### Development

```bash
# Run in development mode
scripts/dev.sh

# Run tests
scripts/test_fast.sh

# Run full test suite with lints
scripts/test_full.sh
```

## Configuration

Settings are stored in TOML format at:
- macOS: `~/Library/Application Support/com.hawklogic.axiom/settings.toml`

See Settings panel in the app for all options.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

**Note**: We do not accept contributions that add AI features, cloud services, or telemetry. This is non-negotiable.

## License

Apache-2.0. See [LICENSE](LICENSE).

## Acknowledgments

Axiom is built with:
- [Tauri](https://tauri.app/) — Desktop framework
- [Svelte](https://svelte.dev/) — UI framework
- [CodeMirror](https://codemirror.net/) — Editor component
- [tree-sitter](https://tree-sitter.github.io/) — Parsing
- [git2-rs](https://github.com/rust-lang/git2-rs) — Git operations

---

**Axiom** — *The instrument, not the pilot.*

*A HawkLogic Systems project.*
