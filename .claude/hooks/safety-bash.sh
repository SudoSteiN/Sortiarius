#!/usr/bin/env bash
# SteinBot Safety Hook: Validate bash commands before execution
# Hook type: PreToolUse (matcher: Bash)
#
# Blocks dangerous commands deterministically — Claude can't override these.
# Exit 0 + JSON = allow/deny decision
# Exit 0 with no output = allow
# Exit 2 = block with stderr message
set -uo pipefail

STEINBOT_HOME="${STEINBOT_HOME:-$HOME/SteinBot}"
INPUT="$(cat)"

COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
[ -z "$COMMAND" ] && exit 0

# --- Destructive command patterns (BLOCKED unconditionally) ---

# Filesystem destruction
if echo "$COMMAND" | grep -qE '^\s*rm\s+(-[a-zA-Z]*)?r[a-zA-Z]*f|^\s*rm\s+(-[a-zA-Z]*)?f[a-zA-Z]*r'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: rm -rf detected. SteinBot safety hook requires explicit confirmation for recursive force-delete. Ask Justin to confirm the exact path."}}'
  exit 0
fi

# Disk destruction
if echo "$COMMAND" | grep -qEi '>\s*/dev/sd|dd\s+of=/dev/'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Direct disk write detected. This is never allowed."}}'
  exit 0
fi

# Fork bomb
if echo "$COMMAND" | grep -qF ':(){ :|:& };:'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Fork bomb detected."}}'
  exit 0
fi

# --- SQL injection patterns (block unguarded destructive SQL) ---

# DROP TABLE/DATABASE without confirmation
if echo "$COMMAND" | grep -qEi 'DROP\s+(TABLE|DATABASE)'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: DROP TABLE/DATABASE detected. Generate a rollback script first and get explicit confirmation from Justin."}}'
  exit 0
fi

# DELETE without WHERE clause
if echo "$COMMAND" | grep -qEi 'DELETE\s+FROM' && ! echo "$COMMAND" | grep -qEi 'WHERE'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: DELETE FROM without WHERE clause. SteinBot domain rules require WHERE clause confirmation for destructive queries."}}'
  exit 0
fi

# TRUNCATE TABLE
if echo "$COMMAND" | grep -qEi 'TRUNCATE\s+TABLE'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: TRUNCATE TABLE detected. Generate a rollback script first and get explicit confirmation."}}'
  exit 0
fi

# --- PowerShell safety: Remove-Az* without -WhatIf ---

if echo "$COMMAND" | grep -qEi 'Remove-Az' && ! echo "$COMMAND" | grep -qEi '\-WhatIf'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Remove-Az* without -WhatIf. SteinBot domain rules require -WhatIf before destructive Azure operations. Re-run with -WhatIf first."}}'
  exit 0
fi

# Remove-AzResourceGroup specifically (extra dangerous)
if echo "$COMMAND" | grep -qEi 'Remove-AzResourceGroup'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Remove-AzResourceGroup is extremely destructive. This requires explicit confirmation from Justin with the exact resource group name."}}'
  exit 0
fi

# --- Azure CLI destructive operations without --yes flag (auto-confirm) ---

if echo "$COMMAND" | grep -qEi 'az\s+(group|vm|sql|keyvault|storage)\s+delete' && ! echo "$COMMAND" | grep -qEi '\-\-no-wait|\-\-yes'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Azure resource deletion detected. Get explicit confirmation from Justin before deleting Azure resources."}}'
  exit 0
fi

# --- Production config protection ---

if echo "$COMMAND" | grep -qEi '(sed|awk|tee|>)\s.*(prod|production|\.env|credentials|secrets)'; then
  echo '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: Potential modification of production config or secrets file. Verify with Justin before proceeding."}}'
  exit 0
fi

# All checks passed — allow the command
exit 0
