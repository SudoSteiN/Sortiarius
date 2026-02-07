#!/usr/bin/env bash
# Sortiarius Secret Scanner: Detect leaked credentials in command output
# Hook type: PostToolUse (matcher: Bash)
#
# Scans command output for patterns that look like exposed secrets:
# - API keys, tokens, connection strings, passwords in stdout
# - AWS, Azure, GCP credential patterns
# - Generic high-entropy strings in suspicious context
#
# This is a safety net — warns after the fact since we can't undo output.
# Async so it doesn't slow down Claude's workflow.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
ALERT_LOG="$SCRATCH_DIR/secret-alerts.log"
INPUT="$(cat)"

COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
OUTPUT="$(echo "$INPUT" | jq -r '.tool_result.stdout // empty')"

# Skip if no output or trivial commands
[ -z "$OUTPUT" ] && exit 0

# Commands that legitimately deal with secrets (don't flag these)
if echo "$COMMAND" | grep -qEi 'vault|secret|credential|keygen|ssh-add|gpg'; then
  exit 0
fi

ALERTS=""

# --- AWS patterns ---
if echo "$OUTPUT" | grep -qEi 'AKIA[0-9A-Z]{16}'; then
  ALERTS="${ALERTS}AWS Access Key ID detected in output\n"
fi

if echo "$OUTPUT" | grep -qEi 'aws_secret_access_key\s*[=:]\s*[A-Za-z0-9/+=]{40}'; then
  ALERTS="${ALERTS}AWS Secret Access Key detected in output\n"
fi

# --- Azure patterns ---
if echo "$OUTPUT" | grep -qEi 'DefaultEndpointsProtocol=https;AccountName=.*AccountKey='; then
  ALERTS="${ALERTS}Azure Storage connection string detected in output\n"
fi

if echo "$OUTPUT" | grep -qEi 'SharedAccessSignature='; then
  ALERTS="${ALERTS}Azure SAS token detected in output\n"
fi

# --- Generic patterns ---
if echo "$OUTPUT" | grep -qEi '(password|passwd|pwd)\s*[=:]\s*[^\s]{8,}'; then
  # Exclude common false positives (placeholder values, example configs)
  if ! echo "$OUTPUT" | grep -qEi 'password.*change-me|password.*example|password.*placeholder|password.*\*\*\*'; then
    ALERTS="${ALERTS}Password value detected in output\n"
  fi
fi

if echo "$OUTPUT" | grep -qEi '(api[_-]?key|apikey|api[_-]?token)\s*[=:]\s*[A-Za-z0-9_\-]{20,}'; then
  ALERTS="${ALERTS}API key/token detected in output\n"
fi

if echo "$OUTPUT" | grep -qEi 'Bearer\s+[A-Za-z0-9\-_.~+/]+=*'; then
  ALERTS="${ALERTS}Bearer token detected in output\n"
fi

# --- GitHub/GitLab tokens ---
if echo "$OUTPUT" | grep -qEi 'gh[pousr]_[A-Za-z0-9_]{36,}'; then
  ALERTS="${ALERTS}GitHub token detected in output\n"
fi

if echo "$OUTPUT" | grep -qEi 'glpat-[A-Za-z0-9\-]{20,}'; then
  ALERTS="${ALERTS}GitLab personal access token detected in output\n"
fi

# --- Private keys ---
if echo "$OUTPUT" | grep -qF 'BEGIN RSA PRIVATE KEY' || \
   echo "$OUTPUT" | grep -qF 'BEGIN OPENSSH PRIVATE KEY' || \
   echo "$OUTPUT" | grep -qF 'BEGIN EC PRIVATE KEY'; then
  ALERTS="${ALERTS}CRITICAL: Private key detected in output!\n"
fi

# --- Connection strings ---
if echo "$OUTPUT" | grep -qEi '(mysql|postgres|mongodb|redis|amqp)://[^:]+:[^@]+@'; then
  ALERTS="${ALERTS}Database connection string with credentials detected in output\n"
fi

# --- If alerts found, warn ---
if [ -n "$ALERTS" ]; then
  TIMESTAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

  # Log to alert file for audit
  mkdir -p "$SCRATCH_DIR"
  echo -e "[$TIMESTAMP] Command: $COMMAND\nAlerts:\n$ALERTS---" >> "$ALERT_LOG" 2>/dev/null

  # Inject warning into context
  WARNING="[Sortiarius Secret Scanner - ALERT]\nPotential credentials detected in command output:\n${ALERTS}\nAction needed:\n- Rotate any exposed credentials immediately\n- Check if the output was logged anywhere persistent\n- Use 'az keyvault secret show' or env vars instead of echoing secrets"

  jq -n --arg msg "$(echo -e "$WARNING")" '{
    "systemMessage": $msg
  }'
fi

exit 0
