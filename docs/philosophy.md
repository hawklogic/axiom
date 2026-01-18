# Axiom Philosophy

**Axiom** is built on immutable principles. These are not preferences—they are constraints.

## Core Axioms

### 1. No Generative AI
Axiom does not generate code, suggest completions probabilistically, or employ machine learning models. Autocomplete is symbol-table driven, deterministic, and alphabetically ordered within kind.

### 2. No Cloud Services
Axiom operates entirely offline. There are no accounts, no sync services, no remote APIs. Your code never leaves your machine.

### 3. No Telemetry
Axiom collects nothing. No usage statistics, no crash reports, no analytics. Zero bytes transmitted.

### 4. No Speculation
Axiom shows only ground truth:
- **Compiler output**: What the compiler actually said
- **AST structure**: What the parser actually parsed
- **ABI rules**: What the binary interface actually is
- **ISA semantics**: What the instruction actually does
- **Debugger state**: What the hardware actually shows

### 5. No Probabilistic Behavior
Same input → same output. Always. Axiom is deterministic by design.

## What Axiom Is

Axiom is an **instrument panel** for systems engineers.

Like an aircraft's flight instruments, Axiom displays factual readings about your code and its execution environment. It does not fly the plane for you.

## What Axiom Is Not

- **Not a productivity accelerator**: We don't promise to make you faster
- **Not an AI assistant**: We don't guess what you meant
- **Not a cloud service**: We don't phone home
- **Not a telemetry collector**: We don't watch you work

## Target Audience

Axiom is for engineers who work close to the metal:
- Embedded C/C++ engineers
- RTOS and kernel developers
- Bare-metal ARM Cortex-M engineers
- Safety-critical and regulated-domain engineers

## Non-Audience

Axiom is explicitly **not** for:
- Web development
- Cloud-first workflows
- AI-assisted coding expectations
- "Magic" IDE experiences

## The Axiom Guarantee

When you use Axiom, you can trust that:
1. Every completion comes from your symbol table
2. Every error comes from your compiler
3. Every byte stays on your machine
4. Every session is reproducible

**Deterministic. Inspectable. Offline.**

---

*Axiom by HawkLogic Systems, licensed under Apache-2.0.*
