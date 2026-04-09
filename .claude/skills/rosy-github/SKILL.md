---
name: rosy-github
description: >
  Use this skill for ANY interaction with the hiibolt/rosy GitHub project — issues,
  PRs, branches, the project board, or any reference to a GitHub issue number (#N).
  This includes: creating, reading, updating, closing, or commenting on issues; managing
  the project board (moving items, changing status/priority/size); creating, reviewing,
  or merging PRs; managing branches; searching issues; and starting work on an issue.
  Triggers on: "#N" (any issue number reference), "start #N", "work on #N", "pick up #N",
  "begin #N", "what's #N", "new issue", "create issue", "plan feature", "open a ticket",
  "move to ready", "close issue", "update the board", "check PR", "what's the status of #N",
  "add a comment", "assign to", "change priority", "create a branch", "link PR", "open PR",
  "merge PR", "review PR", or any reference to GitHub issues, PRs, or the project board.
  Also triggers when the user describes a problem, idea, or change that should become a
  tracked work item — even casually phrased. When in doubt, trigger this skill.
---

# Rosy GitHub

The single skill for all GitHub interactions on `hiibolt/rosy`. Covers issue
creation, issue management, project board operations, PR workflows, and branch
management. Rosy is a transpiler from ROSY (a reimplementation of COSY INFINITY)
to Rust — issues often involve language constructs, type system work, or
transpiler internals.

## Core Philosophy

For **issue creation**: have a conversation, not an intake form. Cover everything,
feel like nothing.

For **everything else** (updates, board moves, PR ops): be fast and direct. The user
wants something done — do it, confirm it, move on.

## Team Reference

| Handle      | Role / Notes                                          |
|-------------|-------------------------------------------------------|
| `@hiibolt`  | Owner, sole maintainer, implementer                   |
| `@claude`   | You — review, scaffolding, implementation, suggestions|

Repo: `hiibolt/rosy`

## Board Fields (Rosy project)

Field metadata is hardcoded below. These IDs rarely change. If any `item-edit` call
fails with an invalid field or option ID error, run `gh project field-list 7 --owner hiibolt --format json`
once to refresh, update your working memory with the new values, and retry.

**Project Number:** `7`
**Project ID:** `PVT_kwHOBXC3xM4BSd_s`
**Repo Node ID:** `R_kgDOPgpY3w`

| Field | Field ID | Options (name = ID) |
|-------|----------|---------------------|
| Status | `PVTSSF_lAHOBXC3xM4BSd_szg__kR0` | Planning=`f75ad846`, Ready=`61e4505c`, In progress=`47fc9ee4`, Done=`98236657` |
| Priority | `PVTSSF_lAHOBXC3xM4BSd_szg__kSU` | Now=`79628723`, Soon=`0a877460`, Long-Term=`da944a9c` |
| Size | `PVTSSF_lAHOBXC3xM4BSd_szg__kSY` | Tiny=`6c6483d2`, Small=`f784b110`, Moderate=`7515a9f1`, Large=`817d0097`, X-Large=`db339eb2`, Gigantic=`8fa6379f` |

**Status column meanings:**
- `Planning` — scope/criteria/approach still need refinement
- `Ready` — fully defined, unblocked, could be picked up today
- New issues default to `Planning` unless the interview produced a complete spec

**Priority/Size:** Ask during the interview; use the IDs above when writing.

**Assignees:** `@hiibolt` (and/or `@claude` when appropriate).

## Labels (pre-fetched)

Use these label names directly with `--label` flags. IDs included for GraphQL if needed.

| Name | ID | Color | Description |
|------|----|-------|-------------|
| `bug` | `LA_kwDOPgpY388AAAACIJSjiw` | `d73a4a` | Something isn't working |
| `documentation` | `LA_kwDOPgpY388AAAACIJSjkg` | `0075ca` | Improvements or additions to documentation |
| `duplicate` | `LA_kwDOPgpY388AAAACIJSjlg` | `cfd3d7` | This issue or pull request already exists |
| `enhancement` | `LA_kwDOPgpY388AAAACIJSjmg` | `a2eeef` | New feature or request |
| `good first issue` | `LA_kwDOPgpY388AAAACIJSjpg` | `7057ff` | Good for newcomers |
| `help wanted` | `LA_kwDOPgpY388AAAACIJSjnw` | `008672` | Extra attention is needed |
| `invalid` | `LA_kwDOPgpY388AAAACIJSjrA` | `e4e669` | This doesn't seem right |
| `question` | `LA_kwDOPgpY388AAAACIJSjsw` | `d876e3` | Further information is requested |
| `wontfix` | `LA_kwDOPgpY388AAAACIJSjuA` | `ffffff` | This will not be worked on |
| `performance` | `LA_kwDOPgpY388AAAACcXbcJQ` | `ff9900` | Performance optimization |

## Session Memory

To avoid redundant API calls within a single conversation:

- **Field metadata**: Always use the hardcoded Board Fields table. Only run
  `gh project field-list` if an `item-edit` fails (stale ID recovery).
- **Project ID**: Always use `PVT_kwHOBXC3xM4BSd_s`. Do NOT run `gh project view`.
- **Labels**: Always use the hardcoded Labels table above. Do NOT run `gh label list`.

---

## CLI Tools Reference

All operations use the `gh` CLI (pre-installed). Run commands via the Bash tool.

### Issue Operations

| Operation | Command |
|-----------|---------|
| Create issue | `gh issue create --repo hiibolt/rosy --title "..." --body "..." --label "enhancement" --assignee hiibolt` |
| Get issue | `gh issue view <N> --repo hiibolt/rosy` |
| Get issue (JSON) | `gh issue view <N> --repo hiibolt/rosy --json title,body,state,labels,assignees,comments` |
| Update issue | `gh issue edit <N> --repo hiibolt/rosy [--title "..." --body "..." --add-label "..." --add-assignee "..."]` |
| Close issue | `gh issue close <N> --repo hiibolt/rosy` |
| Comment on issue | `gh issue comment <N> --repo hiibolt/rosy --body "..."` |
| Search issues | `gh search issues --repo hiibolt/rosy "<query>"` |
| List open issues | `gh issue list --repo hiibolt/rosy --state open --json number,title,labels,assignees` |

### Project Board Operations

| Operation | Command |
|-----------|---------|
| List board items | `gh project item-list 7 --owner hiibolt --format json --limit 100` |
| Add item to board | `gh project item-add 7 --owner hiibolt --url <issue-url> --format json` |
| Edit item field | `gh project item-edit --id <item-id> --project-id PVT_kwHOBXC3xM4BSd_s --field-id <field-id> --single-select-option-id <option-id>` |
| Archive item | `gh project item-archive 7 --owner hiibolt --id <item-id>` |
| Refresh field metadata | `gh project field-list 7 --owner hiibolt --format json` |

**Note:** `item-edit` sets one field per call. To set Status + Priority + Size, make 3 calls.
The `item-add` command returns JSON with the item ID when `--format json` is used.

### Branch & PR Operations

| Operation | Command |
|-----------|---------|
| Create branch | `git checkout -b <branch-name> master && git push -u origin <branch-name>` |
| Create PR | `gh pr create --repo hiibolt/rosy --title "..." --body "..." --base master --head <branch>` |
| Get PR | `gh pr view <N> --repo hiibolt/rosy` |
| Get PR (JSON) | `gh pr view <N> --repo hiibolt/rosy --json title,body,state,files,reviews,comments,statusCheckRollup` |
| PR changed files | `gh pr diff <N> --repo hiibolt/rosy` |
| PR status/checks | `gh pr checks <N> --repo hiibolt/rosy` |
| Review PR | `gh pr review <N> --repo hiibolt/rosy --approve --body "..."` (or `--request-changes` / `--comment`) |
| Merge PR | `gh pr merge <N> --repo hiibolt/rosy [--squash\|--merge\|--rebase]` |
| List open PRs | `gh pr list --repo hiibolt/rosy --state open --json number,title,headRefName,author,createdAt` |
| Update PR branch | `gh pr update-branch <N> --repo hiibolt/rosy` |

---

## Routing

Determine which workflow to use based on the user's request:

| Request type | Workflow |
|-------------|----------|
| Create a new issue / plan a feature / report a bug | **Issue Creation** (Phases 0–3 below) |
| Everything else (update, comment, close, move, branch, PR, search) | **Quick Actions** (see below) |

---

## Quick Actions

For non-creation operations, skip the interview. Just do the thing and confirm.

### Issue Operations
- **Read issue**: `gh issue view` — show title, body, state, assignees, labels
- **Update issue**: `gh issue edit` with the relevant flags
- **Close issue**: `gh issue close`
- **Comment on issue**: `gh issue comment` — post the comment, confirm with link
- **Search issues**: `gh search issues --repo hiibolt/rosy`

### Board Operations
- **Move item to a column**: use the Status field ID and option ID from the Board Fields table, then call `item-edit`
  - **Special: moving to "In progress"** — when an item moves to In progress, also:
    1. **Assign the issue**: `gh issue edit <N> --repo hiibolt/rosy --add-assignee hiibolt` (if not already assigned).
    2. **Create a feature branch**: `git checkout -b <issue-number>-<short-description> master && git push -u origin <branch>`. Skip if a branch for this issue already exists.
    3. **Link branch to issue**: `gh issue comment <N> --repo hiibolt/rosy --body '**[Started]** Working on branch \`<branch-name>\`'`
    4. **Confirm** with: the new status, assignee, and branch name so the user can `git fetch && git checkout` immediately.
- **Change priority/size**: use the field IDs and option IDs from the Board Fields table, then call `item-edit`
- **Add existing issue to board**: `gh project item-add 7 --owner hiibolt --url <issue-url>`
- **Archive item**: `gh project item-archive 7 --owner hiibolt --id <item-id>`

### Branch & PR Operations
- **Create branch for issue**: use pattern `<issue-number>-<short-description>`. All issue work must happen on a feature branch, never directly on `master`.
- **Create PR**: `gh pr create` — set title, body, head branch, base branch (`master`). Always include `Closes #<issue-number>` in the PR body so GitHub auto-links and auto-closes the issue on merge. After creation, add to the project board via `gh project item-add` and set Status to "In progress" using the hardcoded field IDs.
- **Check PR status**: `gh pr view` or `gh pr checks`
- **Review PR**: `gh pr diff` to see changes, then `gh pr review`
- **Merge PR**: `gh pr merge` — confirm merge method with user first. Do **not** auto-set the board status to "Done" — leave that for the user or a separate explicit action.
- **List open PRs**: `gh pr list --state open`

For all quick actions: execute, confirm the result with a link or summary, done.
If any call fails, report the error clearly.

---

## Live Issue Updates

When actively working on an issue (i.e. it's In Progress and you're writing code), keep
the issue thread alive as a development log using `gh issue comment`. This gives
visibility without anyone having to ask "how's it going?"

### When to Comment

| Trigger | What to post |
|---------|-------------|
| **Starting work** | Branch name, initial approach/plan, files you expect to touch |
| **Major milestone reached** | What was completed, what's next |
| **Approach change** | Why the original plan didn't work, what you're doing instead |
| **Blocker hit** | What's blocking, what you've tried, whether you need input |
| **PR opened** | Link to the PR, brief summary of what's included |
| **Work complete** | Final summary: what was done, any follow-up items or tech debt noted |

### Tone & Format

Keep comments **short and scannable**. Use this rough format:

```
**[Status]** Brief headline

- Bullet points with details
- Keep to 2-4 bullets max
```

Status tags: `[Started]`, `[Progress]`, `[Milestone]`, `[Pivot]`, `[Blocked]`, `[PR Ready]`, `[Done]`

**Do not** narrate every file edit or minor refactor. Comment at meaningful boundaries —
when something is *done*, *changed*, or *stuck*.

### Commit Cadence

While working on an issue, **commit frequently** — don't let large amounts of work
accumulate uncommitted. Follow these guidelines:

- **Commit after each logical unit of work**: a new AST node, a completed operator
  implementation, a test passing. Roughly every 15-30 minutes of active coding.
- **Commit before pivoting**: if you're about to change approach or move to a different
  part of the task, commit what you have first.
- **Commit before anything risky**: about to refactor something that might break? Commit
  the working state first.
- **Use descriptive commit messages** that reference the issue number (e.g. `#42: implement
  DA division operator`).
- **Never go more than ~30 minutes of coding without a commit.**

Proactively ask the user if they'd like to commit when you've completed a meaningful
chunk of work — don't wait for them to remember.

---

## Issue Creation Workflow

### Phase 0 — Read the Room (silent)

Before speaking, do these steps:

1. **Classify** the issue as **feature**, **bug**, or **chore/refactor**. Load the
   matching template from `templates/`.

2. **Gauge readiness**: does the user already have a fully-formed spec, or are they
   still figuring it out?
   - **Fast-track**: If they arrive with a clear goal, acceptance criteria, and enough
     detail to fill the template — skip the interview. Go straight to Phase 2, draft
     the issue, and confirm.
   - **Standard**: If they're still thinking, start the conversation (Phase 1).

3. **Scan for duplicates**:
   - `gh search issues --repo hiibolt/rosy "<keywords>"`
   - If likely duplicates exist, mention them conversationally before proceeding

4. **Field metadata**: Use the hardcoded values from the Board Fields table.
   Do NOT run `gh project view` or `gh project field-list`.

Then begin. Don't announce the phases. Just talk.

### Phase 1 — The Interview

Your goal is to draw out a fully specified work item through natural dialogue.
Internally track what you still need — but never recite the list.

**Required for all issue types:**
- One-sentence goal
- Acceptance criteria (checkboxes — how do we know it's done?)
- At least one test case (happy path + one edge case)
- Priority (relative to current board state)
- Size (rough effort signal)
- Status column: `Planning` or `Ready`

**Additionally required by type:**

| Feature | Bug | Chore |
|---------|-----|-------|
| Why now / motivation | Steps to reproduce | What & why |
| Concrete deliverables | Expected vs actual behavior | Approach / plan |
| Out of scope | Environment / context | Behavior changes (if any) |

**Nice-to-have (capture if mentioned, don't force):**
- Dependencies or risks
- Related issues
- COSY INFINITY compatibility notes

**Rosy-specific considerations:**
- Does this affect the PEG grammar (`rosy.pest`)?
- Does this add a new AST node? Which trait impls are needed?
- Does this need new TypeRule registries or IntrinsicTypeRule entries?
- Does the TDD rule apply (COSY/ROSY output diffing)?
- Is there a relevant COSY manual chapter to reference?

**Tone:**
- Make creative suggestions — "What about the edge case where the DA order is 0?"
- Notice what's missing and ask sideways — "So if someone passes a complex number here, what should happen?"
- Offer your opinion: "I'd call this Soon priority since it blocks the operator completeness goal..."
- Keep it technical but fun. We're reimagining a 30-year-old language.

**Exit condition:** When you have a complete picture, present a draft summary and ask:
*"Does this capture it? Anything to add or change before I make it real?"*

Read `references/quality-checklist.md` silently before presenting the summary. If any
quality gate is red, address it conversationally — don't dump the checklist.

### Phase 2 — Structured Draft

Format the issue body using the loaded template from `templates/`. Show the user:
- The title (using the template's title format)
- The full body

One final confirmation: *"Look good?"*

### Phase 3 — Act

Once confirmed, execute these steps in order:

**Step 1 — Create the issue:**
```bash
gh issue create --repo hiibolt/rosy \
  --title "<from Phase 2>" \
  --body "<from Phase 2>" \
  --label "<matching type: enhancement, bug, etc.>" \
  --assignee "<agreed assignees>"
```

**Step 2 — Add to project board:**
```bash
gh project item-add 7 --owner hiibolt --url <issue-url> --format json
```
Parse the item ID from the JSON output.

**Step 3 — Set board fields** (one `item-edit` call per field):
```bash
gh project item-edit \
  --id <item-id> \
  --project-id PVT_kwHOBXC3xM4BSd_s \
  --field-id <from Board Fields table> \
  --single-select-option-id <from Board Fields table>
```

Set in this order:
1. **Status** — using Board Fields table
2. **Priority** — using Board Fields table
3. **Size** — using Board Fields table

**Step 4 — Confirm:**
Report to the user:
- Issue URL
- Board placement summary (status, priority, size, assignees)
- One-line summary

**Error recovery:** If any `item-edit` call fails with an invalid field or option ID,
run `gh project field-list 7 --owner hiibolt --format json` once to refresh the metadata,
update working memory, and retry. For other failures, report clearly which step failed
and offer to retry. Never silently skip a field.

---

## Reference Files

- `references/quality-checklist.md` — Quality gates; read silently before exiting Phase 1
- `templates/feature.md` — Issue body template for features
- `templates/bug.md` — Issue body template for bugs
- `templates/chore.md` — Issue body template for chores/refactors

## What Good Looks Like

**For issue creation:** A great issue should be passable directly to someone with
zero ROSY context and they should know exactly what to build, how to test it, and
when it's done. If it couldn't do that, go back to Phase 1.

**For quick actions:** Fast, correct, confirmed. Execute the operation, report the
result with a link, move on. No ceremony needed.
