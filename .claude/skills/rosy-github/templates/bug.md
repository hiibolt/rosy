# Bug Issue Template

Use for incorrect transpilation, runtime errors, or COSY output mismatches.

**Label:** `bug`
**Title format:** `fix: [short description of what's broken]`

Examples:
- `fix: DA multiplication produces wrong coefficients for order > 3`
- `fix: string concatenation panics on empty operand`
- `fix: IF/ENDIF block generates invalid Rust when nested`

---

**Body:**

```markdown
## What's Broken
[What's happening that shouldn't be?]

## Steps to Reproduce
1. Create a `.rosy` file with: [minimal reproduction]
2. Run: `cargo run --bin rosy -- run <file>`
3. Observe: [error or incorrect output]

## Expected vs Actual
- **Expected:** [correct behavior, ideally matching COSY output]
- **Actual:** [what happens instead — include error messages or output diff]

## Environment
- Rosy version/commit:
- OS:
- Reproducible: always / sometimes / rarely

## COSY Reference
- [ ] Equivalent `.fox` program produces correct output in COSY INFINITY
- [ ] COSY manual section: [chapter/section if relevant]

## Acceptance Criteria
- [ ] The steps to reproduce no longer produce the bug
- [ ] Existing tests pass
- [ ] TDD output diff matches COSY (if applicable)
- [ ] ...

## Test Cases

### Regression Test
- **Given** [the conditions that trigger the bug]
- **When** [the triggering action]
- **Then** [correct behavior]

### Related Edge Cases
- ...

## Dependencies / Risks
- ...

## Notes
[Stack traces, transpiled Rust output, suspected cause]
```
