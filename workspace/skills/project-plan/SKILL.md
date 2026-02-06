---
name: project-plan
description: Maintain a living project plan that tracks progress across sessions and guides what to build next
triggers:
  - project plan
  - what's next
  - where did we leave off
  - project status
  - track progress
  - roadmap
  - backlog
  - sprint
  - what's left
pipeline: []
---

# Project Plan Skill

## When to Use
- At the START of every session in a project directory — check PLAN.md for context
- After completing any milestone or significant piece of work — update PLAN.md
- When Justin asks "what's next" or "where are we"

## Process

### On Session Start (if PLAN.md exists)
1. Read `PLAN.md` from the project root
2. Summarize current status to Justin in 2-3 lines: what's done, what's in progress, what's next
3. Continue from the next uncompleted task

### On Session Start (if PLAN.md does NOT exist)
1. Check if `SPEC.md` exists — if so, generate PLAN.md from the spec's user stories
2. If no spec either, ask Justin what the project should accomplish

### Creating PLAN.md
Generate this structure in the project root:

```markdown
# [Project Name] — Plan

## Current Phase
[e.g., "Phase 1: Core API and Auth" or "MVP Build"]

## Progress
Last updated: YYYY-MM-DD

### Completed
- [x] Task description (YYYY-MM-DD)

### In Progress
- [ ] **CURRENT →** Task description
  - Sub-task 1
  - Sub-task 2

### Up Next
- [ ] Task description
- [ ] Task description

### Blocked / Needs Input
- [ ] Task description — BLOCKED: [reason]

## Architecture Decisions
| Decision | Choice | Date | Rationale |
|----------|--------|------|-----------|

## Session Log
| Date | What was accomplished | Files changed |
|------|----------------------|---------------|
```

### Updating PLAN.md
After completing work:
1. Move completed items from "In Progress" to "Completed" with today's date
2. Move the next item from "Up Next" to "In Progress" and mark it as **CURRENT**
3. Add a session log entry with what was accomplished
4. Add any new tasks discovered during implementation to "Up Next"
5. Record any architecture decisions made

### Rules
- **CURRENT** marker: Exactly one task should have this marker at all times
- Always update PLAN.md at the end of a session, even if work is incomplete — note where you stopped
- Tasks should be small enough to complete in one session (break large ones down)
- Architecture decisions are permanent record — don't delete them, add new ones
- Session log gives Justin (and future sessions) context on what happened

## Parallel Agent Coordination
When using `sortiarius agent parallel`:
1. Before launching agents, update PLAN.md to note which tasks are being parallelized
2. After agents complete, review outputs and update PLAN.md with results
3. Mark any integration work needed as a new task

## Cross-Session Memory
If you learn something project-specific during a session:
- Project-specific: Add it to PLAN.md under "Architecture Decisions"
- General/reusable: Write it to Sortiarius memory files via `sortiarius memory`

## Changelog
- 2026-02-06: Initial creation
