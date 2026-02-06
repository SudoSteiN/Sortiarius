# Agent Instructions

## Core Workflow

### For Simple Requests (< 3 steps)
Execute directly. No preamble needed.

### For Complex Requests (3+ steps, ambiguous, or risky)
1. **Model the problem first:**
   - What entities are involved?
   - What are the constraints?
   - What does success look like?
   - What assumptions am I making?

2. **Consider approaches:**
   - Generate at least 2 approaches
   - Compare: Which is simpler? More robust? Better aligned with the actual goal?
   - Pick one and state why

3. **Execute with checkpoints:**
   - For destructive operations: Confirm before proceeding
   - After each major step: Verify it worked
   - If something fails: Classify as "wrong model" vs "wrong execution"

4. **Learn from outcomes:**
   - What worked? What didn't?
   - Should this become a reusable skill?

## Domain Knowledge

### Azure Infrastructure
- Always use Az CLI or PowerShell, not portal clicks
- Check resource group context before operations
- For Key Vault: Verify access policies before secret operations
- For SQL: Always check if AG (Always On) is involved

### PowerShell Standards
- Use approved verbs (Get-, Set-, New-, Remove-)
- Include -ErrorAction and -WhatIf where appropriate
- Prefer splatting for readability on complex commands

### Database Operations
- Never run destructive queries without WHERE clause confirmation
- For production: Always generate rollback scripts
- Check AG replica status before maintenance

### Microsoft Graph API
- Use application permissions for automation, delegated for interactive
- Handle pagination for list operations
- Respect throttling (429 responses)

## Skill Usage
When a task matches a skill in the skills folder, use that skill's methodology.
Key skills:
- `problem-modeling`: For complex, ambiguous problems
- `azure-ops`: For Azure infrastructure tasks
- `powershell-automation`: For script generation
- `incident-response`: For production issues

## Memory
- Log significant learnings to MEMORY.md
- Reference past solutions when relevant
- Track what works and what doesn't for Justin's specific environment
