# CLAUDE.md - Personal AI Assistant Build Guide

You are building a personal AI assistant for Justin, a Cloud Operations and Database manager at Onbe. This system combines OpenClaw (production runtime) with UPSA concepts (intelligent problem-solving).

## PROJECT OVERVIEW

**Goal:** A personal AI assistant that:
1. Runs on OpenClaw's infrastructure (channels, lane queues, transcripts)
2. Uses UPSA's problem-modeling for complex tasks
3. Learns from interactions and improves over time
4. Is personalized for Justin's Azure/Cloud Ops/DBA work

**User Context:**
- Manages Cloud Operations and Database teams (East, West, SS regions)
- Reports to Satya Gade, manages 4 direct reports (Jaya, Noah, Rawlin, Jad)
- Heavy Azure infrastructure work (PowerShell, SQL, Key Vault, Graph API)
- Uses Claude Pro/Max subscription
- Prefers systematic, methodical approaches

---

## PHASE 1: BOOTSTRAP OPENCLAW

### Step 1.1: Check Prerequisites
```bash
# Verify Node.js 22+
node --version

# If not installed or wrong version, stop and inform user
```

### Step 1.2: Install OpenClaw
```bash
npm install -g openclaw@latest
```

### Step 1.3: Create Project Directory
```bash
mkdir -p ~/.openclaw-justin
cd ~/.openclaw-justin
```

### Step 1.4: Run Onboarding (Interactive)
Tell the user to run this manually - it requires interactive input:
```bash
openclaw onboard --install-daemon
```

### Step 1.5: Verify Installation
```bash
openclaw doctor
```

**CHECKPOINT:** OpenClaw gateway should be running. Verify with `openclaw gateway --status` or check if port 18789 is listening.

---

## PHASE 2: CREATE CORE PROMPT FILES

### Step 2.1: Create SOUL.md
Create `workspace/SOUL.md` with the personality and work context configuration.

### Step 2.2: Create AGENTS.md
Create `workspace/AGENTS.md` with the agent instructions for simple vs complex requests.

### Step 2.3: Create MEMORY.md
Create `workspace/MEMORY.md` with the template for environment facts and learned preferences.

### Step 2.4: Create TOOLS.md
Create `workspace/TOOLS.md` with the tool guidance and dangerous operation rules.

**CHECKPOINT:** Verify all 4 files exist in `workspace/`

---

## PHASE 3: CREATE SKILLS

All skills live in `workspace/skills/`. Each has a `SKILL.md` with YAML frontmatter.

| Skill | Purpose |
|-------|---------|
| `problem-modeling` | Model complex problems before solving them (UPSA/MFR methodology) |
| `azure-ops` | Azure infrastructure operations using PowerShell and Az CLI |
| `powershell-automation` | Generate production-quality PowerShell scripts |
| `incident-response` | Handle production incidents systematically |
| `contrastive-scoring` | Compare multiple approaches before choosing one |
| `verify-response` | Self-check before delivering response |

**CHECKPOINT:** Verify all 6 skill folders have SKILL.md files

---

## PHASE 4: CONFIGURE OPENCLAW

### Step 4.1: Create Configuration
Create or update `openclaw.json` with model settings, workspace path, skill loading, and security allowlists.

### Step 4.2: Verify Configuration
```bash
openclaw doctor
```

---

## PHASE 5: CONNECT CHANNELS

### Step 5.1: Choose Primary Channel
Recommend Slack or Discord for initial setup.

**For Slack:**
1. Create Slack App at api.slack.com
2. Add Bot Token Scopes: `chat:write`, `channels:history`, `channels:read`, `im:history`, `im:read`, `im:write`
3. Install to workspace
4. Get Bot Token and App Token

```json
{
  "channels": {
    "slack": {
      "botToken": "xoxb-...",
      "appToken": "xapp-..."
    }
  }
}
```

**For Discord:**
1. Create Application at discord.com/developers
2. Create Bot, get token
3. Enable Message Content Intent

```json
{
  "channels": {
    "discord": {
      "token": "..."
    }
  }
}
```

### Step 5.2: Test Channel Connection
```bash
# Restart gateway
openclaw gateway --restart

# Send test message
openclaw message send --to [CHANNEL_ID] --message "Hello from OpenClaw"
```

---

## PHASE 6: TEST AND ITERATE

### Test 1: Simple Request
Send: "What's the PowerShell command to list all Azure resource groups?"
Expected: Direct answer with code

### Test 2: Complex Request
Send: "I need to set up a new Key Vault with access for our app service and my team. Think through this carefully."
Expected: Uses problem-modeling skill, shows entities/constraints

### Test 3: Incident Simulation
Send: "Alert: SQL database CPU at 95%. Help me investigate."
Expected: Uses incident-response skill, provides diagnostic commands

### Test 4: Learning Test
After a few interactions, check MEMORY.md for updates

---

## MAINTENANCE COMMANDS

```bash
# Check status
openclaw doctor

# View logs
openclaw logs --tail 100

# Restart gateway
openclaw gateway --restart

# Update OpenClaw
npm update -g openclaw@latest
```

---

## ITERATION CHECKLIST

**After first week of use:**
- [ ] SOUL.md accurately reflects desired personality?
- [ ] AGENTS.md covers common work scenarios?
- [ ] Skills are being triggered appropriately?
- [ ] MEMORY.md is capturing useful learnings?
- [ ] Any new skills needed?

**After first month:**
- [ ] Patterns emerging that should become skills?
- [ ] Any consistent failures to address?
- [ ] Channel configuration working well?
- [ ] Security settings appropriate?

---

## TROUBLESHOOTING

### Gateway Won't Start
```bash
# Check if port in use
lsof -i :18789

# Check logs
openclaw logs --level error
```

### Skills Not Loading
```bash
# Verify skill structure
ls -la workspace/skills/*/SKILL.md

# Check for YAML frontmatter errors
head -20 workspace/skills/problem-modeling/SKILL.md
```

### Channel Not Responding
```bash
# Check channel status
openclaw channels status

# Re-authenticate
openclaw channels login
```

---

## SUCCESS CRITERIA

System is working when:
1. Gateway starts without errors
2. Can send/receive messages via channel
3. Simple requests get direct answers
4. Complex requests trigger problem-modeling
5. Azure/PowerShell commands are correct
6. MEMORY.md updates after significant interactions
7. Justin says "this is actually useful"
