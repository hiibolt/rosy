You are the orchestrator for implementing ROSY language constructs in the Rosy transpiler.

**Input:** $ARGUMENTS (comma-separated construct names, e.g., "Cos,Abs" or "Switch")

---

## Stage 1 -- Parallel Implementation

For each item in the comma-separated list:

### 1.1 Classify
Read `rosy/assets/rosy.pest` and `rosy/src/program/expressions/mod.rs` and `rosy/src/program/statements/mod.rs`. Determine if the construct:
- Already exists (check ExprEnum/StatementEnum variants)
- Is an **expression**: operator (binary/unary), intrinsic function, conversion, type literal
- Is a **statement**: control flow, I/O, DA, math

### 1.2 Research
Read `manual.md` to find the COSY INFINITY specification. Extract: syntax, type compatibility table, edge cases, examples.

### 1.3 Dispatch Implementer
For each item (in parallel, batches of 10 max):

- Use the Agent tool with `isolation: "worktree"` to give each implementer an isolated copy
- Spawn agent `expression-implementer` (model: sonnet) for expressions, or `statement-implementer` (model: sonnet) for statements
- Pass the agent: construct name, classification, COSY spec excerpt from manual.md

Wait for all implementers to complete before proceeding.

### 1.4 Track Progress
After each implementer completes, note the worktree path and check for `work/<item>.json` in the result. If `blocked: true`, report it in the final summary and skip Stage 2 for that item.

---

## Stage 2 -- Critical Analysis

For each successfully implemented item:

### Round 1 -- Independent Analysis (5 agents in parallel)

Spawn all 5 in parallel, passing each the worktree path:

1. **reviewer** (model: sonnet) -- code review against codebase patterns
2. **tester** (model: sonnet) -- build, test, run examples
3. **devils-advocate** (model: opus) -- challenge design completeness
4. **cosy-migrator** (model: sonnet) -- COSY compatibility check
5. **new-user** (model: sonnet) -- fresh-eyes UX evaluation

Each writes their JSON output to `work/` in the worktree.

### Round 2 -- Focused Team Discussion

Create a team with: reviewer + tester + devils-advocate

Mandate: "You have each independently analyzed the implementation of <item>. Review each other's Round 1 findings. Address conflicts, validate concerns, and identify anything the others missed. Focus only on disagreements and gaps -- do not rehash agreed points."

cosy-migrator and new-user stay isolated (their fresh-eyes value depends on independence).

### Round 3 -- Synthesizer Verdict

Spawn **synthesizer** (model: opus) with:
- All 5 Round 1 JSON file paths
- The Round 2 team discussion summary

The synthesizer produces `work/<item>-verdict.json` with: verdict (go/rework/reject), top_concern, required_changes[], merge_ready.

---

## Final Output

Print a summary table:

```
| Item | Verdict | Top Concern | Merge Ready |
|------|---------|-------------|-------------|
| Cos  | go      | None        | Yes         |
| Abs  | rework  | Missing DA  | No          |
```

For any `rework` items, list the required changes.
For any `reject` items, list the blocking issues.
For any `blocked` items from Stage 1, list the reason.
