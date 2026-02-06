# Sortiarius

A personal AI assistant framework that augments [Claude Code](https://docs.anthropic.com/en/docs/claude-code) with persistent memory, enforced workflows, parallel agents, and cross-project learning.

## What It Does

Sortiarius wraps around Claude Code to add capabilities it doesn't have natively:

- **Persistent memory** — Knowledge split by domain (Azure, PowerShell, database, etc.) that carries across sessions
- **25 skills** — Autodiscovered via YAML frontmatter triggers, covering everything from product specs to deployment
- **13 hooks** — Deterministic enforcement that Claude can't override: safety guards, workflow gates, secret scanning, regression tracking
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

## Skills (25)

Skills are autodiscovered from `workspace/skills/*/SKILL.md`. No routing table — YAML frontmatter `triggers:` handle matching.

**Pipeline skills** (10-step app building):
`product-spec` · `project-plan` · `architecture` · `full-stack-dev` · `run-and-fix` · `code-review` · `testing` · `integration` · `deployment` · `github-workflow`

**System skills:**
`orchestrator` · `evaluation` · `coding-agent` · `tmux-orchestration`

**Domain skills:**
`problem-modeling` · `azure-ops` · `powershell-automation` · `incident-response` · `contrastive-scoring` · `verify-response`

**Utility skills:**
`summarize` · `session-logs` · `healthcheck`

## Agent Hierarchy

```
Sortiarius (Level 0 — Super Agent)
├── Knowledge Library (cross-project patterns)
├── Project Agent: MyApp (Level 1)
│   ├── Sub-agent: Backend API (Level 2)
│   ├── Sub-agent: Frontend UI (Level 2)
│   └── Sub-agent: Test Suite (Level 2)
└── Project Agent: OtherApp (Level 1)
    └── Sub-agents as needed
```

Agents communicate through files (SPEC.md, PLAN.md, output files, knowledge library). The registry persists across sessions.

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
│   ├── settings.json            # Hook configuration
│   └── hooks/                   # 13 enforcement scripts
├── bin/
│   ├── sortiarius               # Main CLI
│   └── sortiarius-agent         # Agent launcher
└── workspace/
    ├── memory/                  # Persistent domain knowledge
    ├── knowledge/               # Cross-project solutions library
    ├── skills/                  # 25 autodiscovered skills
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
