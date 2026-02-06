# SteinBot - How to Use

## How It Works

SteinBot lives globally on your machine. After setup, **every** `claude` session is SteinBot - regardless of what directory you're in. No API keys, no external services.

```
~/.claude/CLAUDE.md  →  symlink  →  ~/SteinBot/CLAUDE.md
         ↓
Claude Code reads it automatically in every session
         ↓
SteinBot identity + skills + memory always available
```

When you're in a specific project, that project's local `CLAUDE.md` layers on top - adding project context without replacing SteinBot.

---

## Setup (One Time)

```bash
git clone https://github.com/SudoSteiN/SteinBot.git ~/SteinBot
cd ~/SteinBot
./setup.sh
source ~/.zshrc   # or ~/.bashrc / ~/.bash_profile
```

The setup script will:
1. Verify prerequisites (git, claude CLI)
2. Symlink `~/.claude/CLAUDE.md` to the repo (backs up existing if any)
3. Add the `stein` command to your PATH
4. Create `~/projects/` for new projects

---

## Daily Use

### Open SteinBot anywhere
```bash
stein              # Current directory
claude             # Same - SteinBot identity loads globally
```

### Create a new project
```bash
stein new my-terraform-module
```
Creates `~/projects/my-terraform-module/` with a project-local `CLAUDE.md` template and opens SteinBot in it.

### Manage SteinBot itself
```bash
stein home         # Open in ~/SteinBot to edit skills, memory, config
stein memory       # Quick-edit memory index in your $EDITOR
stein memory azure # Edit azure-specific memory
stein sync         # Preview and commit workspace changes to git
stein update       # Pull latest changes from remote
stein doctor       # Check framework health
```

---

## The Two-Layer System

```
Layer 1 (Global):  ~/.claude/CLAUDE.md → ~/SteinBot/CLAUDE.md
                   Always active. SteinBot identity, skills, domain rules.

Layer 2 (Local):   ~/projects/my-app/CLAUDE.md
                   Project-specific context layered on top.
```

**Example:** Working in a Terraform project. SteinBot's global rules (Azure safety, PowerShell standards) apply automatically. The project's local CLAUDE.md adds: "This project manages the East region infrastructure using Terraform 1.7."

---

## All Commands

| Command | What it does |
|---------|-------------|
| `stein` | Open SteinBot in current directory |
| `stein new <name>` | Create new project at ~/projects/\<name\> |
| `stein home` | Open SteinBot home (manage skills/memory) |
| `stein memory [file]` | Edit memory file in $EDITOR (default: index) |
| `stein sync [--all]` | Preview and commit workspace changes |
| `stein doctor` | Check framework health (symlink, skills, memory) |
| `stein skill new <name>` | Scaffold a new skill with template |
| `stein skill list` | List all skills with their triggers |
| `stein list` | List all projects in ~/projects/ |
| `stein update` | Pull latest SteinBot from git |
| `stein uninstall` | Remove global config (keeps repo) |
| `stein help` | Show all commands |
| `stein <path>` | Open SteinBot in a specific directory |

### Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `STEINBOT_HOME` | `~/SteinBot` | SteinBot repo location |
| `STEINBOT_PROJECTS` | `~/projects` | Where `stein new` creates projects |
| `EDITOR` | `vim` | Editor for `stein memory` |

---

## Skills

Skills live at `~/SteinBot/workspace/skills/` and are **autodiscovered**. Each skill has a `SKILL.md` with YAML frontmatter containing triggers. Claude matches requests to skills automatically — no routing table to maintain.

### List installed skills
```bash
stein skill list
```

### Add a new skill
```bash
stein skill new my-skill-name
```

This scaffolds `~/SteinBot/workspace/skills/my-skill-name/SKILL.md` with the standard template:
```markdown
---
name: my-skill-name
description: What this skill does
triggers:
  - keyword1
  - keyword2
pipeline: []
---

# Skill content here...

## Changelog
```

Edit the generated file, update triggers and content, then `stein sync`.

### Skill YAML fields

| Field | Required | Purpose |
|-------|----------|---------|
| `name` | Yes | Skill identifier |
| `description` | Yes | What the skill does |
| `triggers` | Yes | Keywords that activate the skill |
| `pipeline` | No | Other skills to run first (e.g., `[problem-modeling]`) |

### Check skill health
```bash
stein doctor
```
Validates all skills have proper frontmatter, required fields, and no broken references.

---

## Memory

Memory is split by domain under `~/SteinBot/workspace/memory/`:

| File | Contents |
|------|----------|
| `index.md` | Quick-reference, cross-domain notes, preferences |
| `azure.md` | Subscription, resource groups, Key Vaults, SQL servers |
| `powershell.md` | Script patterns, module notes, preferences |
| `incidents.md` | Past incidents, resolutions, post-mortems |
| `solutions.md` | Reusable solutions from past problems |

### Edit memory
```bash
stein memory          # Opens index.md
stein memory azure    # Opens azure.md
stein memory incidents # Opens incidents.md
```

### Timestamp convention
Use `<!-- learned: YYYY-MM-DD -->` on entries so stale knowledge can be identified and pruned.

**Important:** Memory does NOT auto-sync. Run `stein sync` at the end of productive sessions, or when SteinBot reminds you.

---

## Keeping It Updated

```bash
stein sync          # Preview changes, then commit workspace/
stein sync --all    # Preview and commit everything (not just workspace/)
stein update        # Pull latest from remote
```

`stein sync` now shows a diff summary before committing, so you can review what's going out.

---

## Health Checks

```bash
stein doctor
```

Checks:
- Global symlink (`~/.claude/CLAUDE.md`) is correct
- All skills have valid YAML frontmatter with `name:` and `triggers:`
- Memory directory exists with expected files
- Git repo and remote are configured
- `stein` and `claude` commands are in PATH

---

## Uninstalling

```bash
stein uninstall
```

This removes the `~/.claude/CLAUDE.md` symlink and restores any backup. The repo stays intact. To fully remove, also delete `~/SteinBot` and remove the PATH line from your shell profile.

---

## File Structure

```
~/SteinBot/                                # Home base (source of truth)
├── CLAUDE.md                              # Brain (symlinked to ~/.claude/)
├── HOW-TO-USE.md                          # This file
├── setup.sh                               # One-time global setup
├── .gitignore                             # Ignores secrets, scratch, IDE files
├── bin/
│   └── stein                              # Launcher command
└── workspace/
    ├── memory/                            # Persistent knowledge (split by domain)
    │   ├── index.md                       # Quick-reference and preferences
    │   ├── azure.md                       # Azure environment details
    │   ├── powershell.md                  # Script patterns and preferences
    │   ├── incidents.md                   # Past incidents and lessons
    │   └── solutions.md                   # Reusable solutions
    ├── scratch/                           # Temp files (gitignored)
    └── skills/
        ├── problem-modeling/SKILL.md      # UPSA methodology
        ├── azure-ops/SKILL.md             # Azure patterns
        ├── powershell-automation/SKILL.md # Script templates
        ├── incident-response/SKILL.md     # Incident procedures
        ├── contrastive-scoring/SKILL.md   # Approach comparison
        └── verify-response/SKILL.md       # Self-verification

~/.claude/
└── CLAUDE.md → ~/SteinBot/CLAUDE.md       # Symlink (created by setup.sh)

~/projects/                                # Your projects (created by stein new)
└── my-app/
    └── CLAUDE.md                          # Project-specific context
```
