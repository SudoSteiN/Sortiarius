#!/usr/bin/env bash
# Sortiarius Learning Tracker: Log operations + context health monitoring
# Hook type: PostToolUse (matcher: Bash)
#
# Tracks commands that were run so Sortiarius can learn patterns over time.
# Also monitors context health — warns when sessions get too verbose.
# Writes to workspace/scratch/session-log.jsonl (gitignored).
# This is async — doesn't block Claude's workflow.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
LOG_FILE="$SCRATCH_DIR/session-log.jsonl"

INPUT="$(cat)"

# Ensure scratch directory exists
mkdir -p "$SCRATCH_DIR"

COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
TOOL_OUTPUT="$(echo "$INPUT" | jq -r '.tool_result.stdout // empty' | head -10)"
TOOL_STDERR="$(echo "$INPUT" | jq -r '.tool_result.stderr // empty' | head -5)"
EXIT_CODE="$(echo "$INPUT" | jq -r '.tool_result.exit_code // 0')"
TIMESTAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
CWD="$(echo "$INPUT" | jq -r '.cwd // empty')"
SESSION_ID="$(echo "$INPUT" | jq -r '.session_id // empty')"

# Only log non-trivial commands (skip ls, cd, echo, etc.)
case "$COMMAND" in
  ls*|cd*|echo*|pwd*|cat*|head*|tail*|"") exit 0 ;;
esac

# Detect domain from command content
DOMAIN="general"
if echo "$COMMAND" | grep -qEi 'az\s|Get-Az|Set-Az|New-Az|Remove-Az'; then
  DOMAIN="azure"
elif echo "$COMMAND" | grep -qEi 'powershell|pwsh|\.ps1|Get-|Set-|New-|Remove-|Invoke-'; then
  DOMAIN="powershell"
elif echo "$COMMAND" | grep -qEi 'sql|Invoke-Sqlcmd|sqlcmd'; then
  DOMAIN="database"
elif echo "$COMMAND" | grep -qEi 'git\s'; then
  DOMAIN="git"
elif echo "$COMMAND" | grep -qEi 'npm|yarn|pnpm|node|npx'; then
  DOMAIN="node"
elif echo "$COMMAND" | grep -qEi 'python|pip|pytest'; then
  DOMAIN="python"
elif echo "$COMMAND" | grep -qEi 'docker|docker-compose'; then
  DOMAIN="docker"
elif echo "$COMMAND" | grep -qEi 'terraform|tf\s'; then
  DOMAIN="terraform"
fi

# Write structured log entry
jq -n -c \
  --arg ts "$TIMESTAMP" \
  --arg cmd "$COMMAND" \
  --arg domain "$DOMAIN" \
  --arg cwd "$CWD" \
  --arg exit_code "$EXIT_CODE" \
  --arg session "$SESSION_ID" \
  --arg output_preview "$TOOL_OUTPUT" \
  --arg stderr_preview "$TOOL_STDERR" \
  '{timestamp: $ts, command: $cmd, domain: $domain, cwd: $cwd, exit_code: $exit_code, session: $session, output_preview: $output_preview, stderr_preview: $stderr_preview}' \
  >> "$LOG_FILE" 2>/dev/null

# --- Context Health Monitoring ---
# Count total operations this session and warn at thresholds
TOTAL_OPS="$(wc -l < "$LOG_FILE" 2>/dev/null | tr -d ' ')"
ERRORS="$(grep -c '"exit_code":"[1-9]' "$LOG_FILE" 2>/dev/null || echo "0")"

# Warn at thresholds about context health
case "$TOTAL_OPS" in
  30)
    WARNING="[Sortiarius Context Health]\n30 bash operations this session. Consider:\n- Delegating independent sub-tasks to 'sortiarius agent'\n- Summarizing progress so far\n- Checking if the current approach is efficient"
    jq -n --arg msg "$(echo -e "$WARNING")" '{"systemMessage": $msg}'
    ;;
  60)
    WARNING="[Sortiarius Context Health]\n60 bash operations — this is a long session. Strongly consider:\n- Breaking remaining work into sub-agent tasks\n- Updating PLAN.md with progress so far\n- Whether this session should be wrapped up and continued fresh"
    jq -n --arg msg "$(echo -e "$WARNING")" '{"systemMessage": $msg}'
    ;;
esac

# Warn if error rate is high (>30% of commands failing)
if [ "$TOTAL_OPS" -gt 10 ] && [ "$ERRORS" -gt 0 ]; then
  ERROR_RATE=$((ERRORS * 100 / TOTAL_OPS))
  if [ "$ERROR_RATE" -gt 30 ]; then
    case "$ERRORS" in
      # Only warn once at specific error counts to avoid spam
      4|8|12)
        WARNING="[Sortiarius Context Health]\nHigh error rate: $ERRORS/$TOTAL_OPS commands failed ($ERROR_RATE%). Consider:\n- Stepping back to analyze the root cause\n- Reading error output more carefully before retrying\n- Whether the approach needs to change"
        jq -n --arg msg "$(echo -e "$WARNING")" '{"systemMessage": $msg}'
        ;;
    esac
  fi
fi

exit 0
