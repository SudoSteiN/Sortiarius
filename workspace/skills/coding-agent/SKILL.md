---
name: coding-agent
description: Spawn and coordinate parallel coding agents for independent tasks, code generation, and refactoring
triggers:
  - parallel coding
  - spawn agent
  - background task
  - concurrent work
  - delegate task
pipeline: []
---

# Coding Agent Skill

Coordinate parallel Claude instances for independent coding tasks.

## When to Spawn Sub-Agents

Spawn agents when tasks are **independent** and **non-conflicting**:
- Generating code in separate files that don't import each other
- Refactoring modules that don't share mutable state
- Writing tests for different components
- Researching multiple APIs or docs simultaneously
- Generating rollback scripts while building forward migrations

**Do NOT spawn agents when:**
- Tasks depend on each other's output
- Multiple agents would edit the same file
- The task requires interactive decision-making mid-execution
- Total work is < 5 minutes for a single agent

## How to Use `sortiarius agent parallel`

### Step 1: Create a Task File
Write one task per line. Each line is a complete, self-contained instruction.

```bash
# tasks.txt — each line spawns one agent
Generate unit tests for src/services/auth.ts covering login, logout, token refresh
Generate unit tests for src/services/billing.ts covering invoice creation, payment processing
Refactor src/utils/date.ts to replace moment.js with date-fns, update all imports
Create a new middleware at src/middleware/rate-limit.ts implementing sliding window rate limiting
```

### Step 2: Structure Tasks for Maximum Parallelism
- Each task must name **exact file paths** it will create or modify
- Each task must be **self-contained** — no "see above" or "like the previous one"
- Include acceptance criteria: "The function should handle X, Y, Z edge cases"
- Specify the output format: "Create the file at [path] with [exports]"

### Step 3: Run
```bash
sortiarius agent parallel tasks.txt
```

For single background tasks:
```bash
sortiarius agent bg "Audit all TypeScript files in src/ for any unhandled promise rejections. Output a report to workspace/reports/unhandled-promises.md"
```

## Review Patterns for Agent Output

After agents complete, review systematically:

1. **Conflict check:** Did any agent modify a file another agent also touched?
   ```bash
   git diff --name-only | sort | uniq -d
   ```
2. **Compile check:** Does the project still build?
   ```bash
   npm run build  # or equivalent
   ```
3. **Test check:** Do existing tests still pass?
   ```bash
   npm test
   ```
4. **Integration check:** Read each agent's output. Do the pieces fit together?
5. **Style check:** Run linter to catch inconsistencies between agents.

## Error Handling When Agents Fail

| Failure Type | Action |
|---|---|
| Agent produced wrong output | Re-run with more specific instructions. Add constraints. |
| Agent edited wrong file | `git checkout -- [file]`, re-run with explicit file paths |
| Agent conflicted with another | Resolve manually, then commit. Restructure tasks to avoid overlap. |
| Agent timed out | Break task into smaller pieces. Check if it was waiting on input. |
| Agent partially completed | Review what was done, create a follow-up task for the remainder. |

## Task File Best Practices

```yaml
# Good: Specific, self-contained, non-overlapping
"Create src/utils/validators.ts exporting email, phone, and zip validators with JSDoc"
"Create src/utils/formatters.ts exporting currency, date, and number formatters with JSDoc"

# Bad: Vague, overlapping, dependent
"Update the utils folder"
"Fix the validators and also the formatters"
```

## Changelog
- 2026-02-06: Initial creation
