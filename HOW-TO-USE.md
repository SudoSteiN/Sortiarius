# SteinBot - How to Use

## How It Works

This repo **is** your personal assistant. Claude Code reads `CLAUDE.md` automatically when you open it in this directory. That file contains your personality, work context, domain rules, and skill routing. No API keys, no external services, no middleware.

```
You (in terminal) → Claude Code → reads CLAUDE.md → becomes SteinBot
```

---

## Setup (One Time)

### 1. Clone the repo
```bash
git clone https://github.com/SudoSteiN/SteinBot.git ~/SteinBot
```

### 2. Add a shell alias
```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, or $PROFILE for PowerShell)

# Bash/Zsh:
echo 'alias stein="cd ~/SteinBot && claude"' >> ~/.zshrc
source ~/.zshrc

# PowerShell:
Add-Content $PROFILE 'function stein { Set-Location ~/SteinBot; claude }'
```

### 3. Fill in your environment details
Edit `workspace/MEMORY.md` with your Azure subscription ID, resource group names, SQL instances, etc. This gives the assistant context about your specific environment.

---

## Daily Use

```bash
stein
```

That's it. You're now talking to SteinBot. It will:
- Answer simple questions directly
- Model complex problems before solving them
- Use Azure/PowerShell best practices automatically
- Follow incident response procedures for alerts
- Compare approaches when multiple options exist
- Self-verify before delivering important responses

---

## Example Conversations

**Simple:**
```
> What's the PowerShell to list all resource groups with their tags?
```

**Complex:**
```
> I need to migrate our Key Vault secrets to a new vault in a different region.
> Think through this carefully.
```

**Incident:**
```
> Alert: SQL database CPU at 95% on the East production server. Help me investigate.
```

**Comparison:**
```
> Should I use a managed identity or service principal for our new app service
> to access Key Vault? Compare the approaches.
```

---

## How Skills Work

Skills are markdown files in `workspace/skills/`. Claude Code reads them on-demand when a request matches. You don't trigger them manually - the routing in `CLAUDE.md` handles it.

| Skill | Triggers on |
|-------|------------|
| `problem-modeling` | Complex, ambiguous, multi-step problems |
| `azure-ops` | Azure infrastructure operations |
| `powershell-automation` | Script generation, automation tasks |
| `incident-response` | Alerts, outages, production issues |
| `contrastive-scoring` | "Which approach?", comparing options |
| `verify-response` | Self-check before high-stakes responses |

---

## Memory & Learning

`workspace/MEMORY.md` stores persistent knowledge:
- Your environment details (subscription IDs, resource groups, servers)
- Patterns that work in your specific setup
- Solutions from past problems
- Things that failed and why

The assistant updates this file as it learns. You can also edit it directly.

---

## Customization

### Add a new skill
```bash
mkdir workspace/skills/your-skill-name
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

Then add a routing entry in `CLAUDE.md` under "Domain-Specific Routing".

### Adjust personality
Edit the "Identity & Personality" section in `CLAUDE.md`.

### Add domain rules
Add rules under "Domain Rules" in `CLAUDE.md`.

---

## Keeping It Updated

```bash
cd ~/SteinBot
git add -A && git commit -m "Update memory and skills"
git push
```

This lets you version-control your assistant's knowledge and sync it across machines.

---

## File Structure

```
SteinBot/
├── CLAUDE.md                              # Brain - loaded every session
├── HOW-TO-USE.md                          # This file
├── .gitignore
└── workspace/
    ├── SOUL.md                            # Personality reference
    ├── AGENTS.md                          # Behavior rules reference
    ├── MEMORY.md                          # Persistent knowledge (grows over time)
    ├── TOOLS.md                           # Tool safety rules
    └── skills/
        ├── problem-modeling/SKILL.md      # UPSA methodology
        ├── azure-ops/SKILL.md             # Azure patterns
        ├── powershell-automation/SKILL.md # Script templates
        ├── incident-response/SKILL.md     # Incident procedures
        ├── contrastive-scoring/SKILL.md   # Approach comparison
        └── verify-response/SKILL.md       # Self-verification
```
