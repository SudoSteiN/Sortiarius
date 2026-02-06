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
git clone https://github.com/SudoSteiN/SteinBot.git ~/Sortiarius
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

### Active hooks

| Hook | Event | What it enforces |
|------|-------|-----------------|
| `session-start.sh` | SessionStart | Injects skill manifest + memory into every session |
| `safety-bash.sh` | PreToolUse:Bash | Blocks: `rm -rf`, `DROP TABLE`, `DELETE` without `WHERE`, `Remove-Az*` without `-WhatIf`, Azure resource deletion, production config writes |
| `safety-files.sh` | PreToolUse:Edit/Write | Blocks: edits to `.env`, credentials, `.git/`, SSH keys |
| `learning-tracker.sh` | PostToolUse:Bash | Logs commands to session log (async, non-blocking) |
| `session-stop.sh` | Stop | Checks for uncommitted workspace changes |
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
├── HOW-TO-USE.md                          # This file
├── setup.sh                               # One-time global setup
├── .gitignore                             # Ignores secrets, scratch, IDE files
├── .claude/
│   ├── settings.json                      # Hook configuration (committed)
│   └── hooks/                             # Hook scripts
│       ├── session-start.sh               # Context injection on startup
│       ├── safety-bash.sh                 # Block dangerous bash commands
│       ├── safety-files.sh                # Protect sensitive files
│       ├── learning-tracker.sh            # Log commands for pattern analysis
│       ├── session-stop.sh                # Workspace dirty check
│       └── session-learn.sh               # Session analysis + memory suggestions
├── bin/
│   ├── sortiarius                         # Main launcher command
│   └── sortiarius-agent                   # Autonomous agent launcher
└── workspace/
    ├── memory/                            # Persistent knowledge (split by domain)
    │   ├── index.md                       # Quick-reference and preferences
    │   ├── azure.md                       # Azure environment details
    │   ├── powershell.md                  # Script patterns and preferences
    │   ├── incidents.md                   # Past incidents and lessons
    │   └── solutions.md                   # Reusable solutions
    ├── scratch/                           # Temp files (gitignored)
    │   ├── session-log.jsonl              # Command log (from learning-tracker hook)
    │   └── agents/                        # Agent output directories
    └── skills/
        ├── problem-modeling/SKILL.md      # UPSA methodology
        ├── azure-ops/SKILL.md             # Azure patterns
        ├── powershell-automation/SKILL.md # Script templates
        ├── incident-response/SKILL.md     # Incident procedures
        ├── contrastive-scoring/SKILL.md   # Approach comparison
        └── verify-response/SKILL.md       # Self-verification

~/.claude/
└── CLAUDE.md → ~/Sortiarius/CLAUDE.md     # Symlink (created by setup.sh)

~/projects/                                # Your projects (created by sortiarius new)
└── my-app/
    └── CLAUDE.md                          # Project-specific context
```
