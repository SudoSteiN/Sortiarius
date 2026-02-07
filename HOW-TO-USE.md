# Sortiarius - How to Use

## How It Works

Sortiarius lives globally on your machine. After setup, **every** `claude` session is Sortiarius - regardless of what directory you're in. No API keys, no external services.

```
~/.claude/CLAUDE.md  →  symlink  →  ~/Sortiarius/CLAUDE.md
         ↓
Claude Code reads it automatically in every session
         ↓
Sortiarius identity + skills + memory + hooks always available
```

When you're in a specific project, that project's local `CLAUDE.md` layers on top - adding project context without replacing Sortiarius.

**What makes this intelligent:** Hooks enforce safety rules deterministically (code, not suggestions). Session logging tracks patterns. Autonomous agents run parallel tasks. Memory persists across sessions. The system gets smarter over time.

---

## Setup (One Time)

```bash
git clone https://github.com/SudoSteiN/Sortiarius.git ~/Sortiarius
cd ~/Sortiarius
./setup.sh
source ~/.zshrc   # or ~/.bashrc / ~/.bash_profile
```

The setup script will:
1. Verify prerequisites (git, claude CLI)
2. Symlink `~/.claude/CLAUDE.md` to the repo (backs up existing if any)
3. Add the `sortiarius` command to your PATH
4. Create `~/projects/` for new projects

---

## Daily Use

### Open Sortiarius anywhere
```bash
sortiarius              # Current directory
claude             # Same - Sortiarius identity loads globally
```

### Create a new project
```bash
sortiarius new my-terraform-module
```
Creates `~/projects/my-terraform-module/` with a project-local `CLAUDE.md` template and opens Sortiarius in it.

### Manage Sortiarius itself
```bash
sortiarius home         # Open in ~/Sortiarius to edit skills, memory, config
sortiarius memory       # Quick-edit memory index in your $EDITOR
sortiarius memory azure # Edit azure-specific memory
sortiarius sync         # Preview and commit workspace changes to git
sortiarius update       # Pull latest changes from remote
sortiarius doctor       # Check framework health (including hooks)
```

---

## The Two-Layer System

```
Layer 1 (Global):  ~/.claude/CLAUDE.md → ~/Sortiarius/CLAUDE.md
                   Always active. Sortiarius identity, skills, domain rules.

Layer 2 (Local):   ~/projects/my-app/CLAUDE.md
                   Project-specific context layered on top.
```

**Example:** Working in a Terraform project. Sortiarius's global rules (Azure safety, PowerShell standards) apply automatically. The project's local CLAUDE.md adds: "This project manages the East region infrastructure using Terraform 1.7."

---

## All Commands

