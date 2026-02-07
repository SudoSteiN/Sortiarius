---
name: evaluation
description: Evaluation criteria system that defines acceptance criteria, runs automated checks, and determines when development loops should stop
triggers:
  - evaluate
  - check criteria
  - are we done
  - acceptance criteria
  - quality gate
  - stop condition
  - done criteria
  - ready to ship
  - pass fail
pipeline: [run-and-fix, testing, code-review]
---

# Evaluation Skill — When to Stop

## Purpose
Development loops (run-and-fix, test-fix, refactor) need clear stop conditions.
Without evaluation criteria, you either stop too early (first thing that compiles) or
never stop (endless polish). This skill defines what "done" means and checks it.

## When to Use

- **Before starting a dev loop**: Generate CRITERIA.md from SPEC.md
- **During dev loops**: Check criteria after each iteration
- **Before declaring a phase complete**: Run full evaluation
- **Before pushing/shipping**: Final quality gate

## Process

### Step 1: Generate CRITERIA.md

Read the project's SPEC.md and generate evaluation criteria:

```markdown
# CRITERIA.md — Auto-generated from SPEC.md
# Last generated: 2026-02-06
# Re-generate if SPEC.md changes

## Build Criteria (Gate: Phase 4-5)
- [ ] Project compiles without errors
- [ ] No TypeScript/type errors (0 errors in `tsc --noEmit`)
- [ ] No lint errors (0 errors in `eslint .` or equivalent)
- [ ] All dependencies resolve (no missing modules)

## Functional Criteria (Gate: Phase 5-7)
<!-- Generated from SPEC.md User Stories -->
- [ ] P0: [user story from spec]
- [ ] P0: [user story from spec]
- [ ] P1: [user story from spec]
<!-- P2 stories are nice-to-have, not gating -->

## Test Criteria (Gate: Phase 7)
- [ ] Unit test coverage >= 70% for business logic
- [ ] All P0 user stories have at least one integration test
- [ ] No test is skipped or pending without a linked issue
- [ ] Tests run in < 60 seconds

## API Criteria (Gate: Phase 5-6)
<!-- Generated from SPEC.md API Contract -->
- [ ] All endpoints from spec are implemented
- [ ] All endpoints return correct status codes
- [ ] Error responses follow consistent format
- [ ] Authentication/authorization enforced where specified

## UI Criteria (Gate: Phase 5-6)
<!-- Generated from SPEC.md UI Screens -->
- [ ] All screens from spec are implemented
- [ ] No console errors in browser
- [ ] Responsive on mobile viewport (375px)
- [ ] Loading states for async operations

## Security Criteria (Gate: Phase 6)
- [ ] No secrets in code (secret-scan hook covers runtime)
- [ ] Input validation on all user inputs
- [ ] SQL injection protection (parameterized queries)
- [ ] XSS protection (output encoding)
- [ ] CSRF protection if using cookies

## Deployment Criteria (Gate: Phase 9)
- [ ] Dockerfile builds successfully
- [ ] Container starts and responds to health check
- [ ] Environment variables documented in .env.example
- [ ] README has setup instructions

## Performance Criteria (Gate: Phase 6)
- [ ] API response time < 200ms for simple queries
- [ ] No N+1 query patterns
- [ ] Bundle size < 500KB (frontend, if applicable)
```

### Step 2: Run Automated Checks

The evaluation runner checks what it can automatically:

```bash
# Build check
npm run build 2>&1; echo "BUILD_EXIT=$?"

# Type check
npx tsc --noEmit 2>&1; echo "TYPE_EXIT=$?"

# Lint check
npm run lint 2>&1; echo "LINT_EXIT=$?"

# Test check
npm test -- --coverage 2>&1; echo "TEST_EXIT=$?"

# Security check (basic)
npm audit --production 2>&1; echo "AUDIT_EXIT=$?"
```

### Step 3: Score the Result

After running checks, produce an evaluation report:

