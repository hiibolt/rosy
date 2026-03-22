---
name: bigpicture
description: >
  Gather and display comprehensive project state: board items grouped by status,
  open PRs, recent git activity, and open issues. Use when the user asks "what's
  the status", "catch me up", "what should I work on", "what's on the board", or
  any request for project-level context. Also invoke proactively before creating
  issues, planning work, or when project context would inform your response.
---

# BigPicture — Project State Overview

Fetches and formats a comprehensive snapshot of the Rosy project's current state.
Designed for quick scanning, not raw data dumps.

## When to Use

- User asks about project status, board state, or what to work on next
- Before creating issues (rosy-github Phase 0 can reference this)
- When you need to understand how a task fits into the bigger picture
- User says "catch me up", "what's happening", "orient me", etc.

## Data Sources

Fetch all of these in parallel where possible:

### 1. Board State
```
mcp__github-projects__list_items
  projectNumber: 7
  owner: "hiibolt"
  limit: 100
```

### 2. Open Issues
```
mcp__github__list_issues
  owner: "hiibolt"
  repo: "rosy"
  state: "open"
  per_page: 50
```

### 3. Recent Git Activity
```bash
git log --oneline -10 master
```

### 4. Open PRs
```bash
gh pr list --repo hiibolt/rosy --state open --json number,title,headRefName,author,createdAt,baseRefName
```

### 5. Branch Overview
```bash
git branch -r --sort=-committerdate --format='%(refname:short) %(committerdate:relative)' | head -10
```

## Output Format

Format the gathered data into this structure. Omit empty sections. Use inline
annotations for quick scanning.

```markdown
# Rosy — Project Snapshot

## Board

### In Progress (N)
- #XX title — @assignee, P[0-2], [XS-XL]

### In Review (N)
- #XX title — @assignee, P[0-2], [XS-XL]

### Ready (N)
- #XX title — @assignee, P[0-2], [XS-XL]
  ⚠ unassigned items get flagged

### Planning (N)
- #XX title — P[0-2], [XS-XL]

## Open PRs (N)
- #XX branch ← base (@author, age)

## Recent Commits (master)
- sha7 message (age)
- ...

## Orphaned Issues
Issues that are open but NOT on the project board:
- #XX title
```

## Formatting Rules

- **Group board items by status column**, ordered: In Progress → In Review → Ready → Planning
- **Show count** in each section header
- **Inline metadata**: `— @assignee, P1, M` (priority + size as short codes)
- **Flag unassigned Ready items** with ⚠ — these need attention
- **Orphaned issues**: any open issue not found in the board items list
- **Skip Done column** — only show active work
- **Relative dates** for PRs and commits (e.g., "2 days ago")
- If a section has zero items, show the header with `(0)` and move on

## Error Handling

- If `github-projects` MCP is unavailable: fall back to `gh project item-list 7 --owner hiibolt --format json` via Bash
- If GitHub MCP is unavailable: fall back to `gh issue list --repo hiibolt/rosy --state open --json number,title,labels,assignees` via Bash
- If any fetch fails: show what you have, note what's missing
- Never fail silently — always report partial results with a note about what couldn't be fetched