| Command | What it does |
|---------|-------------|
| `sortiarius` | Open Sortiarius in current directory |
| `sortiarius new <name>` | Create new project at ~/projects/\<name\> |
| `sortiarius home` | Open Sortiarius home (manage skills/memory) |
| `sortiarius memory [file]` | Edit memory file in $EDITOR (default: index) |
| `sortiarius sync [--all]` | Preview and commit workspace changes |
| `sortiarius doctor` | Check framework health (symlink, skills, memory, hooks) |
| `sortiarius skill new <name>` | Scaffold a new skill with template |
| `sortiarius skill list` | List all skills with their triggers |
| `sortiarius agent run <prompt>` | Run an autonomous Claude agent |
| `sortiarius agent bg <prompt>` | Run agent in background |
| `sortiarius agent parallel <file>` | Run multiple agents from task file |
| `sortiarius agent status` | Check background agent status |
| `sortiarius agent review [dir]` | Review agent output |
| `sortiarius agent digest` | Analyze session logs for patterns |
| `sortiarius agent cancel <id>` | Stop a running agent |
| `sortiarius agent cleanup` | Prune dead/completed agents from registry |
| `sortiarius worktree add <name>` | Create git worktree + branch for parallel Claude session |
| `sortiarius worktree ls` | List active worktrees |
| `sortiarius worktree rm <name>` | Remove a worktree |
| `sortiarius worktree prune` | Clean up stale/merged worktrees |
| `sortiarius worktree aliases` | Print shell aliases (za, zb, zc) for quick worktree switching |
| `sortiarius ui [--port N]` | Launch web dashboard (default port 8420) |
| `sortiarius list` | List all projects in ~/projects/ |
| `sortiarius update` | Pull latest Sortiarius from git |
| `sortiarius uninstall` | Remove global config (keeps repo) |
| `sortiarius help` | Show all commands |
| `sortiarius <path>` | Open Sortiarius in a specific directory |

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SORTIARIUS_HOME` | `~/Sortiarius` | Sortiarius repo location |
| `SORTIARIUS_PROJECTS` | `~/projects` | Where `sortiarius new` creates projects |
| `EDITOR` | `vim` | Editor for `sortiarius memory` |

---

## Hooks (Deterministic Enforcement)

Hooks are the key innovation. They run **as code** before/after Claude's actions — Claude can't ignore them.

### How hooks work

Hooks are configured in `.claude/settings.json` and live as shell scripts in `.claude/hooks/`. They fire automatically on specific Claude Code events.

### Active hooks (13 total)

| Hook | Event | What it enforces |
|------|-------|-----------------|
| `session-start.sh` | SessionStart | Injects skill manifest + memory + agent status + project context (PLAN.md current task) |
| `safety-bash.sh` | PreToolUse:Bash | Blocks: `rm -rf`, `DROP TABLE`, `DELETE` without `WHERE`, `Remove-Az*` without `-WhatIf`, Azure resource deletion, production config writes |
| `safety-files.sh` | PreToolUse:Edit/Write | Blocks: edits to `.env`, credentials, `.git/`, SSH keys |
| `dev-workflow.sh` | PreToolUse:Bash | Enforces conventional commit format, blocks direct commits to main, blocks force push |
| `workflow-guard.sh` | PreToolUse:Edit/Write | Blocks writing code without SPEC.md (spec-before-code), warns if PLAN.md missing or stale |
| `pre-push-guard.sh` | PreToolUse:Bash | Warns when pushing if tests exist but weren't run this session |
| `dependency-guard.sh` | PreToolUse:Bash | Warns on new package installs, detects typosquat package names |
| `learning-tracker.sh` | PostToolUse:Bash | Logs commands + context health monitoring (warns at 30/60 ops, high error rate) |
| `secret-scan.sh` | PostToolUse:Bash | Scans command output for leaked credentials (API keys, tokens, connection strings) |
| `regression-guard.sh` | PostToolUse:Edit/Write | Tracks code modifications, reminds to re-run tests at thresholds |
| `session-stop.sh` | Stop | Self-audit + quality gate (tests/build run?) + workspace check + knowledge library reminder |
| `session-learn.sh` | Stop | Analyzes session patterns and suggests memory updates |

### What gets blocked (examples)

```bash
# These will be BLOCKED by safety-bash.sh:
rm -rf /some/path              # Recursive force-delete
DELETE FROM users              # DELETE without WHERE
Remove-AzVM -Name "prod-vm"   # Remove-Az* without -WhatIf
az vm delete --name prod-vm    # Azure resource deletion
DROP TABLE production_data     # DROP TABLE

