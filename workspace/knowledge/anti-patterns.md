# Sortiarius Knowledge Library — Anti-Patterns

> Things we tried that didn't work, and why.
> Just as important as solutions — prevents repeating mistakes.

## Template

```markdown
## [Anti-Pattern Name]
<!-- learned: YYYY-MM-DD -->
<!-- project: project-name -->
<!-- tags: relevant-tags -->

**What we tried:** The approach that failed.
**Why it failed:** Root cause analysis.
**What we did instead:** The approach that worked.
**How to recognize:** Signs that you're about to make this mistake again.
```

---

## Hook Bootstrapping Deadlock
<!-- learned: 2026-02-06 -->
<!-- project: sortiarius -->
<!-- tags: hooks, enforcement, architecture -->

**What we tried:** workflow-guard.sh hard-blocked ALL code writes when no SPEC.md existed, including writes to the Sortiarius framework itself and the hook's own source code.

**Why it failed:** The hook used `SORTIARIUS_HOME` (defaulting to `$HOME/Sortiarius`) to exclude framework files, but the actual repo path was `/home/user/SteinBot`. The hook couldn't be edited because the hook was blocking edits.

**What we did instead:**
1. Created SPEC.md first (hook allows .md files)
2. Added `CLAUDE_PROJECT_DIR` exclusion so the framework repo is never gated by its own enforcement hooks

**How to recognize:** Any enforcement hook that can block modifications to itself is a potential deadlock. Always ensure hooks have a bypass for their own repo.
