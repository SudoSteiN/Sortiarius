---
name: self-improve
description: Systematic CLAUDE.md self-improvement — Claude writes rules for itself after corrections
triggers:
  - improve yourself
  - update your rules
  - you keep doing
  - add rule
  - don't do that again
  - remember this
  - learn from this
pipeline: []
---

# Self-Improve Skill

## When to Use
- After Justin corrects a mistake or preference
- When a pattern of errors emerges
- When Justin says "remember this" or "don't do that again"
- Boris tip #3: "After every correction, tell Claude: Update your CLAUDE.md so you don't make that mistake again."

## Process

### Step 1: Identify the Correction
When corrected, classify what happened:
- **Style/preference:** "Use X instead of Y" → goes to project CLAUDE.md or global
- **Technical mistake:** "That approach is wrong because..." → goes to memory/solutions.md
- **Process error:** "Always do X before Y" → goes to CLAUDE.md workflow section
- **Domain rule:** "In Azure, never do X" → goes to CLAUDE.md domain rules

### Step 2: Formulate the Rule
Write the rule in imperative form. Be specific, not vague.

Bad: "Be careful with database operations"
Good: "Always generate a rollback script before running ALTER TABLE in production"

Bad: "Use better error handling"
Good: "Use Write-Verbose instead of Write-Host in PowerShell scripts"

### Step 3: Check for Conflicts
Before adding a rule:
1. Read the target CLAUDE.md
2. Search for existing rules about the same topic
3. If a conflicting rule exists, update it (don't duplicate)
4. If a related rule exists, extend it

### Step 4: Write the Rule
Determine the correct location:

| Type | Location |
|------|----------|
| Global preference (all projects) | `~/Sortiarius/CLAUDE.md` → Domain Rules section |
| Project-specific rule | `./CLAUDE.md` → Project Rules section |
| Technical solution | `~/Sortiarius/workspace/memory/solutions.md` |
| Domain knowledge | `~/Sortiarius/workspace/memory/<domain>.md` |

Add with a timestamp:
```markdown
<!-- learned: 2026-02-07 -->
- Always use `wasm-pack build --target web` not `--target bundler` for Vite projects
```

### Step 5: Confirm
Tell Justin what rule was added and where. Example:
> Added rule to project CLAUDE.md: "Always use `wasm-pack build --target web` for Vite projects"

## Auto-Detection Patterns

Watch for these correction signals in conversation:
- "No, do X instead" → Preference rule
- "That's wrong because..." → Technical correction
- "I already told you..." → Repetition — escalate to CLAUDE.md rule
- "Always/Never do X" → Explicit rule
- "Why did you do X?" → Possible misunderstanding, ask before adding rule
- "Stop doing X" → Clear rule, add immediately

## Conflict Resolution
If a new rule contradicts an existing one:
1. Ask Justin which should take precedence
2. Remove or update the old rule
3. Add a note explaining why it changed

## Changelog
- 2026-02-07: Initial creation — CLAUDE.md self-improvement system
