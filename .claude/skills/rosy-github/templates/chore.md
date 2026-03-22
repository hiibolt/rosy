# Chore / Refactor Issue Template

Use for tech debt, build system changes, dependency updates, refactors, or
non-user-facing improvements to the transpiler.

**Label:** `chore`
**Title format:** `chore: [description]` or `refactor: [what's being restructured]`

Examples:
- `chore: update pest to v2.8`
- `refactor: extract operator codegen into shared trait`
- `chore: add CI workflow for COSY output diffing`

---

**Body:**

```markdown
## What & Why
[What are we changing, and what problem does it solve?
"Because the macro is unreadable" is okay, but "because it makes adding
new operators take 5 min instead of 45" is better.]

## Approach
[High-level plan. What will change structurally?]

## Acceptance Criteria
- [ ] No unintended behavior changes (or changes listed explicitly below)
- [ ] All existing tests pass
- [ ] TDD output diffs still match COSY
- [ ] ...

## Test Plan
- [ ] `cargo test` passes
- [ ] `cargo build --release` succeeds
- [ ] Manually tested: [describe what]
- [ ] ...

## Dependencies / Risks
[What could this affect? What should reviewers watch for?]

## Notes
[Context, links to docs, benchmarks before/after]
```
