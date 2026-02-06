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
source ~/.zshrc   # or ~/.bashrc
```

That's it. The setup script:
1. Symlinks `~/.claude/CLAUDE.md` to the repo (global identity)
2. Adds `stein` command to your PATH
3. Creates `~/projects/` for new projects
4. Installs a memory sync hook

---

## Daily Use

### Open SteinBot anywhere
```bash
stein              # Current directory - SteinBot is already loaded
claude             # Same thing - SteinBot identity loads globally
```

### Create a new project
```bash
stein new my-terraform-module
```
This creates `~/projects/my-terraform-module/` with a project-local `CLAUDE.md` template and opens SteinBot in it.

### Manage SteinBot itself
```bash
stein home         # Open in ~/SteinBot to edit skills, memory, config
stein memory       # Quick-edit MEMORY.md in your editor
stein sync         # Commit and push workspace changes to git
```

---

## The Two-Layer System

```
Layer 1 (Global):  ~/.claude/CLAUDE.md → ~/SteinBot/CLAUDE.md
                   Always active. SteinBot identity, skills, domain rules.

Layer 2 (Local):   ~/projects/my-app/CLAUDE.md
                   Project-specific context layered on top.
```

**Example:** You're working in a Terraform project. SteinBot's global rules (Azure safety, PowerShell standards) still apply. The project's local CLAUDE.md adds: "This project manages the East region infrastructure using Terraform 1.7."

---

## Commands

| Command | What it does |
|---------|-------------|
| `stein` | Open SteinBot in current directory |
| `stein new <name>` | Create new project with template CLAUDE.md |
| `stein home` | Open SteinBot home (manage skills/memory) |
| `stein memory` | Edit memory file in your editor |
| `stein sync` | Git commit + push workspace changes |
| `stein <path>` | Open SteinBot in a specific directory |
| `claude` | Works too - SteinBot is global |

---

## Skills (always available)

Skills live at `~/SteinBot/workspace/skills/` and are available from any directory. Claude reads them on-demand based on the request.

| Skill | Triggers on |
|-------|------------|
| `problem-modeling` | Complex, ambiguous, multi-step problems |
| `azure-ops` | Azure infrastructure operations |
| `powershell-automation` | Script generation, automation tasks |
| `incident-response` | Alerts, outages, production issues |
| `contrastive-scoring` | "Which approach?", comparing options |
| `verify-response` | Self-check before high-stakes responses |

---

## Memory

`~/SteinBot/workspace/MEMORY.md` persists across all sessions:
- Environment details (subscription IDs, resource groups, servers)
- Patterns that work
- Past solutions
- Things that failed and why

Memory syncs to git automatically, or manually with `stein sync`.

---

## Adding a New Skill

```bash
mkdir ~/SteinBot/workspace/skills/your-skill-name
```

Create `SKILL.md`:
```markdown
---
name: your-skill-name
description: What this skill does
triggers:
  - keyword1
  - keyword2
---

# Skill content here
```

Add a routing entry in `~/SteinBot/CLAUDE.md` under "Domain-Specific Routing".

Then `stein sync` to push it.

---

## File Structure

```
~/SteinBot/                                # Home base (source of truth)
├── CLAUDE.md                              # Brain (symlinked to ~/.claude/)
├── HOW-TO-USE.md                          # This file
├── setup.sh                               # One-time global setup
├── bin/
│   └── stein                              # Launcher command
└── workspace/
    ├── SOUL.md                            # Personality reference
    ├── AGENTS.md                          # Behavior rules reference
    ├── MEMORY.md                          # Persistent knowledge
    ├── TOOLS.md                           # Tool safety rules
    └── skills/
        ├── problem-modeling/SKILL.md
        ├── azure-ops/SKILL.md
        ├── powershell-automation/SKILL.md
        ├── incident-response/SKILL.md
        ├── contrastive-scoring/SKILL.md
        └── verify-response/SKILL.md

~/.claude/
└── CLAUDE.md → ~/SteinBot/CLAUDE.md       # Symlink (created by setup.sh)

~/projects/                                # Your projects (created by stein new)
└── my-app/
    └── CLAUDE.md                          # Project-specific context
```
