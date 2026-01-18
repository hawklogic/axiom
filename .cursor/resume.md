# Axiom IDE — Resume Checklist

Use this checklist when resuming work after context loss.

## Quick Resume Steps

1. [ ] Read .cursor/axiom_agent_state.json
2. [ ] Read .cursor/context.md
3. [ ] Read docs/PROGRESS.md
4. [ ] Verify philosophy hash: sha256sum docs/philosophy.md
5. [ ] Check current branch: git branch --show-current
6. [ ] Run quick verification:
   - cargo test --workspace (if crates exist)
   - npm run check (if frontend exists)
7. [ ] Resume from next_step in agent state

## State File Locations

| File | Purpose |
|------|---------|
| .cursor/axiom_agent_state.json | Machine-readable state |
| .cursor/context.md | Human-readable summary |
| docs/PROGRESS.md | Detailed progress log |
| docs/WORKSTREAMS.md | Workstream definitions |
| docs/DECISIONS.md | Decision log |

## Invariants (Must Never Change)

- No AI/ML code generation
- No cloud services
- No telemetry
- Apache-2.0 only
- SPDX headers on all source files
- Axiom is product name, HawkLogic is steward
- Deterministic autocomplete ordering
- No PATH mutation

## Branch Rules

- main — stable releases only
- dev — integration branch
- feature/* — new features
- fix/* — bug fixes
- docs/* — documentation

Never commit directly to main. Always PR through dev.

## Checkpoint Protocol

At the end of each workstream:
1. Update axiom_agent_state.json
2. Update .cursor/context.md
3. Update docs/PROGRESS.md
4. Commit with message: [WS#] Complete: <summary>
5. Push to remote
6. Merge to dev
