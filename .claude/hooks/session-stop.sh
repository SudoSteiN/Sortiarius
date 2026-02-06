#!/usr/bin/env bash
# Sortiarius Session Discipline Hook: Check workspace state before session ends
# Hook type: Stop
#
# Checks if workspace has uncommitted changes and reminds about sortiarius sync.
# Also checks if this was a significant session that should update memory.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
INPUT="$(cat)"

# Prevent infinite loop: if stop hook is already active, let it through
STOP_HOOK_ACTIVE="$(echo "$INPUT" | jq -r '.stop_hook_active // false')"
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

MESSAGES=""

# Check for uncommitted workspace changes
if [ -d "$SORTIARIUS_HOME/.git" ]; then
  dirty_count="$(git -C "$SORTIARIUS_HOME" diff --name-only workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  untracked_count="$(git -C "$SORTIARIUS_HOME" ls-files --others --exclude-standard workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  total=$((dirty_count + untracked_count))
  if [ "$total" -gt 0 ]; then
    MESSAGES="${MESSAGES}Workspace has $total uncommitted changes. Remind Justin to run 'sortiarius sync' to save memory/skill updates.\n"
  fi
fi

# Check if memory index was updated this session (look at recent mtime)
MEMORY_INDEX="$SORTIARIUS_HOME/workspace/memory/index.md"
if [ -f "$MEMORY_INDEX" ]; then
  # If memory wasn't modified in the last 2 hours, suggest updating
  if [ "$(find "$MEMORY_INDEX" -mmin +120 2>/dev/null)" ]; then
    MESSAGES="${MESSAGES}Memory hasn't been updated recently. Consider whether anything from this session should be recorded.\n"
  fi
fi

# If we have messages, inject them as context for Claude's wrap-up
if [ -n "$MESSAGES" ]; then
  FULL_MSG="[Sortiarius Session End Check]\n${MESSAGES}Before ending, address these items or acknowledge them to Justin."
  jq -n --arg msg "$(echo -e "$FULL_MSG")" '{
    "decision": "block",
    "reason": $msg
  }'
  exit 0
fi

exit 0
