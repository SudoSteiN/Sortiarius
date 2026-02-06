# Claude Code Kickoff Prompt

Copy everything below this line and paste it into Claude Code:

---

I want you to build a personal AI assistant using OpenClaw as the runtime. Read the CLAUDE.md file in this project for complete instructions.

**Project:** Personal AI Assistant for Cloud Operations/DBA work
**Runtime:** OpenClaw (already handles channels, queues, transcripts)
**Enhancement:** Custom skills for systematic problem-solving

## Your Task

Execute the phases in CLAUDE.md in order:

1. **Phase 1:** Bootstrap OpenClaw (install, verify)
2. **Phase 2:** Create the 4 core prompt files (SOUL.md, AGENTS.md, MEMORY.md, TOOLS.md)
3. **Phase 3:** Create the 6 skills (problem-modeling, azure-ops, powershell-automation, incident-response, contrastive-scoring, verify-response)
4. **Phase 4:** Configure openclaw.json
5. **Phase 5:** Help me connect Slack or Discord (guide me through it)
6. **Phase 6:** Run tests to verify everything works

## Rules for You

1. **Do each phase completely before moving to the next**
2. **Create actual files, don't just show me the content**
3. **After creating files, verify they exist**
4. **When you need my input (API keys, channel choice), ask clearly**
5. **If something fails, diagnose before retrying**

## My Context (use this in SOUL.md and AGENTS.md)

- I'm Justin, Cloud Operations and Database manager at Onbe
- I manage teams: Jaya (DBA), Noah (IT Ops), Rawlin (Cloud Ops), Jad (Senior Cloud Engineer)
- I report to Satya Gade
- Main tech: Azure, PowerShell, SQL Server, Microsoft Graph API
- I prefer systematic approaches, no fluff, direct communication
- I want the assistant to model complex problems before solving them

## Start Now

Begin with Phase 1. Check if OpenClaw is already installed, if not, install it. Then proceed through each phase, creating all necessary files.

Don't ask me if I'm ready. Just start.
