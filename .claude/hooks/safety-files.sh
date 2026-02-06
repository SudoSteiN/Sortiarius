#!/usr/bin/env bash
# Sortiarius Safety Hook: Protect sensitive files from modification
# Hook type: PreToolUse (matcher: Edit|Write)
#
# Prevents Claude from modifying files that should never be auto-edited.
set -uo pipefail

INPUT="$(cat)"

FILE_PATH="$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')"
[ -z "$FILE_PATH" ] && exit 0

# --- Protected file patterns ---

# Environment and secret files
if echo "$FILE_PATH" | grep -qEi '\.(env|pem|key|pfx|p12)$|credentials\.json|secrets\.json|\.env\.|botToken|appToken'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Cannot modify secrets/credentials file. These files must be edited manually for security."}}'
  exit 0
fi

# Git internals
if echo "$FILE_PATH" | grep -qE '\.git/'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Cannot modify .git internals directly."}}'
  exit 0
fi

# SSH keys
if echo "$FILE_PATH" | grep -qE '\.ssh/(id_|authorized_keys|known_hosts)'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Cannot modify SSH keys or config. Edit these manually."}}'
  exit 0
fi

exit 0
