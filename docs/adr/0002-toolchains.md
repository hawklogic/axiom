# ADR 0002: Toolchain Strategy

## Status
Accepted

## Context
Axiom needs to detect and invoke C/C++ toolchains without assuming the user's system is configured in any particular way. The principle is: "Axiom does not assume your system is sane."

## Decision

### Supported Toolchains
1. **LLVM/Clang** (default for macOS)
2. **GCC** (optional)
3. **GNU ARM Embedded Toolchain** (for embedded targets)
4. **Python** (bundled CPython 3.11 for build scripts)

### Detection Strategy
- Probe known paths only (no PATH scanning)
- macOS paths:
  - `/usr/bin/clang`
  - `/opt/homebrew/bin/clang`
  - `/Applications/ARM/bin/arm-none-eabi-gcc`
  - `/usr/local/bin/gcc-*`
- Parse `--version` output for version info
- Store detected toolchains in settings

### Override Precedence
1. User-specified path in settings (highest priority)
2. Bundled toolchain (Python only in v1)
3. Auto-detected from known paths (lowest priority)

### Rules
- **Never mutate PATH**: All invocations use absolute paths
- **Never auto-install**: User must install toolchains manually
- **Never auto-upgrade**: Version stays as detected until user changes
- **Explicit over implicit**: Show user exactly which binary will be used

### Python Bundling
- Bundle python-build-standalone CPython 3.11
- Location: `vendor/python/bin/python3`
- Used for: build scripts, test runners, flash utilities
- Never used for: user code execution

## Consequences
- Users must have toolchains installed
- No "magic" installation experience
- Full transparency about which tools are used
- Reproducible builds across machines (when same versions installed)

## References
- docs/philosophy.md (Section 4: No Speculation)
