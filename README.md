# Sortiarius

A personal AI assistant framework that augments [Claude Code](https://docs.anthropic.com/en/docs/claude-code) with persistent memory, enforced workflows, parallel agents, and cross-project learning.

## What It Does

Sortiarius wraps around Claude Code to add capabilities it doesn't have natively:

- **Persistent memory** — Knowledge split by domain (Azure, PowerShell, database, etc.) that carries across sessions
- **30 skills** — Autodiscovered via YAML frontmatter triggers, covering everything from product specs to deployment
- **13 hooks + 2 validators** — Deterministic enforcement that Claude can't override: safety guards, workflow gates, secret scanning, regression tracking, code validation
- **Agent teams** — Builder/validator pairs with task dependencies, self-validating commands, and native slash commands
- **Agent hierarchy** — 3-level system (super agent → project agents → task sub-agents) with persistent registry
- **Knowledge library** — Cross-project solutions, patterns, and anti-patterns that prevent reinventing the wheel
- **Evaluation system** — Acceptance criteria derived from specs, with automated checks and stop conditions for dev loops
- **10-step app building pipeline** — Spec → Plan → Architect → Build → Fix → Review → Test → Integrate → Ship → Commit
- **Web dashboard** — Local monitoring UI at `http://127.0.0.1:8420`

## Quick Start

```bash
git clone https://github.com/SudoSteiN/Sortiarius.git ~/Sortiarius
cd ~/Sortiarius
./setup.sh
source ~/.zshrc  # or ~/.bashrc
```

After setup, every `claude` session is Sortiarius — regardless of directory.

## How It Works

```
~/.claude/CLAUDE.md  →  symlink  →  ~/Sortiarius/CLAUDE.md
         ↓
Claude Code reads it automatically in every session
         ↓
Sortiarius identity + skills + memory + hooks always active
```

Project-specific `CLAUDE.md` files layer on top for project context.

## Commands

```bash
sortiarius                       # Open in current directory
sortiarius new <name>            # Create new project
sortiarius home                  # Manage skills/memory/hooks
sortiarius memory [domain]       # Edit memory (azure, powershell, etc.)
sortiarius sync                  # Commit workspace changes
sortiarius doctor                # Health check
sortiarius ui                    # Launch web dashboard
sortiarius worktree add <name>   # Create parallel dev worktree
sortiarius worktree aliases      # Get za/zb/zc aliases for fast switching
sortiarius agent run "task"      # Spawn autonomous agent
sortiarius agent parallel file   # Multiple parallel agents
sortiarius agent status          # Check all agents
sortiarius skill list            # List all skills + triggers
```

## Hook System

Hooks enforce rules as **code** — Claude can't ignore or bypass them.

| Hook | Event | Purpose |
|------|-------|---------|
| `session-start.sh` | SessionStart | Context injection (skills, memory, agents, PLAN.md) |
| `safety-bash.sh` | PreToolUse:Bash | Blocks destructive commands |
| `safety-files.sh` | PreToolUse:Edit/Write | Protects secrets and credentials |
| `dev-workflow.sh` | PreToolUse:Bash | Enforces conventional commits, branch rules |
| `workflow-guard.sh` | PreToolUse:Edit/Write | Blocks code without SPEC.md |
| `pre-push-guard.sh` | PreToolUse:Bash | Warns if tests not run before push |
| `dependency-guard.sh` | PreToolUse:Bash | Flags new packages, detects typosquats |
| `learning-tracker.sh` | PostToolUse:Bash | Logs commands + context health monitoring |
| `secret-scan.sh` | PostToolUse:Bash | Scans output for leaked credentials |
| `regression-guard.sh` | PostToolUse:Edit/Write | Tracks modifications, reminds to test |
| `session-stop.sh` | Stop | Self-audit, quality gate, workspace check |
| `session-learn.sh` | Stop | Suggests memory updates from session patterns |

### Validation Hooks (`.claude/hooks/validators/`)

| Hook | Used By | Purpose |
|------|---------|---------|
| `code_validator.sh` | Builder agent | Language-aware lint/type checks after Write/Edit |
| `validate_file_contains.sh` | Planner agent | Validates output file has required sections |

## Skills (30)

Skills are autodiscovered from `workspace/skills/*/SKILL.md`. No routing table — YAML frontmatter `triggers:` handle matching.

**Pipeline skills** (10-step app building):
`product-spec` · `project-plan` · `architecture` · `full-stack-dev` · `run-and-fix` · `code-review` · `testing` · `integration` · `deployment` · `github-workflow`

**System skills:**
`orchestrator` · `evaluation` · `coding-agent` · `tmux-orchestration` · `worktree-workflow`

**Productivity skills:**
`self-improve` · `prompt-playbook` · `data-analytics` · `learning-mode`

**Domain skills:**
`problem-modeling` · `azure-ops` · `powershell-automation` · `incident-response` · `contrastive-scoring` · `verify-response`

**Utility skills:**
`summarize` · `session-logs` · `healthcheck`

## Agent Teams & Task System

For significant builds, the task system orchestrates builder/validator agent pairs with dependency tracking:

```
/plan_w_team "Add OAuth2 support"
  → Planner generates specs/oauth2-plan.md (self-validated via Stop hook)

/build specs/oauth2-plan.md
  → Task 1: auth-builder (builder) → Task 2: auth-validator (validator)
  → Task 3: api-builder (builder) → Task 4: api-validator (validator)
  → Task 5: integration-builder → Task 6: integration-validator
```

**Agent definitions** at `.claude/agents/team/`:
- **builder** — Implements one task. PostToolUse hooks run code validation after every edit
- **validator** — Read-only verification. `disallowedTools: Write, Edit` — structurally cannot modify files
- **planner** — Creates specs only. `disallowedTools: Task` — cannot spawn agents

**Slash commands** at `.claude/commands/`:
- `/plan_w_team` — Self-validating planning with Stop hooks
- `/build` — Plan executor using TaskCreate/TaskUpdate with dependencies
- `/prime` — Read-only context loader (haiku model)

## Agent Hierarchy

```
Sortiarius (Level 0 — Super Agent)
├── Knowledge Library (cross-project patterns)
├── Project Agent: MyApp (Level 1)
│   ├── Builder: Backend API (Level 2) → Validator: Backend API
│   ├── Builder: Frontend UI (Level 2) → Validator: Frontend UI
│   └── Builder: Test Suite (Level 2) → Validator: Test Suite
└── Project Agent: OtherApp (Level 1)
    └── Builder/Validator pairs as needed
```

Agents communicate through the task system (TaskCreate/Update/List/Get) and files (SPEC.md, PLAN.md, knowledge library). The registry persists across sessions.

## Knowledge Library

Lives at `workspace/knowledge/`:
- **solutions.md** — Specific solutions with implementation details
- **patterns.md** — Architectural patterns (pre-seeded with REST, error handling, project structure)
- **components.md** — Reusable code with source references
- **anti-patterns.md** — Failed approaches and why they failed

Searched before building anything new. Updated after project milestones.

## Evaluation System

Development loops get clear stop conditions:

1. Generate **CRITERIA.md** from SPEC.md
2. Run automated checks (build, types, lint, tests, security)
3. Score: PASS (>=90%) / CONDITIONAL (80-89%) / ITERATE (60-79%) / ESCALATE (<60%)

## File Structure

```
~/Sortiarius/
├── CLAUDE.md                    # Brain (symlinked to ~/.claude/)
├── SPEC.md                      # Framework spec
├── README.md                    # This file
├── HOW-TO-USE.md                # Detailed usage guide
├── setup.sh                     # One-time setup
├── .claude/
│   ├── settings.json            # Hook configuration + agent teams env
│   ├── agents/team/             # Agent definitions (builder, validator, planner)
│   ├── commands/                # Native slash commands (/plan_w_team, /build, /prime)
│   └── hooks/                   # 13 enforcement scripts + 2 validators
├── bin/
│   ├── sortiarius               # Main CLI
│   └── sortiarius-agent         # Agent launcher
└── workspace/
    ├── memory/                  # Persistent domain knowledge
    ├── knowledge/               # Cross-project solutions library
    ├── templates/               # GitHub templates (PR, issues, CI)
    ├── skills/                  # 30 autodiscovered skills
    ├── ui/                      # Web dashboard (Python stdlib)
    └── scratch/                 # Session logs, agent registry (gitignored)
```

## Requirements

- [Claude Code CLI](https://docs.anthropic.com/en/docs/claude-code) (`claude` command)
- `jq` (used by hooks)
- `git`
- Python 3 (for web dashboard only)

## License

Personal project. Not licensed for redistribution.
