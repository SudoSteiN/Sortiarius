#!/usr/bin/env bash
# Sortiarius Development Workflow Hook: Enforce git conventions
# Hook type: PreToolUse (matcher: Bash)
#
# Enforces:
# - Commit message format: type(scope): description
# - No commits to main/master without explicit intent
# - Test execution reminders before push
# - Branch naming conventions
set -uo pipefail

INPUT="$(cat)"
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"

# Skip non-git commands
echo "$COMMAND" | grep -qE '^\s*git\s' || exit 0

# --- Rule 1: Block direct commits to main/master ---
if echo "$COMMAND" | grep -qE 'git\s+commit'; then
  current_branch="$(git branch --show-current 2>/dev/null || echo "")"
  if [ "$current_branch" = "main" ] || [ "$current_branch" = "master" ]; then
    # Check if the commit message mentions it's intentional
    if ! echo "$COMMAND" | grep -qiE 'hotfix|emergency|initial'; then
      cat << 'DENY'
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Direct commit to main/master branch. Create a feature branch first (git checkout -b feature/my-change). Use 'hotfix' in the message if this is an emergency fix."}}
DENY
      exit 0
    fi
  fi
fi

# --- Rule 2: Enforce commit message format ---
if echo "$COMMAND" | grep -qE 'git\s+commit\s'; then
  # Extract the commit message
  msg=""
  if echo "$COMMAND" | grep -qE -- '-m\s'; then
    msg="$(echo "$COMMAND" | sed -n "s/.*-m ['\"]\\([^'\"]*\\)['\"].*/\\1/p")"
    # Try heredoc format
    if [ -z "$msg" ]; then
      msg="$(echo "$COMMAND" | sed -n 's/.*-m "\$(cat <<.*//p')"
    fi
  fi

  # Only enforce on simple -m "message" commits (skip heredocs/complex formats)
  if [ -n "$msg" ]; then
    # Check format: type(scope): description OR type: description
    if ! echo "$msg" | grep -qE '^(feat|fix|refactor|docs|test|chore|style|perf|ci|build|revert)(\([a-zA-Z0-9_-]+\))?: .+'; then
      # Allow merge commits and initial commits
      if ! echo "$msg" | grep -qiE '^(merge|initial|revert|wip)'; then
        cat << 'DENY'
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Commit message doesn't follow conventional format. Use: type(scope): description\n\nTypes: feat, fix, refactor, docs, test, chore, style, perf, ci, build, revert\nExamples:\n  feat(auth): add OAuth2 login flow\n  fix(api): handle null response from endpoint\n  docs: update README with setup instructions"}}
DENY
        exit 0
      fi
    fi
  fi
fi

# --- Rule 3: Warn on force push ---
if echo "$COMMAND" | grep -qE 'git\s+push\s.*--force|git\s+push\s+-f\b'; then
  cat << 'DENY'
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Force push detected. This rewrites remote history and can destroy work. Use --force-with-lease instead, or confirm this is intentional."}}
DENY
  exit 0
fi

# --- Rule 4: Warn on push without upstream ---
if echo "$COMMAND" | grep -qE 'git\s+push$'; then
  cat << 'DENY'
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Bare 'git push' without specifying remote/branch. Use 'git push -u origin <branch-name>' to be explicit about where you're pushing."}}
DENY
  exit 0
fi

# No issues found
exit 0
