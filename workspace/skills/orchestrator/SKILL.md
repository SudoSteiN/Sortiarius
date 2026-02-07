---
name: orchestrator
description: Multi-agent orchestration using the Claude Code task system — builder/validator teams, task dependencies, and parallel execution
triggers:
  - start project
  - new project
  - manage agents
  - project agents
  - coordinate
  - orchestrate
  - delegate project
  - super agent
  - knowledge library
  - cross-project
  - reuse solution
  - what have we built before
  - build with team
  - task system
  - agent team
pipeline: [coding-agent]
---

# Orchestrator Skill — Multi-Agent Task System

## Core Paradigm: Plan → Build → Validate

Every significant piece of work follows a three-phase pattern:

1. **Plan** — A planner agent (or the super agent) creates a structured plan with tasks, dependencies, and team assignments
2. **Build** — Builder agents execute tasks in parallel where possible
3. **Validate** — Validator agents verify each builder's work (read-only, cannot modify)

This is the **builder/validator pair pattern**: every builder task has a corresponding validator task. Double the compute, double the trust.

## The Task System (TaskCreate/Update/List/Get)

The Claude Code task system replaces flat to-do lists with a dependency-aware work queue.

### Key Tools

| Tool | Purpose |
|------|---------|
| `TaskCreate` | Create a new task with subject, description, activeForm |
| `TaskUpdate` | Set status (pending/in_progress/completed), owner, dependencies |
| `TaskList` | View all tasks with current state |
| `TaskGet` | Get full details of a specific task |

### Task Dependencies

```
TaskUpdate({ taskId: "3", addBlockedBy: ["1", "2"] })
```
Task 3 cannot start until tasks 1 and 2 are both completed. When a blocking task is marked completed, dependent tasks automatically unblock.

### Deployment Pattern

```
1. Create ALL tasks via TaskCreate
2. Set dependencies via TaskUpdate (addBlockedBy)
3. Assign owners via TaskUpdate (owner)
4. Deploy builder agents for unblocked tasks (run_in_background: true)
5. As builders complete, deploy validator agents
6. Monitor via TaskList, react to completions
```

## Agent Definitions

Agent types are defined at `.claude/agents/team/`:

| Agent | File | Can Write | Purpose |
|-------|------|-----------|---------|
| **builder** | `builder.md` | Yes | Implements one task. Self-validates via PostToolUse hooks (lint, type check) |
| **validator** | `validator.md` | **No** (disallowedTools) | Verifies builder work. Read-only — structurally enforced |
| **planner** | `planner.md` | Plan only | Creates structured specs. Cannot spawn agents (Task tool disabled) |

### Builder/Validator Pair Example

```
Task 1: "Build auth module" → assigned to auth-builder (agent type: builder)
Task 2: "Validate auth module" → assigned to auth-validator (agent type: validator)
  → depends on Task 1
  → validator reads what builder produced
  → reports PASS or FAIL
```

## Native Slash Commands

| Command | What it does |
|---------|-------------|
| `/plan_w_team` | Create a plan with team orchestration (self-validating) |
| `/build` | Execute a plan file using the task system |
| `/prime` | Read-only context loading (codebase overview) |

### Using /plan_w_team

```
/plan_w_team Update the authentication system to support OAuth2
```

This deploys a **planner agent** that:
1. Analyzes the codebase
2. Creates a structured plan in `specs/`
3. Self-validates the plan has all required sections (Stop hook)
4. Cannot spawn agents — outputs plan only

### Using /build

```
/build specs/auth-oauth2-update.md
```

This deploys a **build orchestrator** that:
1. Reads the plan
2. Creates all tasks via TaskCreate
3. Sets dependencies
4. Deploys builder + validator agents
5. Monitors completion

## Architecture

```
Sortiarius (Super Agent — Level 0)
├── Knowledge Library (cross-project patterns)
├── Project Registry (all projects, states)
│
├── Project Agent: ProjectA (Level 1)
│   ├── /plan_w_team → specs/feature-x.md
│   ├── /build specs/feature-x.md
│   │   ├── Task 1: feature-builder (builder) ──→ Task 2: feature-validator
│   │   ├── Task 3: api-builder (builder) ──→ Task 4: api-validator
│   │   └── Task 5: integration-builder ──→ Task 6: integration-validator
│   └── PLAN.md updated with results
│
└── Project Agent: ProjectB (Level 1)
    └── Same pattern
```

## Self-Validation

Agents validate their own work at multiple levels:

1. **PostToolUse hooks on builder** — After every Write/Edit, code_validator.sh runs language-appropriate checks (lint, type check, syntax). Blocks the agent until fixed.
2. **Stop hooks on planner** — validate_file_contains.sh ensures plan has all required sections. Agent cannot stop until plan is complete.
3. **Validator agent** — Dedicated read-only agent verifies builder output after completion.

## Cross-Project Reuse Protocol

Before building any significant component:
1. Search `~/Sortiarius/workspace/knowledge/` for existing solutions
2. If a match exists, copy and adapt — don't rebuild
3. After building, add new solutions to the knowledge library

## When to Use Teams vs Sub-Agents vs Worktrees

| Approach | Best for | Communication |
|----------|----------|--------------|
| **Agent Teams** (task system) | Parallel tasks with dependencies, build+validate pairs | TaskCreate/Update shared state |
| **Sub-agents** (Task tool) | Quick research, single-purpose work | Return value to parent |
| **Worktrees** | Interactive parallel dev, separate features | Separate sessions, merge via PR |

## Changelog
- 2026-02-06: Initial creation — agent hierarchy design
- 2026-02-07: Rewrite with task system, builder/validator pairs, slash commands
