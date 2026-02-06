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
stein memory       # Quick-edit MEMORY.md in your $EDITOR
stein sync         # Commit and push workspace changes to git
stein update       # Pull latest changes from remote
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
| `stein memory` | Edit memory file in $EDITOR |
| `stein sync` | Commit and push workspace changes |
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

Skills live at `~/SteinBot/workspace/skills/` and are available from any directory. Claude reads them on-demand when CLAUDE.md's routing table matches the request.

| Skill | Triggers on |
|-------|------------|
| `problem-modeling` | Complex, ambiguous, multi-step problems |
| `azure-ops` | Azure infrastructure operations |
| `powershell-automation` | Script generation, automation tasks |
| `incident-response` | Alerts, outages, production issues |
| `contrastive-scoring` | "Which approach?", comparing options |
| `verify-response` | Self-check before high-stakes responses |

### Add a new skill

```bash
mkdir ~/SteinBot/workspace/skills/your-skill-name
```

Create `SKILL.md` with YAML frontmatter:
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

Add a routing entry in `~/SteinBot/CLAUDE.md` under "Domain-Specific Routing". Then `stein sync`.

---

## Memory

`~/SteinBot/workspace/MEMORY.md` is SteinBot's persistent knowledge:
- Your environment details (subscription IDs, resource groups, servers)
- Patterns that work in your setup
- Solutions from past problems
- Things that failed and why

**Important:** Memory does NOT auto-sync. Run `stein sync` at the end of productive sessions, or when SteinBot reminds you.

---

## Keeping It Updated

```bash
stein sync          # Push workspace changes (memory, skills)
stein update        # Pull latest from remote
```

This lets you version-control your assistant's knowledge and sync across machines.

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
├── bin/
│   └── stein                              # Launcher command
└── workspace/
    ├── MEMORY.md                          # Persistent knowledge (grows over time)
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
