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

## CI/CD — GitHub Actions

Before merging: `gh pr checks <number>` -- all must pass. Never merge with failing checks.
Re-run flaky checks: `gh run rerun <run-id> --failed`

### Creating CI Workflows
Sortiarius provides CI templates at `workspace/templates/github/workflows/`. Copy and customize during project init.

```bash
# View recent workflow runs
gh run list --limit 10

# View a specific run's logs
gh run view <run-id> --log

# View only failed job logs
gh run view <run-id> --log-failed

# Re-run failed jobs only
gh run rerun <run-id> --failed

# Trigger a workflow manually
gh workflow run ci.yml --ref main
```

### Debugging CI Failures
Pattern: "Go fix the failing CI tests."
1. `gh run list --limit 5` — find the failing run
2. `gh run view <id> --log-failed` — get failure details
3. Read the failing test or build step
4. Fix locally, push, monitor re-run

## Worktree-Based Parallel Development

Use git worktrees for parallel Claude sessions (Boris tip #1):

```bash
# Create worktrees for parallel work
sortiarius worktree add auth-system
sortiarius worktree add payment-flow

# List active worktrees
sortiarius worktree ls

# Generate shell aliases (za, zb, zc)
eval "$(sortiarius worktree aliases)"

# After merging, clean up
sortiarius worktree rm auth-system
sortiarius worktree prune
```

Each worktree gets its own Claude session, its own branch, and its own context window. Merge via PR when done.

See `worktree-workflow` skill for the full pattern.

## Release Management

```bash
# Create a release from a tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
gh release create v1.0.0 --title "v1.0.0" --generate-notes

# Create release with custom notes
gh release create v1.0.0 --title "v1.0.0" --notes "$(cat <<'EOF'
## What's New
- Feature A
- Feature B

## Bug Fixes
- Fix C
EOF
)"

# Upload release assets
gh release upload v1.0.0 dist/*.tar.gz
```

## Repository Setup via gh API

```bash
# Configure branch protection
gh api repos/{owner}/{repo}/branches/main/protection -X PUT -f "required_pull_request_reviews[required_approving_review_count]=1"

# Set repo description and topics
gh repo edit --description "Project description" --add-topic "rust,wasm,woodworking"

# Configure Dependabot (create file)
# .github/dependabot.yml — generated during project init
```

## GitHub Projects (Boards)

```bash
# Create a project board
gh project create --owner @me --title "My Project" --format board

# List projects
gh project list

# Add issues to project
gh project item-add <project-number> --owner @me --url <issue-url>
```

## Changelog
- 2026-02-06: Initial creation
- 2026-02-07: Add CI debugging, worktrees, releases, repo setup, projects
