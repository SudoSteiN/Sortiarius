---
name: run-and-fix
description: Iterative build-run-fix loop that runs commands, parses errors, fixes issues, and retries automatically
triggers:
  - run and fix
  - fix errors
  - debug loop
  - make it work
  - keep trying
  - iterate until working
  - build and test
  - fix build
  - fix tests
pipeline: []
---

# Run-and-Fix Skill

## When to Use
When you need to get something working through iteration: build errors, test failures, runtime crashes, linting issues. Instead of fixing one error and stopping, run the full loop.

## Process

### The Loop
```
1. Run the command (build, test, lint, start)
2. Did it succeed? → Done. Report success.
3. Did it fail? → Parse the error output
4. Classify the error (see Error Taxonomy below)
5. Apply the fix
6. Increment attempt counter
7. If attempts < MAX_ATTEMPTS (default: 5) → Go to step 1
8. If attempts >= MAX_ATTEMPTS → Stop. Report what's still broken and why.
```

### Implementation Pattern
When executing:
```bash
# Run the command, capture output
output=$(command 2>&1) || true
exit_code=$?

# If success, done
# If failure, analyze output, fix, retry
```

### Error Taxonomy and Fix Strategy

| Error Type | Detection Pattern | Fix Approach |
|-----------|------------------|-------------|
| **Missing dependency** | `Module not found`, `Cannot find module`, `No module named`, `command not found` | Install the dependency, then retry |
| **Type error** | `TypeError`, `type mismatch`, `expected X got Y`, `Property does not exist` | Fix the type, add cast, update interface |
| **Syntax error** | `SyntaxError`, `Unexpected token`, `Parse error` | Fix the syntax at the reported line |
| **Import error** | `ImportError`, `Cannot resolve`, `Module has no exported member` | Fix the import path or add the export |
| **Build config** | `tsconfig`, `webpack`, `vite`, `babel`, config file errors | Fix the config file |
| **Test failure** | `FAIL`, `AssertionError`, `Expected X received Y` | Fix the code or the test (read both before deciding) |
| **Port in use** | `EADDRINUSE`, `address already in use` | Kill the process or use a different port |
| **Permission** | `EACCES`, `Permission denied` | Fix permissions or use correct user |
| **Runtime crash** | `FATAL`, `Segfault`, `out of memory` | Analyze stack trace, fix root cause |
| **Lint error** | `eslint`, `prettier`, `flake8`, `rubocop` | Apply the suggested fix or disable the rule with justification |

### Rules

1. **Read the FULL error output** before attempting a fix — don't just fix the first line
2. **Fix the root cause, not the symptom** — if 5 type errors come from one wrong interface, fix the interface
3. **Track what you've already tried** — if the same fix didn't work twice, try a different approach
4. **Batch related errors** — if there are 10 missing imports, fix all 10 before re-running
5. **Never suppress errors to make them pass** — no `// @ts-ignore`, `# type: ignore`, `eslint-disable` unless genuinely necessary
6. **Know when to stop** — if you're going in circles (same error after 3 attempts), step back and reconsider the approach. Report to Justin.

### Max Attempts by Context

| Context | Max Attempts | Rationale |
|---------|-------------|-----------|
| Build/compile | 5 | Should converge quickly |
| Test suite | 7 | Tests may have cascading failures |
| Lint/format | 3 | Usually one-shot fixes |
| Runtime debug | 5 | May need deeper investigation |
| Infrastructure | 3 | Higher risk, stop sooner |

### Output Format
After completing the loop, report:
```
Status: [FIXED / PARTIALLY FIXED / STUCK]
Attempts: X/Y
Errors fixed: [list]
Remaining issues: [list, if any]
Changes made: [files modified]
```

### When to Escalate
Stop the loop and ask Justin if:
- The fix would require changing the project's architecture
- The error suggests a fundamental design problem
- You've hit max attempts and aren't making progress
- The fix requires access/credentials you don't have

## Changelog
- 2026-02-06: Initial creation
