#!/usr/bin/env bash
# Sortiarius Session Learning Hook: Analyze session log and suggest memory updates
# Hook type: Stop (secondary — runs after session-stop.sh)
#
# Reads the session log, detects patterns, and generates memory update suggestions.
# This makes Sortiarius actually learn from interactions.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
INPUT="$(cat)"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
LOG_FILE="$SCRATCH_DIR/session-log.jsonl"

# Prevent infinite loop
STOP_HOOK_ACTIVE="$(echo "$INPUT" | jq -r '.stop_hook_active // false')"
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

# Only process if we have a session log
[ -f "$LOG_FILE" ] || exit 0

# Count commands by domain
TOTAL="$(wc -l < "$LOG_FILE" | tr -d ' ')"
[ "$TOTAL" -lt 3 ] && exit 0  # Skip trivial sessions

AZURE_COUNT="$(grep -c '"domain":"azure"' "$LOG_FILE" 2>/dev/null || true)"
PS_COUNT="$(grep -c '"domain":"powershell"' "$LOG_FILE" 2>/dev/null || true)"
DB_COUNT="$(grep -c '"domain":"database"' "$LOG_FILE" 2>/dev/null || true)"
ERRORS="$(grep -c '"exit_code":"[1-9]' "$LOG_FILE" 2>/dev/null || true)"

# Build learning summary
LEARN_MSG=""

if [ "$ERRORS" -gt 2 ]; then
  FAILED_CMDS="$(jq -r 'select(.exit_code != "0") | .command' "$LOG_FILE" 2>/dev/null | head -3 | tr '\n' '; ')"
  LEARN_MSG="${LEARN_MSG}This session had $ERRORS failed commands. Consider logging what went wrong to memory/solutions.md. Failed: ${FAILED_CMDS}\n"
fi

if [ "$AZURE_COUNT" -gt 3 ]; then
  LEARN_MSG="${LEARN_MSG}Heavy Azure session ($AZURE_COUNT commands). Any new patterns worth recording in memory/azure.md?\n"
fi

if [ "$PS_COUNT" -gt 3 ]; then
  LEARN_MSG="${LEARN_MSG}Heavy PowerShell session ($PS_COUNT commands). Any reusable patterns for memory/powershell.md?\n"
fi

if [ "$DB_COUNT" -gt 2 ]; then
  LEARN_MSG="${LEARN_MSG}Database operations detected ($DB_COUNT commands). Any incidents or solutions for memory/incidents.md?\n"
fi

# If substantial session, suggest a memory update
if [ -n "$LEARN_MSG" ]; then
  jq -n --arg msg "[Sortiarius Learning Summary — $TOTAL commands this session]\n${LEARN_MSG}Ask Justin if any of this should be persisted to memory." '{
    "systemMessage": $msg
  }'
fi

# Rotate the session log (keep it from growing forever)
if [ "$TOTAL" -gt 500 ]; then
  tail -100 "$LOG_FILE" > "$LOG_FILE.tmp" && mv "$LOG_FILE.tmp" "$LOG_FILE"
fi

exit 0
