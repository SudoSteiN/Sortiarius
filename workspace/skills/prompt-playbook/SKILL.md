---
name: prompt-playbook
description: Reusable prompt patterns for getting the best output from Claude Code sessions
triggers:
  - challenge this
  - review approach
  - grill me
  - clean room
  - scrap and redo
  - elegant solution
  - explain why
  - be thorough
pipeline: []
---

# Prompt Playbook Skill

## When to Use
- When Justin wants to level up the quality of Claude's output
- When a first-pass solution is mediocre and needs rethinking
- When doing code review or architectural review
- Boris tip #6: "Challenge Claude. Write detailed specs and reduce ambiguity."

## Prompt Patterns

### 1. The Griller
**When:** You want rigorous review before merging.
**Pattern:** "Grill me on these changes and don't make a PR until I pass your test."

**How Sortiarius applies this:**
- Review every changed file
- List concerns as numbered questions
- Wait for answers before proceeding
- Don't proceed until all concerns are addressed

### 2. The Clean Room
**When:** A solution feels hacky or over-engineered.
**Pattern:** "Knowing everything you know now, scrap this and implement the elegant solution."

**How Sortiarius applies this:**
- Throw away current approach entirely (don't try to fix it)
- Restate what the actual goal is (from SPEC.md or context)
- Design from scratch using everything learned from the failed attempt
- The second attempt is almost always better because constraints are clearer

### 3. The Deep Spec
**When:** Starting complex work where ambiguity leads to rework.
**Pattern:** Write an extremely detailed spec before any code.

**How Sortiarius applies this:**
- Use the product-spec skill
- Don't start coding until SPEC.md is approved
- Treat ambiguity as a bug — resolve it before it becomes code

### 4. The Bug Squasher
**When:** A bug comes in (Slack thread, CI failure, user report).
**Pattern:** "Here's the bug report: [paste]. Fix it."

**How Sortiarius applies this:**
- Parse the bug report for symptoms, expected vs actual behavior
- Search codebase for relevant code
- Identify root cause before writing a fix
- Write the fix + test that would have caught it
- Don't ask permission — just fix it

### 5. The Docker Detective
**When:** Distributed system issues, container failures.
**Pattern:** "Here are the docker logs: [paste]. Troubleshoot this."

**How Sortiarius applies this:**
- Parse timestamps and error patterns from logs
- Identify which service failed first (root cause vs cascading failures)
- Check configuration, environment variables, network connectivity
- Propose fix with rollback plan

### 6. The Plan-Then-Execute
**When:** Complex multi-step implementation.
**Pattern:** Start in plan mode. Pour energy into the plan. One-shot the implementation.

**How Sortiarius applies this:**
- Use problem-modeling skill for the plan
- Use contrastive-scoring if multiple approaches exist
- Have the plan reviewed (by Justin or a second Claude session)
- Execute the plan in a single focused pass
- If something goes sideways, re-plan instead of patching

### 7. The CI Fixer
**When:** CI/CD pipeline failures.
**Pattern:** "Go fix the failing CI tests."

**How Sortiarius applies this:**
- Run `gh run list --limit 5` to find recent failures
- Run `gh run view <id> --log-failed` to get failure details
- Read the failing test or build step
- Fix and push, monitor the re-run

### 8. The Explainer
**When:** Learning or onboarding, understanding existing code.
**Pattern:** "Explain this codebase. Start with the architecture, then drill into [area]."

**How Sortiarius applies this:**
- Draw ASCII architecture diagram
- Explain data flow from entry point to output
- Identify key patterns and design decisions
- Call out areas of tech debt or complexity

## Anti-Patterns (Don't Do These)
- **Vague prompts:** "Make it better" → Better: "Reduce the response time of the /api/users endpoint from 800ms to under 200ms"
- **Over-specifying HOW:** "Use a HashMap with String keys" → Better: "I need O(1) lookup by user ID"
- **Micromanaging:** Telling Claude which files to edit → Better: Describe the outcome, let Claude find the right files
- **Ignoring plan mode:** Jumping straight to implementation on complex tasks → Always plan first

## Changelog
- 2026-02-07: Initial creation — reusable prompt patterns from Boris's tips
