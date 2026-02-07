#!/usr/bin/env bash
# Sortiarius Session Discipline Hook: Self-audit + quality gate + workspace check
# Hook type: Stop
#
# Three checks at session end:
# 1. Self-audit: Did the pipeline get followed? Were tests/builds run?
# 2. Quality gate: Were code files written without tests being run?
# 3. Workspace: Are there uncommitted changes?
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
SESSION_LOG="$SCRATCH_DIR/session-log.jsonl"
MODIFIED_LOG="$SCRATCH_DIR/session-modified-files.log"
INPUT="$(cat)"

# Prevent infinite loop: if stop hook is already active, let it through
STOP_HOOK_ACTIVE="$(echo "$INPUT" | jq -r '.stop_hook_active // false')"
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

MESSAGES=""

# --- Self-Audit: Check pipeline adherence ---
if [ -f "$SESSION_LOG" ]; then
  SESSION_CMDS="$(wc -l < "$SESSION_LOG" 2>/dev/null | tr -d ' ')"

  # If significant session (>10 commands), check for workflow signals
  if [ "$SESSION_CMDS" -gt 10 ]; then
    # Check if code was written (via modified files log from regression-guard)
    CODE_WRITTEN=false
    if [ -f "$MODIFIED_LOG" ] && [ "$(wc -l < "$MODIFIED_LOG" | tr -d ' ')" -gt 0 ]; then
      CODE_WRITTEN=true
    fi

    if $CODE_WRITTEN; then
      # Were tests run?
      TESTS_RAN=false
      if grep -qEi '"command".*(npm test|npx jest|pytest|cargo test|go test|vitest|mocha|make test|yarn test)' "$SESSION_LOG" 2>/dev/null; then
        TESTS_RAN=true
      fi

      if ! $TESTS_RAN; then
        MESSAGES="${MESSAGES}QUALITY GATE: Code was modified this session but tests were never run. Consider running tests before ending.\n"
      fi

      # Was a build attempted?
      BUILD_RAN=false
      if grep -qEi '"command".*(npm run build|cargo build|go build|make build|tsc|webpack|vite build)' "$SESSION_LOG" 2>/dev/null; then
        BUILD_RAN=true
      fi

      if ! $BUILD_RAN; then
        MESSAGES="${MESSAGES}QUALITY GATE: Code was modified but no build was run. Consider verifying the project still compiles.\n"
      fi
    fi
  fi
fi

# --- Quality Gate: Report modified file count ---
if [ -f "$MODIFIED_LOG" ]; then
  MOD_UNIQUE="$(awk '{print $2}' "$MODIFIED_LOG" 2>/dev/null | sort -u | wc -l | tr -d ' ')"
  if [ "$MOD_UNIQUE" -gt 0 ]; then
    MESSAGES="${MESSAGES}Session modified $MOD_UNIQUE code files. Update PLAN.md if applicable.\n"
  fi
  # Clean up the modified log for next session
  rm -f "$MODIFIED_LOG"
fi

# --- Workspace: Check for uncommitted changes ---
if [ -d "$SORTIARIUS_HOME/.git" ]; then
  dirty_count="$(git -C "$SORTIARIUS_HOME" diff --name-only workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  untracked_count="$(git -C "$SORTIARIUS_HOME" ls-files --others --exclude-standard workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  total=$((dirty_count + untracked_count))
  if [ "$total" -gt 0 ]; then
    MESSAGES="${MESSAGES}Workspace has $total uncommitted changes. Remind Justin to run 'sortiarius sync' to save memory/skill updates.\n"
  fi
fi

# --- Memory: Check if memory was updated (manually OR via auto-memory) ---
MEMORY_DIR="$SORTIARIUS_HOME/workspace/memory"
SESSION_MARKER="$SCRATCH_DIR/.session-start"
PENDING_MEMORIES="$SCRATCH_DIR/pending-memories.jsonl"
MEMORY_UPDATED=false
# Manual memory update check
if [ -f "$SESSION_MARKER" ] && [ -d "$MEMORY_DIR" ]; then
  if find "$MEMORY_DIR" -type f -newer "$SESSION_MARKER" 2>/dev/null | grep -q .; then
    MEMORY_UPDATED=true
  fi
fi
# Auto-memory pending entries count as "will be updated" (memory-writer.sh runs after this)
if [ -f "$PENDING_MEMORIES" ] && [ -s "$PENDING_MEMORIES" ]; then
  MEMORY_UPDATED=true
fi
if ! $MEMORY_UPDATED; then
  MESSAGES="${MESSAGES}Memory hasn't been updated this session. Consider whether anything should be recorded.\n"
fi

# --- Knowledge Library: Check if updated this session, remind if not ---
KNOWLEDGE_SOLUTIONS="$SORTIARIUS_HOME/workspace/knowledge/solutions.md"
KNOWLEDGE_UPDATED=false
if [ -f "$SESSION_MARKER" ] && [ -f "$KNOWLEDGE_SOLUTIONS" ]; then
  if find "$KNOWLEDGE_SOLUTIONS" -newer "$SESSION_MARKER" 2>/dev/null | grep -q .; then
    KNOWLEDGE_UPDATED=true
  fi
fi
if ! $KNOWLEDGE_UPDATED && ! $MEMORY_UPDATED && [ -f "$SESSION_LOG" ]; then
  SESSION_CMDS="$(wc -l < "$SESSION_LOG" 2>/dev/null | tr -d ' ')"
  if [ "$SESSION_CMDS" -gt 20 ]; then
    MESSAGES="${MESSAGES}Significant session ($SESSION_CMDS operations). Consider updating the knowledge library with any reusable solutions.\n"
  fi
fi

# If we have messages, inject them as context for Claude's wrap-up
if [ -n "$MESSAGES" ]; then
  FULL_MSG="[Sortiarius Session End — Self-Audit & Quality Gate]\n${MESSAGES}Before ending, address these items or acknowledge them to Justin."
  jq -n --arg msg "$(echo -e "$FULL_MSG")" '{
    "decision": "block",
    "reason": $msg
  }'
  exit 0
fi

exit 0
