# Feature Issue Template

Use for new language constructs, transpiler capabilities, or enhancements.

**Label:** `enhancement`
**Title format:** `feat: [short imperative description]`

Examples:
- `feat: implement DA exponentiation operator`
- `feat: add VE intrinsic function support`
- `feat: support multi-dimensional array indexing`

---

For small features, Goal + Acceptance Criteria + one test case is sufficient.
Omit sections that would just say "N/A".

**Body:**

```markdown
## Goal
[What are we building and why does it matter? Reference COSY compatibility if relevant.]

## Acceptance Criteria
- [ ] ...
- [ ] ...

## Test Cases

### Happy Path
- **Given** [initial state / ROSY input]
- **When** [the construct is used]
- **Then** [expected transpiled output / runtime behavior]

### Edge Cases
- **Given** [edge condition, e.g. type mismatch, zero-order DA]
- **When** [the construct is used]
- **Then** [expected graceful behavior / error message]

### COSY Compatibility
- [ ] Output matches COSY INFINITY for equivalent `.fox` program
- [ ] TDD codegen test added (if TypeRule/IntrinsicTypeRule applies)

## Implementation Notes
[Which files need changes? Grammar, AST node, traits, runtime?]

## Out of Scope
- ...

## Dependencies / Risks
- ...

## Notes
[Design decisions, alternatives considered, COSY manual references]
```
