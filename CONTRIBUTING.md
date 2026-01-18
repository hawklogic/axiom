# Contributing to Axiom

Thank you for your interest in contributing to Axiom.

## Before You Contribute

Please read and understand:
- [Philosophy](docs/philosophy.md) — Our core principles
- [License](LICENSE) — Apache-2.0

## Contribution Guidelines

### What We Accept

- Bug fixes with tests
- Documentation improvements
- Performance optimizations
- Accessibility improvements
- Test coverage improvements

### What We Do NOT Accept

The following will be rejected without discussion:

1. **AI/ML features**: No generative AI, no probabilistic suggestions, no "smart" anything
2. **Cloud services**: No remote APIs, no sync, no accounts
3. **Telemetry**: No analytics, no crash reporting, no usage tracking
4. **Non-Apache-2.0 code**: All contributions must be Apache-2.0 compatible

### Code Requirements

#### SPDX Headers (Mandatory)

Every source file (`.rs`, `.ts`, `.svelte`, `.css`) MUST include:

```
// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 HawkLogic Systems
```

Run `scripts/check_spdx.sh` to verify before submitting.

#### Code Style

- Rust: `cargo fmt` and `cargo clippy`
- TypeScript/Svelte: Prettier with project config
- No TODO comments in merged code
- No commented-out code

#### Testing

- All new code must have tests
- All tests must pass: `scripts/test_full.sh`
- No speculative assertions — test only verifiable facts

### Pull Request Process

1. Fork the repository
2. Create a feature branch from `dev`: `git checkout -b feature/your-feature dev`
3. Make your changes
4. Run tests: `scripts/test_full.sh`
5. Run SPDX check: `scripts/check_spdx.sh`
6. Commit with clear message
7. Push to your fork
8. Open PR against `dev` branch

### Commit Messages

Format:
```
[Component] Brief description

Longer explanation if needed.
```

Examples:
- `[axiom-parser] Fix crash on empty file`
- `[ui] Add keyboard shortcut for split view`
- `[docs] Update toolchain detection docs`

### Review Process

- All PRs require review
- CI must pass
- SPDX headers must be present
- Philosophy compliance checked

## Development Setup

### Prerequisites

- Rust 1.75+
- Node.js 20+
- macOS (primary platform)

### Setup

```bash
git clone https://github.com/hawklogic/axiom.git
cd axiom
npm install
cargo build --workspace
```

### Development

```bash
# Run in dev mode
scripts/dev.sh

# Run fast tests
scripts/test_fast.sh

# Run full test suite
scripts/test_full.sh
```

## Questions?

Open an issue with the `question` label.

---

*Axiom is a HawkLogic Systems project, licensed under Apache-2.0.*