# These are ALLOWED:
rm file.txt                    # Single file delete (not recursive force)
DELETE FROM users WHERE id=5   # DELETE with WHERE clause
Remove-AzVM -Name "test" -WhatIf  # Has -WhatIf
Get-AzVM                       # Read-only Azure operation
```

### Editing hooks

```bash
sortiarius home    # Opens Sortiarius repo in Claude
# Edit .claude/hooks/*.sh
# Edit .claude/settings.json to add/remove hooks
sortiarius sync    # Commit changes
```

---

## Autonomous Agents

Sortiarius can spawn independent Claude instances for parallel work.

### Run a one-shot agent
```bash
sortiarius agent run "Generate a PowerShell script to audit all Key Vault access policies"
```

### Run in background
```bash
sortiarius agent bg "Analyze our Azure resource tags for compliance"
sortiarius agent status   # Check progress
sortiarius agent review   # Read output when done
```

### Run multiple agents in parallel
Create a task file (`tasks.txt`):
```
# One task per line
Generate rollback script for database migration
Audit Key Vault access policies across all resource groups
Write incident response runbook for SQL AG failover
```

```bash
sortiarius agent parallel tasks.txt
```

All three agents run simultaneously. Results are saved to `workspace/scratch/agents/`.

### Analyze session patterns
```bash
sortiarius agent digest
```
Reads the session command log (built by the learning-tracker hook) and uses Claude to identify:
- Repeated command patterns (candidates for automation)
- Failure patterns (things that keep going wrong)
- Suggested memory updates
- Suggested skill improvements

---

## Task System & Agent Teams

For significant builds, Sortiarius uses Claude Code's task system with builder/validator agent pairs. This gives dependency-aware work queues and double-verification.

### How it works

1. **Plan** — Use `/plan_w_team <description>` to generate a structured spec in `specs/` with tasks, dependencies, and team assignments. A Stop hook validates the plan has all required sections before the agent can finish.
2. **Build** — Use `/build specs/my-plan.md` to execute the plan. This creates all tasks via TaskCreate, sets dependencies, and deploys builder+validator agent pairs.
3. **Validate** — Each builder task gets a corresponding read-only validator that verifies the work.

### Agent Definitions (`.claude/agents/team/`)

| Agent | Can Write | Purpose |
|-------|-----------|---------|
| **builder** | Yes | Implements one task. Self-validates via PostToolUse hooks (lint, type check after every edit) |
| **validator** | **No** | Verifies builder work. `disallowedTools: Write, Edit, NotebookEdit` — structurally read-only |
| **planner** | Plan only | Creates structured specs. `disallowedTools: Task` — cannot spawn agents |

### Native Slash Commands (`.claude/commands/`)

| Command | What it does |
|---------|-------------|
| `/plan_w_team` | Self-validating planning command. Stop hook ensures plan has Objective, Tasks, Team Orchestration sections |
| `/build` | Reads a plan file, creates tasks, deploys builder+validator pairs with dependencies |
| `/prime` | Read-only context loader (haiku model). Reads codebase structure, docs, config |

### Self-Validation Hooks (`.claude/hooks/validators/`)

| Hook | Used By | Purpose |
|------|---------|---------|
| `code_validator.sh` | Builder agent | Runs language-appropriate checks after Write/Edit (Python/ruff, Rust/cargo, TypeScript/tsc, Shell/bash -n). Blocks until fixed |
| `validate_file_contains.sh` | Planner/plan_w_team | Validates output file exists with required sections. Forces agent to continue until complete |

### When to use

- **Task system** (`/plan_w_team` + `/build`): 3+ parallel tasks, needs verification
- **Sub-agents** (`Task` tool): Quick research, single-purpose work
- **Worktrees**: Interactive parallel dev, long-running features

---

## The Learning Loop

Sortiarius gets smarter over time through this cycle:

```
1. You work with Sortiarius
         ↓
2. learning-tracker.sh logs every command (domain, success/failure)
         ↓
3. session-stop.sh reminds you about unsaved changes
         ↓
4. session-learn.sh analyzes the session and suggests memory updates
         ↓
5. You update memory files (sortiarius memory azure, etc.)
         ↓
6. sortiarius sync commits the updates
         ↓
7. session-start.sh loads updated memory into the next session
         ↓
