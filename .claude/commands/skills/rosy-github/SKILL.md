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

Field metadata is hardcoded below. These IDs rarely change. If any `edit_item` call
fails with an invalid field or option ID error, call `list_fields` once to refresh,
update your working memory with the new values, and retry.

**Project Number:** `7`
**Project ID:** `PVT_kwHOBXC3xM4BSd_s`

| Field | Field ID | Options (name = ID) |
|-------|----------|---------------------|
| Status | `PVTSSF_lAHOBXC3xM4BSd_szg__kR0` | Planning=`f75ad846`, Ready=`61e4505c`, In progress=`47fc9ee4`, In review=`df73e18b`, Done=`98236657` |
| Priority | `PVTSSF_lAHOBXC3xM4BSd_szg__kSU` | P0=`79628723`, P1=`0a877460`, P2=`da944a9c` |
| Size | `PVTSSF_lAHOBXC3xM4BSd_szg__kSY` | XS=`6c6483d2`, S=`f784b110`, M=`7515a9f1`, L=`817d0097`, XL=`db339eb2` |

**Status column meanings:**
- `Planning` — scope/criteria/approach still need refinement
- `Ready` — fully defined, unblocked, could be picked up today
- New issues default to `Planning` unless the interview produced a complete spec

**Priority/Size:** Ask during the interview; use the IDs above when writing.

**Assignees:** `@hiibolt` (and/or `@claude` when appropriate).

## Session Memory

To avoid redundant API calls within a single conversation:

- **Field metadata**: Always use the hardcoded Board Fields table. Only call `list_fields`
  if an `edit_item` fails (stale ID recovery).
- **Project ID**: Always use `PVT_kwHOBXC3xM4BSd_s`. Do NOT call `view_project`.
- **`get_me` result**: If `get_me` was called earlier, remember the username — do not
  call it again.

---

## MCP Tools Reference

This skill uses two MCP servers:

**GitHub MCP** (`github`) — for issues, PRs, and repo operations:
- `mcp__github__create_issue` — create issues
- `mcp__github__get_issue` — get issue details
- `mcp__github__update_issue` — update issues
- `mcp__github__add_issue_comment` — comment on issues
- `mcp__github__search_issues` — search issues
- `mcp__github__list_issues` — list open/closed issues
- `mcp__github__create_branch` — create branches
- `mcp__github__create_pull_request` — create PRs
- `mcp__github__get_pull_request` — read PR details
- `mcp__github__get_pull_request_files` — see changed files in PR
- `mcp__github__get_pull_request_status` — check PR CI/review status
- `mcp__github__get_pull_request_comments` — read PR comments
- `mcp__github__get_pull_request_reviews` — read PR reviews
- `mcp__github__create_pull_request_review` — review a PR
- `mcp__github__merge_pull_request` — merge PRs
- `mcp__github__list_pull_requests` — list PRs
- `mcp__github__update_pull_request_branch` — update PR branch

**GitHub Projects MCP** (`github-projects`) — for project board operations:
- `mcp__github-projects__list_projects` — find project number
- `mcp__github-projects__view_project` — get project ID
- `mcp__github-projects__list_fields` — get field IDs and option IDs
- `mcp__github-projects__list_items` — see current board state
- `mcp__github-projects__add_item` — add issue to board
- `mcp__github-projects__edit_item` — set field values (one per call)
- `mcp__github-projects__archive_item` — archive an item

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
- **Read issue**: `get_issue` — show title, body, state, assignees, labels
- **Update issue**: `update_issue` with the relevant fields
- **Close issue**: `update_issue` with `state: "closed"`
- **Comment on issue**: `add_issue_comment` — post the comment, confirm with link
- **Search issues**: `search_issues` with the user's query scoped to `hiibolt/rosy`

### Board Operations
- **Move item to a column**: use the Status field ID and option ID from the Board Fields table, then call `edit_item`
  - **Special: moving to "In Progress"** — when an item moves to In Progress, also:
    1. **Assign the issue**: call `update_issue` to add `hiibolt` as assignee (if not already assigned).
    2. **Create a feature branch**: call `create_branch` using the pattern `<issue-number>-<short-description>` from the default branch (`master`). Skip if a branch for this issue already exists.
    3. **Link branch to issue**: call `add_issue_comment` on the issue with a short comment noting the branch name (e.g. `**[Started]** Working on branch \`42-add-da-division\``).
    4. **Confirm** with: the new status, assignee, and branch name so the user can `git fetch && git checkout` immediately.
- **Change priority/size**: use the field IDs and option IDs from the Board Fields table, then call `edit_item`
- **Add existing issue to board**: `add_item` with the issue URL
- **Archive item**: `archive_item` with the item ID

### Branch & PR Operations
- **Create branch for issue**: `create_branch` — use pattern `<issue-number>-<short-description>`. All issue work must happen on a feature branch, never directly on `master`.
- **Create PR**: `create_pull_request` — set title, body, head branch, base branch (`master`). Always include `Closes #<issue-number>` in the PR body so GitHub auto-links and auto-closes the issue on merge. After creation, add to the project board via `add_item` and set Status to "In Review" using the hardcoded field IDs.
- **Check PR status**: `get_pull_request` or `get_pull_request_status`
- **Review PR**: `get_pull_request_files` to see changes, then `create_pull_request_review`
- **Merge PR**: `merge_pull_request` — confirm merge method with user first. Do **not** auto-set the board status to "Done" — leave that for the user or a separate explicit action.
- **List open PRs**: `list_pull_requests` with `state: "open"`

For all quick actions: execute, confirm the result with a link or summary, done.
If any call fails, report the error clearly.

---

## Live Issue Updates

When actively working on an issue (i.e. it's In Progress and you're writing code), keep
the issue thread alive as a development log using `add_issue_comment`. This gives
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
   - `mcp__github__search_issues` with relevant keywords, scoped to `hiibolt/rosy`
   - If likely duplicates exist, mention them conversationally before proceeding

4. **Field metadata**: Use the hardcoded values from the Board Fields table.
   Do NOT call `view_project` or `list_fields`.

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
- Offer your opinion: "I'd call this P1 since it blocks the operator completeness goal..."
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
```
mcp__github__create_issue
  owner: "hiibolt"
  repo: "rosy"
  title: <from Phase 2>
  body: <from Phase 2>
  assignees: [<agreed assignees>]
  labels: [<matching type: "enhancement", "bug", or "chore">]
```

**Step 2 — Add to project board:**
```
mcp__github-projects__add_item
  projectNumber: 7
  owner: "hiibolt"
  url: <issue URL from step 1>
```

**Step 3 — Set board fields** (one `edit_item` call per field):
```
mcp__github-projects__edit_item
  itemId: <from step 2 response>
  projectId: PVT_kwHOBXC3xM4BSd_s
  fieldId: <from Board Fields table>
  singleSelectOptionId: <from Board Fields table>
```

Each field is a separate call. Pass the value as a top-level parameter (not nested):
- **Single-select fields** (Status, Priority, Size): use `singleSelectOptionId`

Set in this order:
1. **Status** — using Board Fields table
2. **Priority** — using Board Fields table
3. **Size** — using Board Fields table

**Step 4 — Confirm:**
Report to the user:
- Issue URL
- Board placement summary (status, priority, size, assignees)
- One-line summary

**Error recovery:** If any `edit_item` call fails with an invalid field or option ID,
call `list_fields` once to refresh the metadata, update working memory, and retry.
For other failures, report clearly which step failed and offer to retry. Never silently
skip a field.

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
