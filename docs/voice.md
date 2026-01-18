# Axiom Voice Guidelines

## Tone Characteristics

### Minimalist
- Say less
- Every word earns its place
- No filler, no fluff

### Dry
- Understated
- No exclamation marks
- No enthusiasm theater

### Precise
- Technical accuracy over friendliness
- Specific over vague
- Measurable over subjective

### Slightly Dark (Tasteful)
- Acknowledge complexity
- Respect the difficulty of systems work
- Occasional gallows humor, never forced

## Examples

### Good
- "Compilation failed. 3 errors."
- "No toolchain detected."
- "Settings saved."
- "Symbol not found in index."

### Bad
- "Oops! Something went wrong!"
- "Great job! Your code compiled!"
- "We couldn't find what you're looking for."
- "Hang tight while we process that!"

## UI Microcopy Patterns

### Status Messages
- State the fact: "Build complete. 0 warnings."
- No celebration: Not "Build successful!"
- No apology: Not "Sorry, build failed."

### Error Messages
- What happened: "Linker error: undefined reference to 'main'"
- Where: "src/main.c:42"
- No speculation about why

### Empty States
- State the condition: "No files open."
- Suggest action minimally: "Open a file to begin."
- No emoji, no illustrations

### Confirmations
- Direct: "Delete this file?"
- Options: "Delete" / "Cancel"
- No "Are you sure you want to..."

## Strings Catalog

All user-facing strings live in `src/lib/strings/index.ts`.

Categories:
- Panel titles
- Button labels
- Status messages
- Error messages
- Tooltips
- Dialog text

Never hardcode strings in components.

---

*Axiom by HawkLogic Systems.*
