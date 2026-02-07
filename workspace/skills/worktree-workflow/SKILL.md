---
name: worktree-workflow
description: Git worktree-based parallel development — run multiple Claude sessions on different features simultaneously
triggers:
  - worktree
  - parallel development
  - multiple branches
  - parallel sessions
  - spin up worktrees
pipeline: [github-workflow]
---

# Worktree Workflow Skill

## When to Use
- Working on multiple independent features simultaneously
- Need to context-switch between features without stashing
- Want to run parallel Claude sessions for maximum throughput
- Boris tip #1: "Spin up 3-5 git worktrees, each running its own Claude session"

## What Are Git Worktrees?
Git worktrees let you check out multiple branches of the same repo simultaneously, each in its own directory. Unlike separate clones, they share the same `.git` storage. Each worktree gets its own Claude Code session with isolated context.

## Process

### Creating Worktrees for Parallel Work

```bash
# From the project root
sortiarius worktree add auth-system      # Creates worktree + branch feature/auth-system
sortiarius worktree add payment-flow     # Creates worktree + branch feature/payment-flow
sortiarius worktree add admin-dashboard  # Creates worktree + branch feature/admin-dashboard
```

This creates:
```
~/projects/myapp/                    # Main worktree (main branch)
~/projects/myapp-wt-auth-system/     # Worktree (feature/auth-system)
~/projects/myapp-wt-payment-flow/    # Worktree (feature/payment-flow)
~/projects/myapp-wt-admin-dashboard/ # Worktree (feature/admin-dashboard)
```

### Shell Aliases for Fast Switching

```bash
sortiarius worktree aliases    # Prints aliases you can source
# Output:
#   alias za='cd ~/projects/myapp-wt-auth-system && claude'
#   alias zb='cd ~/projects/myapp-wt-payment-flow && claude'
#   alias zc='cd ~/projects/myapp-wt-admin-dashboard && claude'
```

Add to your shell profile: `eval "$(sortiarius worktree aliases)"`

### Managing Worktrees

```bash
sortiarius worktree ls         # List all active worktrees
sortiarius worktree rm auth    # Remove worktree after merging
sortiarius worktree prune      # Clean up stale/merged worktrees
```

### Integration with Claude Sessions

Each worktree session is fully independent:
- Has its own CLAUDE.md (copied from main, can diverge)
- Runs in its own terminal tab/pane
- Doesn't share context window with other sessions
- Can run tests, builds independently

### Worktree + Orchestrator Pattern

For large features that break into independent parts:

1. **Plan in main worktree:** Use plan mode to decompose the feature
2. **Create worktrees per sub-feature:** `sortiarius worktree add <sub-feature>`
3. **Work in parallel:** Each worktree gets its own Claude session
4. **Merge back:** PR each worktree branch into the feature branch
5. **Clean up:** `sortiarius worktree prune`

### When NOT to Use Worktrees
- Features that heavily overlap in the same files (merge conflicts)
- Quick fixes that take < 30 minutes (overhead not worth it)
- When you need shared state between sessions (use sub-agents instead)

## Rules
- Always branch from the latest main/develop before creating a worktree
- Use `feature/` prefix for worktree branches by default
- Clean up worktrees after merging — they clutter the filesystem
- The session-start hook auto-detects worktrees and injects context
- Don't modify the same files in multiple worktrees simultaneously

## Changelog
- 2026-02-07: Initial creation — git worktree parallel development workflow