8. Sortiarius has more context → better responses → repeat
```

Periodically run `sortiarius agent digest` to mine deeper patterns from the accumulated session logs.

---

## Knowledge Library

Cross-project solutions, patterns, and reusable components live at `~/Sortiarius/workspace/knowledge/`:

| File | Purpose |
|------|---------|
| `index.md` | Overview, project cross-reference, tag index |
| `solutions.md` | Specific solutions with implementation details |
| `patterns.md` | Architectural patterns and design decisions |
| `components.md` | Reusable code components with source references |
| `anti-patterns.md` | Things that didn't work and why |

**Always searched before building something new.** The session-stop hook reminds you to update it after significant sessions. Solutions are tagged (auth, api, db, ui, deploy, test, security, perf, infra) and rated (HIGH/MEDIUM/LOW reuse).

---

## Evaluation System

Development loops get clear stop conditions via `~/Sortiarius/workspace/skills/evaluation/SKILL.md`:

1. **CRITERIA.md** — Auto-generated from SPEC.md with checkable acceptance criteria per category (Build, Functional, Test, API, UI, Security, Deployment, Performance)
2. **Automated checks** — Build, types, lint, tests, security audit
3. **Scoring** — 0-100% with phase-specific thresholds
4. **Gate decisions**:
   - `>= 90%` → PASS (proceed to next phase)
   - `80-89%` → CONDITIONAL PASS (proceed, log gaps)
   - `60-79%` → ITERATE (continue dev loop)
   - `< 60%` → ESCALATE (stop, discuss approach)

---

## Skills

Skills live at `~/Sortiarius/workspace/skills/` and are **autodiscovered**. Each skill has a `SKILL.md` with YAML frontmatter containing triggers. Claude matches requests to skills automatically — no routing table to maintain.

### List installed skills
```bash
sortiarius skill list
```

### Add a new skill
```bash
sortiarius skill new my-skill-name
```

This scaffolds `~/Sortiarius/workspace/skills/my-skill-name/SKILL.md` with the standard template.

### Skill YAML fields

| Field | Required | Purpose |
|-------|----------|---------|
| `name` | Yes | Skill identifier |
| `description` | Yes | What the skill does |
| `triggers` | Yes | Keywords that activate the skill |
| `pipeline` | No | Other skills to run first (e.g., `[problem-modeling]`) |

---

## Memory

Memory is split by domain under `~/Sortiarius/workspace/memory/`:

| File | Contents |
|------|----------|
| `index.md` | Quick-reference, cross-domain notes, preferences |
| `azure.md` | Subscription, resource groups, Key Vaults, SQL servers |
| `powershell.md` | Script patterns, module notes, preferences |
| `incidents.md` | Past incidents, resolutions, post-mortems |
| `solutions.md` | Reusable solutions from past problems |

### Edit memory
```bash
sortiarius memory          # Opens index.md
sortiarius memory azure    # Opens azure.md
sortiarius memory incidents # Opens incidents.md
```

### Timestamp convention
Use `<!-- learned: YYYY-MM-DD -->` on entries so stale knowledge can be identified and pruned.

**Important:** Memory does NOT auto-sync. Run `sortiarius sync` at the end of productive sessions, or when Sortiarius reminds you (the Stop hook will nudge you).

---

## Health Checks

```bash
sortiarius doctor
```

Checks:
- Global symlink (`~/.claude/CLAUDE.md`) is correct
- All skills have valid YAML frontmatter with `name:` and `triggers:`
- Memory directory exists with expected files
- Git repo and remote are configured
- `sortiarius` and `claude` commands are in PATH
- `.claude/settings.json` is valid JSON with correct hook count
- All hook scripts are executable
- Agent launcher exists

---

## Uninstalling

```bash
sortiarius uninstall
```

This removes the `~/.claude/CLAUDE.md` symlink and restores any backup. The repo stays intact. To fully remove, also delete `~/Sortiarius` and remove the PATH line from your shell profile.

---

## File Structure

```
~/Sortiarius/                              # Home base
├── CLAUDE.md                              # Brain (symlinked to ~/.claude/)
├── SPEC.md                                # Framework spec
├── README.md                              # Repo overview
├── HOW-TO-USE.md                          # This file
├── setup.sh                               # One-time global setup
├── .gitignore                             # Ignores secrets, scratch, IDE files
├── .claude/
│   ├── settings.json                      # Hook configuration + agent teams env
│   ├── agents/team/                       # Agent definitions for task system
│   │   ├── builder.md                     # Focused implementation agent (self-validates)
│   │   ├── validator.md                   # Read-only verification agent
│   │   └── planner.md                     # Planning agent (no agent spawning)
│   ├── commands/                          # Native slash commands
│   │   ├── plan_w_team.md                 # Self-validating team planning (/plan_w_team)
│   │   ├── build.md                       # Plan executor with task system (/build)
│   │   └── prime.md                       # Read-only context loader (/prime)
│   └── hooks/                             # 13 enforcement scripts + 2 validators
│       ├── session-start.sh               # Context injection (skills, memory, agents, PLAN.md)
│       ├── safety-bash.sh                 # Block dangerous bash commands
│       ├── safety-files.sh                # Protect sensitive files
│       ├── dev-workflow.sh                # Enforce conventional commits, branch rules
│       ├── workflow-guard.sh              # Enforce spec-before-code
│       ├── pre-push-guard.sh             # Warn if tests not run before push
│       ├── dependency-guard.sh            # Flag new packages, detect typosquats
│       ├── learning-tracker.sh            # Log commands + context health monitoring
│       ├── secret-scan.sh                 # Scan output for leaked credentials
│       ├── regression-guard.sh            # Track code mods, remind to test
│       ├── session-stop.sh                # Self-audit + quality gate
│       ├── session-learn.sh               # Session analysis + memory suggestions
│       └── validators/                    # Validation scripts for agent hooks
│           ├── code_validator.sh          # Language-aware lint/type checks
│           └── validate_file_contains.sh  # Section presence validation
├── bin/
│   ├── sortiarius                         # Main CLI
│   └── sortiarius-agent                   # Agent launcher with persistent registry
└── workspace/
    ├── memory/                            # Persistent knowledge (split by domain)
    │   ├── index.md                       # Quick-reference and preferences
    │   ├── azure.md                       # Azure environment details
    │   ├── powershell.md                  # Script patterns and preferences
    │   ├── incidents.md                   # Past incidents and lessons
    │   └── solutions.md                   # Reusable solutions
    ├── knowledge/                         # Cross-project solution library
    │   ├── index.md                       # Overview, tags, project cross-reference
    │   ├── solutions.md                   # Specific solutions with details
    │   ├── patterns.md                    # Architectural patterns
    │   ├── components.md                  # Reusable code components
    │   └── anti-patterns.md               # Failed approaches and lessons
    ├── ui/                                # Web dashboard
    │   ├── server.py                      # Python stdlib HTTP server
    │   └── index.html                     # Single-page dashboard
    ├── scratch/                           # Temp files (gitignored)
    │   ├── session-log.jsonl              # Command log (from learning-tracker hook)
    │   ├── agent-registry.json            # Persistent agent registry
    │   └── agents/                        # Agent output directories
    ├── templates/                         # Reusable GitHub templates
    │   └── github/
    │       ├── pull_request_template.md   # PR template
    │       ├── ISSUE_TEMPLATE/            # Bug + feature templates
    │       └── workflows/ci.yml           # CI workflow (multi-stack)
    └── skills/                            # 30 autodiscovered skills
        ├── product-spec/SKILL.md          # Requirements gathering (pipeline step 1)
        ├── project-plan/SKILL.md          # Cross-session tracking (step 2)
        ├── architecture/SKILL.md          # Tech stack decisions (step 3)
        ├── full-stack-dev/SKILL.md        # Code implementation (step 4)
        ├── run-and-fix/SKILL.md           # Iterative debugging (step 5)
        ├── code-review/SKILL.md           # Quality review (step 6)
        ├── testing/SKILL.md               # Test coverage (step 7)
        ├── integration/SKILL.md           # Combine agent outputs (step 8)
        ├── deployment/SKILL.md            # Containerize and ship (step 9)
        ├── github-workflow/SKILL.md       # Git workflow + Actions + releases (step 10)
        ├── orchestrator/SKILL.md          # Agent hierarchy management
        ├── evaluation/SKILL.md            # Acceptance criteria + stop conditions
        ├── worktree-workflow/SKILL.md     # Git worktree parallel development
        ├── self-improve/SKILL.md          # CLAUDE.md self-improvement after corrections
        ├── prompt-playbook/SKILL.md       # Reusable prompt patterns
        ├── data-analytics/SKILL.md        # Database querying and data analysis
        ├── learning-mode/SKILL.md         # Explanations, diagrams, presentations
        ├── coding-agent/SKILL.md          # Parallel agent coordination
        ├── tmux-orchestration/SKILL.md    # Multi-pane terminal orchestration
        ├── problem-modeling/SKILL.md      # UPSA methodology
        ├── azure-ops/SKILL.md             # Azure patterns
        ├── powershell-automation/SKILL.md # Script templates
        ├── incident-response/SKILL.md     # Incident procedures
        ├── contrastive-scoring/SKILL.md   # Approach comparison
        ├── verify-response/SKILL.md       # Self-verification
        ├── summarize/SKILL.md             # Content summarization
        ├── session-logs/SKILL.md          # Query session logs
        └── healthcheck/SKILL.md           # System health audits

~/.claude/
└── CLAUDE.md → ~/Sortiarius/CLAUDE.md     # Symlink (created by setup.sh)

~/projects/                                # Your projects (created by sortiarius new)
└── my-app/
    ├── CLAUDE.md                          # Project-specific context
    ├── SPEC.md                            # Product spec (generated by product-spec skill)
    ├── PLAN.md                            # Progress tracker (generated by project-plan skill)
    └── CRITERIA.md                        # Evaluation criteria (generated by evaluation skill)
```
