---
name: code-review
description: Automated code review covering security, performance, quality, and architecture
triggers:
  - review code
  - code review
  - review PR
  - audit code
  - security review
  - review changes
pipeline: []
---

# Code Review Skill

Systematic code review process. Apply all relevant sections based on the scope of changes.

## How to Run a Review

1. Get the diff: `git diff main...HEAD` or `gh pr diff <number>`
2. Identify changed files and their domains (frontend, backend, infra, tests)
3. Run each applicable checklist below against the changes
4. Output findings in the standard format (see bottom)

## Security Review (OWASP-Aware)

Check every changed file for:

- [ ] **Injection:** Are user inputs parameterized? No string concatenation in SQL/shell/LDAP queries.
- [ ] **Auth/AuthZ:** Are endpoints protected? Do permission checks happen server-side, not just client?
- [ ] **XSS:** Is output encoded/escaped? React `dangerouslySetInnerHTML` justified and sanitized?
- [ ] **Sensitive data:** No secrets, tokens, passwords, or PII in code or logs. Check `console.log`, `Write-Verbose`.
- [ ] **CSRF:** State-changing endpoints require CSRF tokens or SameSite cookies.
- [ ] **Deserialization:** No `eval()`, `pickle.loads()`, `JSON.parse()` on untrusted input without validation.
- [ ] **Dependencies:** New deps added? Check for known vulnerabilities (`npm audit`, `pip audit`).
- [ ] **Error messages:** Errors returned to users must not leak stack traces, file paths, or DB schema.

## Performance Review

- [ ] **N+1 queries:** Database calls inside loops. Look for ORM `.find()` or `.get()` in iterations.
- [ ] **Missing indexes:** New queries on columns without indexes. Check migration files.
- [ ] **Unbounded queries:** `SELECT *` without `LIMIT`, or list endpoints without pagination.
- [ ] **Memory leaks:** Event listeners without cleanup, subscriptions without unsubscribe, growing caches.
- [ ] **Unnecessary re-renders (React):** Missing `useMemo`/`useCallback` on expensive computations or callbacks passed as props.
- [ ] **Large payloads:** API responses returning more data than the consumer needs. Over-fetching.
- [ ] **Blocking operations:** Synchronous I/O on hot paths. Long-running operations without async/await.

## Code Quality Review

- [ ] **DRY:** Is logic duplicated? Should it be extracted into a shared function/module?
- [ ] **Single Responsibility:** Does each function/class do one thing? Functions over 40 lines are suspect.
- [ ] **Naming:** Variables and functions describe what they hold/do. No `data`, `temp`, `result`, `handle`.
- [ ] **Error handling:** Are errors caught and handled meaningfully? No empty `catch {}` blocks. No swallowed errors.
- [ ] **Type safety:** TypeScript `any` is justified? Proper null checks? No `!` assertions without comment.
- [ ] **Magic values:** No hardcoded numbers or strings. Use constants or config.
- [ ] **Dead code:** Commented-out code, unused imports, unreachable branches.
- [ ] **Test coverage:** Are new code paths covered by tests? Are edge cases tested?

## Architecture Review

- [ ] **Separation of concerns:** Business logic is not in controllers/routes. UI logic is not in API calls.
- [ ] **Dependency direction:** Dependencies flow inward. Core logic does not import from infrastructure.
- [ ] **Interface boundaries:** New modules expose clean interfaces, not internal implementation details.
- [ ] **Consistency:** New code follows existing patterns in the codebase. Deviations are justified.
- [ ] **Coupling:** Can this module be changed independently? Would changing it break unrelated modules?

## Output Format

Report findings in this structure:

```markdown
## Code Review: [PR title or description]

### Critical (must fix before merge)
- **[SECURITY]** `src/api/users.ts:42` — SQL query uses string interpolation. Use parameterized query.
  **Fix:** Replace template literal with `db.query('SELECT * FROM users WHERE id = $1', [userId])`

### Warning (should fix, non-blocking)
- **[PERF]** `src/services/orders.ts:78` — `findAll()` called inside loop. Batch into single query.
  **Fix:** Collect IDs, then `WHERE id IN (...)`

### Suggestion (nice to have)
- **[QUALITY]** `src/utils/format.ts:15` — Variable `d` should be `formattedDate` for clarity.

### Looks Good
- Auth middleware changes are solid. Token validation covers all edge cases.
- Test coverage for the new billing flow is thorough.
```

**Severity rules:**
- **Critical:** Security vulnerabilities, data loss risk, broken functionality
- **Warning:** Performance issues, missing error handling, poor patterns
- **Suggestion:** Naming, style, minor improvements

## Changelog
- 2026-02-06: Initial creation
