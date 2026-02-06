#!/usr/bin/env bash
# SteinBot Learning Tracker: Log significant operations for pattern detection
# Hook type: PostToolUse (matcher: Bash)
#
# Tracks commands that were run so SteinBot can learn patterns over time.
# Writes to workspace/scratch/session-log.jsonl (gitignored).
# This is async — doesn't block Claude's workflow.
set -uo pipefail

STEINBOT_HOME="${STEINBOT_HOME:-$HOME/SteinBot}"
SCRATCH_DIR="$STEINBOT_HOME/workspace/scratch"
LOG_FILE="$SCRATCH_DIR/session-log.jsonl"

INPUT="$(cat)"

# Ensure scratch directory exists
mkdir -p "$SCRATCH_DIR"

COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
TOOL_OUTPUT="$(echo "$INPUT" | jq -r '.tool_result.stdout // empty' | head -5)"
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
  '{timestamp: $ts, command: $cmd, domain: $domain, cwd: $cwd, exit_code: $exit_code, session: $session, output_preview: $output_preview}' \
  >> "$LOG_FILE" 2>/dev/null

exit 0
