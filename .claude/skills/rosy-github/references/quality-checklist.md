# Quality Gates — Pre-Issue Checklist

Read this silently before exiting Phase 1. If any gate is red, go back and address it
conversationally — don't dump this list on the user.

## The INVEST Test

| Letter | Meaning        | Question to ask yourself                                               |
|--------|----------------|------------------------------------------------------------------------|
| I      | Independent    | Can this be built/shipped without another issue being done first?      |
| N      | Negotiable     | Is the scope flexible, or is it rigidly over-specified?                |
| V      | Valuable       | Is it clear who benefits and how?                                      |
| E      | Estimable      | Could the effort be estimated roughly?                                 |
| S      | Small          | Does the Size field reflect this? `Large`/`X-Large`/`Gigantic` issues are a smell — split them. |
| T      | Testable       | Do the acceptance criteria make it unambiguous whether it's done?      |

If the issue fails **S** — it's too big. Gently suggest splitting it.
If it fails **T** — the acceptance criteria need work. Keep going.

## Transpiler-Specific Concerns

### TDD Compliance
- If this adds an operator or intrinsic, does it have TypeRule/IntrinsicTypeRule entries?
- Will `codegen.rs` auto-generate the `.rosy` and `.fox` test files?
- Is the expected COSY output known and documented?

### Pipeline Impact
- Does this touch the PEG grammar? If so, are there ambiguity risks?
- Does this add a new AST node? Are all trait impls covered (FromRule, Transpile,
  TranspileableExpr/TranspileableStatement)?
- Does this affect the type resolution pass?

### Type System
- Are all supported type combinations documented?
- Are unsupported combinations explicitly handled (clear error, not panic)?
- Does the return type logic match COSY INFINITY's behavior?

### COSY Compatibility
- Is there a COSY manual reference for this construct?
- Has the equivalent `.fox` program been tested in COSY INFINITY?
- Are there known COSY behaviors that are intentionally different in ROSY?

## Common Things People Forget

- [ ] What happens with DA type inputs? (order 0, high order, negative coefficients)
- [ ] What happens with complex types (CM, CD)?
- [ ] What happens with array types (`RE ** 2`, `ST ** 3`)?
- [ ] Does this interact with the existing operator/intrinsic registries?
- [ ] Does this need a version bump in `rosy/Cargo.toml`?
- [ ] Are there related COSY manual chapters that should be cross-referenced?

## Scope Creep Watch
- Did anything come up in the conversation that sounds like a separate issue?
- If so, suggest capturing it as a follow-on ticket rather than bloating this one.