```markdown
# Evaluation Report
Generated: 2026-02-06T14:30:00Z
Project: my-app
Phase: 5 (Fix)

## Results
| Category | Status | Score | Details |
|----------|--------|-------|---------|
| Build | PASS | 10/10 | Clean build, 0 errors |
| Types | PASS | 10/10 | 0 type errors |
| Lint | WARN | 7/10 | 3 warnings (non-blocking) |
| Tests | FAIL | 5/10 | 12/20 tests passing, 60% coverage |
| Security | PASS | 9/10 | 0 critical, 1 moderate |
| Functional | PARTIAL | 6/10 | 3/5 P0 stories complete |

## Overall: NOT READY (47/60 = 78%)
Threshold for Phase 5: 80%

## Blocking Issues
1. Tests: 8 failing tests in src/routes/auth.test.ts
2. Functional: P0 story "User can reset password" not implemented
3. Functional: P0 story "User can view dashboard" has rendering bug

## Recommended Actions
1. Fix auth test failures (likely related to JWT middleware change)
2. Implement password reset flow
3. Debug dashboard rendering — check data fetching hook
```

### Step 4: Gate Decision

Based on the evaluation report, decide:

| Score | Decision | Action |
|-------|----------|--------|
| >= 90% | **PASS** | Proceed to next phase |
| 80-89% | **CONDITIONAL PASS** | Proceed but log the gaps for follow-up |
| 60-79% | **ITERATE** | Continue dev loop, focus on blocking issues |
| < 60% | **ESCALATE** | Stop and discuss with Justin — may need rethink |

### Stop Conditions for Dev Loops

The run-and-fix loop should stop when ANY of these are true:
1. **All criteria pass** (>= 90% score)
2. **Max attempts reached** (per run-and-fix skill limits)
3. **Circular failure** — same error after 3 consecutive attempts
4. **Architecture concern** — fix would require design change
5. **Diminishing returns** — score improved < 5% over last 3 iterations

### Never Stop When:
- Build doesn't compile (always fix this)
- P0 stories are incomplete (these are non-negotiable)
- Tests are actively failing (not skipped — failing)
- Security criteria have critical findings

## Integration with Pipeline

Each pipeline phase has an evaluation gate:

| Phase | Minimum Score | Key Criteria |
|-------|--------------|-------------|
| 4. Build | Build compiles | Build criteria only |
| 5. Fix | 80% overall | Build + Functional |
| 6. Review | 85% overall | + Security + Performance |
| 7. Test | 90% overall | + Test criteria |
| 8. Integrate | 85% overall | Integration tests pass |
| 9. Ship | 90% overall | + Deployment criteria |

## Self-Evaluation for Sub-Agents

When a sub-agent (Level 2) completes its task, it MUST self-evaluate:

```markdown
## Sub-Agent Self-Evaluation
Task: [what was assigned]
Status: COMPLETE / PARTIAL / FAILED

Checklist:
- [ ] Code compiles in isolation
- [ ] Matches the interface/contract specified in the task
- [ ] Edge cases handled (null, empty, error states)
- [ ] No hardcoded values that should be configurable
- [ ] File paths match what was specified in the task

Confidence: HIGH / MEDIUM / LOW
If LOW: [explain what's uncertain]
```

This self-evaluation is included in the agent's output file for the project agent to review.

## Generating CRITERIA.md from SPEC.md

To auto-generate, read SPEC.md and map sections:

| SPEC.md Section | CRITERIA.md Category |
|----------------|---------------------|
| User Stories (P0) | Functional Criteria (blocking) |
| User Stories (P1) | Functional Criteria (non-blocking) |
| Data Model | Build Criteria (schema validates) |
| API Contract | API Criteria |
| UI Screens | UI Criteria |
| Tech Stack | Build Criteria (dependencies) |
| Non-Goals | Exclusion list (don't evaluate these) |

## Changelog
- 2026-02-06: Initial creation — evaluation criteria system
