---
name: orchestrator
description: Super agent that manages project agents, knowledge library, and cross-project coordination
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
pipeline: [coding-agent]
---

# Orchestrator Skill — Agent Hierarchy

## Architecture

```
Sortiarius (Super Agent — Level 0)
├── Knowledge Library (cross-project patterns, reusable solutions)
├── Project Registry (all projects, their states, assigned agents)
│
├── Project Agent: ProjectA (Level 1)
│   ├── Context: SPEC.md + PLAN.md + codebase
│   ├── Sub-agent: Backend API (Level 2)
│   ├── Sub-agent: Frontend UI (Level 2)
│   └── Sub-agent: Test Suite (Level 2)
│
└── Project Agent: ProjectB (Level 1)
    ├── Context: SPEC.md + PLAN.md + codebase
    └── Sub-agents as needed
```

## Roles

### Level 0 — Super Agent (Sortiarius)
**You are always this agent.** Your responsibilities:
- Maintain the knowledge library at `~/Sortiarius/workspace/knowledge/`
- Track all projects in the project registry
- Route new requests to existing project agents or create new ones
- Detect cross-project reuse opportunities ("We solved this in ProjectA")
- Enforce workflow (hooks handle this deterministically)
- Aggregate learnings from project agents back into knowledge library

### Level 1 — Project Agent
A dedicated Claude instance (or session) that fully understands one project.
- Created via `sortiarius agent run` with full project context injected
- Has access to: project SPEC.md, PLAN.md, codebase, relevant knowledge entries
- Can spawn Level 2 sub-agents for parallel work
- Reports completions back via output files
- Updates knowledge library with new solutions

### Level 2 — Task Sub-Agent
Short-lived Claude instances for specific tasks within a project.
- Spawned by project agents via `sortiarius agent parallel`
- Single-purpose: write one module, run one test suite, research one API
- Reports results to its parent project agent
- Never spawns its own sub-agents (max depth = 2)

## When to Create a Project Agent

Create a new Level 1 project agent when:
- Starting a new application from scratch (goes through 10-step pipeline)
- The project is complex enough to need its own SPEC.md and PLAN.md
- Multiple sessions will be needed to complete the work
- The project directory is distinct (its own repo under `~/projects/`)

**Do NOT create a project agent for:**
- Quick scripts or one-off tasks (just do them directly)
- Bug fixes in existing projects (work within existing context)
- Research or investigation tasks (use Level 2 sub-agents)

## How to Start a New Project

When Justin says "build me an app" or "start a new project":

### Step 1: Check Knowledge Library
Before anything else, check if we've built something similar:

```bash
# Search the knowledge library
cat ~/Sortiarius/workspace/knowledge/index.md
cat ~/Sortiarius/workspace/knowledge/solutions.md
cat ~/Sortiarius/workspace/knowledge/patterns.md
```

If a relevant solution exists, reference it. Don't reinvent.

### Step 2: Register the Project

```bash
# Create the project directory
mkdir -p ~/projects/<project-name>
cd ~/projects/<project-name>
git init
```

Update the project registry:
```json
// ~/Sortiarius/workspace/scratch/project-registry.json
{
  "projects": [
    {
      "name": "project-name",
      "path": "~/projects/project-name",
      "status": "speccing",
      "created": "2026-02-06",
      "spec": false,
      "plan": false,
      "current_phase": "spec",
      "tech_stack": [],
      "agents_spawned": 0,
      "knowledge_entries": []
    }
  ]
}
```

### Step 3: Follow the Pipeline
Execute the 10-step pipeline from CLAUDE.md. The project agent owns this:

1. **Spec** → `product-spec` skill → generates SPEC.md
2. **Plan** → `project-plan` skill → generates PLAN.md
3. **Architect** → `architecture` skill → tech decisions
4. **Build** → `full-stack-dev` + `coding-agent` skills → parallel code gen
5. **Fix** → `run-and-fix` skill → iterative debugging
6. **Review** → `code-review` skill → quality check
7. **Test** → `testing` skill → test coverage
8. **Integrate** → `integration` skill → combine agent outputs
9. **Ship** → `deployment` skill → containerize, deploy
10. **Commit** → `github-workflow` skill → PR, CI

### Step 4: Spawn Sub-Agents for Phase 4 (Build)

This is where parallelism matters most. For the build phase:

```bash
# Create task file from PLAN.md
cat > /tmp/build-tasks.txt << 'EOF'
Create the database schema and models at src/models/ based on SPEC.md data model section
Create the API routes at src/routes/ implementing all endpoints from SPEC.md API contract
Create the frontend components at src/components/ matching SPEC.md UI screens
Create the authentication module at src/auth/ with JWT token handling
Generate unit tests at tests/ for all models and routes
EOF

# Spawn parallel agents
sortiarius agent parallel /tmp/build-tasks.txt
```

### Step 5: Report to Knowledge Library

After project completion or significant milestones, update the knowledge library:

```markdown
<!-- In ~/Sortiarius/workspace/knowledge/solutions.md -->
## JWT Authentication with Refresh Tokens
<!-- learned: 2026-02-06 -->
<!-- project: my-app -->
<!-- reuse: high -->

Pattern: Access token (15min) + refresh token (7d) stored in httpOnly cookie.
Implementation: src/auth/jwt.ts in my-app project.
Key files: src/auth/jwt.ts, src/middleware/auth.ts, src/routes/auth.ts
What worked: Sliding window refresh, token blacklist in Redis.
What didn't: Storing refresh token in localStorage (XSS risk).
```

## Cross-Project Reuse Protocol

Before building any significant component, the super agent MUST:

1. **Search knowledge library** for existing solutions
2. **Check patterns catalog** for established approaches
3. **Review similar project structures** if they exist

If a match is found:
- Reference the existing solution in the new project's PLAN.md
- Copy and adapt (don't rebuild from scratch)
- Note in knowledge library that this pattern was reused

If no match:
- Build it fresh
- After building, add to knowledge library for future reuse

## Agent Communication Protocol

Agents communicate through files, not messages:

| Direction | Mechanism |
|---|---|
| Super → Project | Project SPEC.md + PLAN.md + injected context from session-start.sh |
| Project → Sub-agent | Task description in `sortiarius agent run "..."` |
| Sub-agent → Project | Output file at `workspace/scratch/agent-<id>/output.md` |
| Project → Super | Knowledge library updates + project registry status |
| Super → Super (cross-session) | Memory files + knowledge library + project registry |

## Evaluation Integration

Every project agent must run the `evaluation` skill before declaring a phase complete.
See `~/Sortiarius/workspace/skills/evaluation/SKILL.md` for the criteria system.

The evaluation skill generates CRITERIA.md from SPEC.md. Phases gate on evaluation:
- Phase 4 (Build) → code compiles, no type errors
- Phase 5 (Fix) → all builds pass
- Phase 6 (Review) → no critical issues
- Phase 7 (Test) → coverage thresholds met
- Phase 8 (Integrate) → integration tests pass
- Phase 9 (Ship) → container builds and starts
- Phase 10 (Commit) → CI passes

## Changelog
- 2026-02-06: Initial creation — agent hierarchy design
