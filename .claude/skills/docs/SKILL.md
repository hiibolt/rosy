---
name: docs
description: >
  Audit and update project documentation after code changes — README files, skills,
  the root CLAUDE.md, and memory. Use after any session that added/removed any statements
  or expressions, changed the build/test setup, or renamed things. Also use when the user 
  says "update docs", "sync docs", "are the docs up to date", or "refresh documentation". 
  Can be invoked proactively at the end of a work session when significant structural changes 
  were made.
---

# Docs Sync

Audit project documentation for staleness after code changes. The goal is to keep
docs accurate without the user having to remember which files to update.

## What to audit

Check each of these sources against the current codebase. Only update what's actually
stale — don't rewrite things that are still accurate.

### 1. Root CLAUDE.md

**File:** `/CLAUDE.md`

Check these sections against reality:
- **Project Structure** — Do the directory descriptions match? Any new directories (utils/, services/) missing?
- **Commands** — Are all build/test/lint commands listed? Did test counts change significantly?
- **Code Style** — Do the conventions still hold? Any new patterns established?
- **Unified Frontend Architecture** — Do the component names and layout descriptions match what exists?
- **Database** — Did the Mutex type, table list, or migration pattern change?
- **Architecture Rules** — Are the stated patterns still accurate?

How to check: Read CLAUDE.md, then glob for key files it references. If a referenced file
doesn't exist or a described pattern no longer applies, update the section.

### 2. Module-Level Documentation

While you don't need to audit the documentation of the expressions or statements yourself,
you do need to need to audit the module-level documentation especially since those tend to go
out of sync as more additions are made. 

**Important: `r#` in doc links.** Modules like `break`, `if`, `loop` use `r#` in Rust code
(`pub mod r#break`) because they're reserved keywords. However, rustdoc intra-doc links must
**not** include the `r#` prefix — write `[`break`]`, not `[`r#break`]`. Rustdoc parses `r#foo`
as a link to item `r` with anchor `#foo`, producing a confusing "unresolved link to `r`" warning.
Always grep for `\[`r#` after editing module docs and fix any occurrences.

**Files:**
- `rosy/src/program/expressions/**/mod.rs`
- `rosy/src/program/statements/**/mod.rs`

Note that they recurse down. All modulefiles should be checked. If any changes were made or
are necessary, update the root documentation accordingly (`rosy/src/main.rs`).

### 3. Skills

**Directory:** `.claude/skills/*/SKILL.md`

For each skill, check whether the patterns it describes still match the code:

| Skill | What to verify |
|-------|---------------|
| `docs` | This skill - should it be updating other parts too? |
| `rosy-github` | Are the MCP tool names current? |
| `bigpicture` | Are the data source commands still valid? |

Only read skills that are likely affected by the changes made in this session. Don't
audit every skill for an IDE plugin tweak.

### 4. Memory

**File:** `/home/hiibolt/.claude/projects/-home-hiibolt-development-rosy/memory/MEMORY.md`

Check if any memory entries reference things that no longer exist (deleted files,
renamed patterns, old architecture). Update or remove stale entries.

## How to run

1. **Identify what changed** — Look at the git diff or the conversation history to
   understand what was modified in this session.
2. **Scope the audit** — Only check docs that could be affected. A Rust-only change
   doesn't need a component README audit. A frontend restructure doesn't need the
   discord-fingerprint skill checked.
3. **Read and compare** — For each doc in scope, read it and verify against the code.
4. **Fix silently** — Update stale sections without asking. Only flag things to the
   user if there's ambiguity (e.g., a README describes a pattern and you're not sure
   if the old or new way is intended).
5. **Report** — Output a short summary of what was updated and what was already current.

## Output format

```
## Docs Audit

| Document | Status |
|----------|--------|
| CLAUDE.md | current |
| src/lib/README.md | updated — added FooService to services section |
| src/lib/components/README.md | updated — added NewComponent, removed OldComponent |
| docs skill | current |
| memory | current |

N files updated, M already current.
```

Keep it brief. The user wants confidence that docs are synced, not a detailed changelog.
