---
name: github-workflow
description: Full GitHub workflow including branching, commits, PRs, issues, and CI/CD awareness
triggers:
  - github
  - pull request
  - PR
  - commit
  - branch
  - merge
  - issue
  - code review
  - CI
pipeline: []
---

# GitHub Workflow Skill

Standard GitHub workflow for all projects.

## Branch Naming

Format: `<type>/<short-description>`. Types: `feature/`, `fix/`, `refactor/`, `docs/`, `chore/`, `hotfix/`
Examples: `feature/rate-limiting`, `fix/auth-token-expiry`, `refactor/billing-service`

## Commit Message Format

```
type(scope): description
```
**Types:** feat, fix, refactor, docs, test, chore, perf, ci
**Rules:** Imperative mood, lowercase, no period, max 72 chars. Optional body explains WHY.

```bash
git commit -m "feat(auth): add JWT refresh token rotation"
git commit -m "fix(billing): prevent duplicate invoice on retry"
```

## PR Creation

Use `gh pr create` with structured body:
```bash
gh pr create --title "feat(auth): add JWT refresh token rotation" --body "$(cat <<'EOF'
## Summary
- Add automatic JWT refresh token rotation on each use
- Invalidate old refresh tokens after rotation

## Changes
- `src/services/auth.ts` — new `rotateRefreshToken()` method
- `src/middleware/auth.ts` — call rotation on token refresh endpoint

## Test plan
- [ ] Unit tests for token rotation logic
- [ ] Integration test: refresh flow rotates token
- [ ] Manual: verify login/refresh cycle in staging
EOF
)"
```

### PR Checklist (verify before creating)
- [ ] Branch up to date with base (`git pull --rebase origin main`)
- [ ] All tests pass locally, linter clean
- [ ] PR title follows commit format, description explains WHY
- [ ] Breaking changes called out explicitly

## Issue Management

```bash
gh issue create --title "Bug: token refresh fails" --label "bug,auth" --body "..."
gh pr create --title "fix(auth): handle expired token" --body "Closes #42"  # Auto-close
gh issue edit 42 --milestone "v2.1"
```

**Labels:** `bug`, `feature`, `enhancement`, `documentation`, `breaking`, `security`, `priority:high`, `priority:low`

## Code Review Process

```bash
gh pr diff <number>                    # View changes
gh pr checks <number>                  # Check CI status
gh pr review <number> --approve        # Approve
gh pr review <number> --request-changes --body "..."  # Request changes
```

## CI/CD Awareness

Before merging: `gh pr checks <number>` -- all must pass. Never merge with failing checks.
Re-run flaky checks: `gh run rerun <run-id> --failed`

## Changelog
- 2026-02-06: Initial creation
