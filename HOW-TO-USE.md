# How to Use These Files

## Step 1: Create Project Directory
```bash
mkdir ~/personal-assistant
cd ~/personal-assistant
```

## Step 2: Put CLAUDE.md in the Directory
Copy the `CLAUDE.md` file from this output into `~/personal-assistant/CLAUDE.md`

## Step 3: Open Claude Code in That Directory
```bash
cd ~/personal-assistant
claude
```

## Step 4: Paste This Kickoff Prompt

```
Read CLAUDE.md and execute all phases in order. Create all files as specified. Start with Phase 1 now. Only ask me questions for: API keys, channel choice, or environment-specific values like subscription IDs. Don't ask if I'm ready, just start.
```

That's it. Claude Code will:
1. Read your CLAUDE.md
2. Install OpenClaw
3. Create all the prompt files (SOUL.md, AGENTS.md, etc.)
4. Create all 6 skills
5. Configure the system
6. Guide you through channel setup
7. Test everything

## What You'll Need to Provide

When Claude Code asks:
- **Anthropic API key** - Get from console.anthropic.com (you already have Claude Pro, just need API access)
- **Slack or Discord** - Which channel you want first
- **Bot tokens** - Claude Code will tell you exactly where to get them
- **Azure details** (optional) - Subscription ID, common resource group names

## Expected Time
- Phase 1-2: 10 minutes
- Phase 3-4: 15 minutes
- Phase 5: 20-30 minutes (channel auth is the slowest part)
- Phase 6: 10 minutes

**Total: About 1 hour for a working system**

## If Something Breaks

Tell Claude Code:
```
That failed. Run openclaw doctor and diagnose the issue.
```

Or:
```
Check if the file was created correctly and fix any issues.
```

## Claude Code Pro Tips

**Keep it in context:**
- Claude Code reads CLAUDE.md automatically when it's in the root
- If it forgets, say: "Re-read CLAUDE.md"

**Be directive:**
- "Create the file now" > "Can you create the file?"
- "Fix it" > "What do you think went wrong?"

**Batch operations:**
- "Create all 6 skills now" works better than one at a time

**Verification:**
- After file creation, say: "Verify all files exist with ls"
- Say: "Cat the file to make sure it's correct"

**If it gets stuck:**
- "Stop. What phase are we on? What's the next concrete step?"
- "Skip this for now, move to the next phase"

**Resuming later:**
- "We're on Phase 3. Continue from where we left off."

## Files You'll End Up With

```
workspace/
├── SOUL.md                # Personality
├── AGENTS.md              # Instructions
├── MEMORY.md              # Learned knowledge
├── TOOLS.md               # Tool guidance
└── skills/
    ├── problem-modeling/
    │   └── SKILL.md
    ├── azure-ops/
    │   └── SKILL.md
    ├── powershell-automation/
    │   └── SKILL.md
    ├── incident-response/
    │   └── SKILL.md
    ├── contrastive-scoring/
    │   └── SKILL.md
    └── verify-response/
        └── SKILL.md
```

## After It's Running

Talk to your assistant via Slack/Discord. Test with:
1. Simple: "List all Azure resource groups"
2. Complex: "Help me plan a Key Vault migration - think through this carefully"
3. Incident: "Alert: Database CPU spiking"

The system will route simple requests directly, and use problem-modeling for complex ones.
