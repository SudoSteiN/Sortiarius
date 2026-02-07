#!/usr/bin/env bash
# Sortiarius Memory Detector: Detect memorable events in real-time
# Hook type: PostToolUse (matcher: Bash), async, 5s timeout
#
# Analyzes the current command + recent session history to detect:
# - Error-Recovery: command fails → edit → command succeeds
# - Retry Storm: 3+ failures of same command before success
# - Dependency Install: cargo add, npm install, brew install, etc.
# - Environment Discovery: --version, which, rustup show, etc.
#
# Appends candidates to scratch/pending-memories.jsonl for memory-writer.sh
# to commit at session end.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
LOG_FILE="$SCRATCH_DIR/session-log.jsonl"
PENDING_FILE="$SCRATCH_DIR/pending-memories.jsonl"

INPUT="$(cat)"

# Parse current command info from stdin
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
EXIT_CODE="$(echo "$INPUT" | jq -r '.tool_result.exit_code // 0')"
TOOL_OUTPUT="$(echo "$INPUT" | jq -r '.tool_result.stdout // empty' | head -10)"
TOOL_STDERR="$(echo "$INPUT" | jq -r '.tool_result.stderr // empty' | head -5)"
SESSION_ID="$(echo "$INPUT" | jq -r '.session_id // empty')"
TIMESTAMP="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

# Skip trivial commands
case "$COMMAND" in
  ls*|cd*|echo*|pwd*|cat*|head*|tail*|"") exit 0 ;;
esac

mkdir -p "$SCRATCH_DIR"

# --- Domain detection (mirrors learning-tracker.sh) ---
detect_domain() {
  local cmd="$1"
  if echo "$cmd" | grep -qEi 'az\s|Get-Az|Set-Az|New-Az|Remove-Az'; then echo "azure"
  elif echo "$cmd" | grep -qEi 'powershell|pwsh|\.ps1|Get-|Set-|New-|Remove-|Invoke-'; then echo "powershell"
  elif echo "$cmd" | grep -qEi 'sql|Invoke-Sqlcmd|sqlcmd'; then echo "database"
  elif echo "$cmd" | grep -qEi 'cargo|rustc|rustup'; then echo "rust"
  elif echo "$cmd" | grep -qEi 'npm|yarn|pnpm|node|npx'; then echo "node"
  elif echo "$cmd" | grep -qEi 'python|pip|pytest'; then echo "python"
  elif echo "$cmd" | grep -qEi 'docker|docker-compose'; then echo "docker"
  elif echo "$cmd" | grep -qEi 'terraform|tf\s'; then echo "terraform"
  else echo "general"
  fi
}

# --- Command signature: strip args/paths to get canonical form ---
cmd_signature() {
  echo "$1" | sed -E 's|/[^ ]+||g; s/[0-9]+//g; s/\s+/ /g' | head -c 120
}

# --- Fingerprint: md5 of category + domain + signature ---
fingerprint() {
  echo "${1}:${2}:${3}" | md5 2>/dev/null || echo "${1}:${2}:${3}" | md5sum 2>/dev/null | cut -d' ' -f1
}

# --- Check if fingerprint already in pending file ---
is_duplicate() {
  local fp="$1"
  [ -f "$PENDING_FILE" ] && grep -qF "\"fingerprint\":\"$fp\"" "$PENDING_FILE" 2>/dev/null
}

# --- Append a memory candidate ---
append_candidate() {
  local category="$1" domain="$2" summary="$3" detail="$4" fp="$5" target="$6"
  jq -n -c \
    --arg ts "$TIMESTAMP" \
    --arg cat "$category" \
    --arg dom "$domain" \
    --arg sum "$summary" \
    --arg det "$detail" \
    --arg fp "$fp" \
    --arg tgt "$target" \
    --arg sess "$SESSION_ID" \
    '{timestamp: $ts, category: $cat, domain: $dom, summary: $sum, detail: $det, fingerprint: $fp, target_file: $tgt, session: $sess}' \
    >> "$PENDING_FILE" 2>/dev/null
}

# --- Route category+domain to target memory file ---
route_target() {
  local category="$1" domain="$2"
  local base="$SORTIARIUS_HOME/workspace/memory"
  case "$category" in
    error-recovery)
      case "$domain" in
        azure) echo "$base/azure.md" ;;
        powershell) echo "$base/powershell.md" ;;
        database) echo "$base/incidents.md" ;;
        *) echo "$base/solutions.md" ;;
      esac
      ;;
    retry-storm) echo "$base/solutions.md" ;;
    environment|dependency) echo "$base/index.md" ;;
    *) echo "$base/solutions.md" ;;
  esac
}

DOMAIN="$(detect_domain "$COMMAND")"
CMD_SIG="$(cmd_signature "$COMMAND")"

# ==========================================================
# Signal 1: Error-Recovery
# Current command succeeded (exit 0) AND a recent command with
# similar signature failed (exit != 0)
# ==========================================================
if [ "$EXIT_CODE" = "0" ] && [ -f "$LOG_FILE" ]; then
  # Look at last 20 log entries for a failed command with similar base command
  BASE_CMD="$(echo "$COMMAND" | awk '{print $1}')"
  RECENT_FAIL="$(tail -20 "$LOG_FILE" 2>/dev/null | jq -r "select(.exit_code != \"0\") | select(.command | startswith(\"$BASE_CMD\")) | .command" 2>/dev/null | tail -1)"

  if [ -n "$RECENT_FAIL" ]; then
    FAIL_STDERR="$(tail -20 "$LOG_FILE" 2>/dev/null | jq -r "select(.exit_code != \"0\") | select(.command | startswith(\"$BASE_CMD\")) | .stderr_preview // .output_preview // \"\"" 2>/dev/null | tail -1)"
    FP="$(fingerprint "error-recovery" "$DOMAIN" "$CMD_SIG")"
    TARGET="$(route_target "error-recovery" "$DOMAIN")"

    if ! is_duplicate "$FP"; then
      SUMMARY="Fixed: \`$BASE_CMD\` — was failing, now succeeds"
      DETAIL="Failed cmd: $RECENT_FAIL\nError: $FAIL_STDERR\nFix applied, then: $COMMAND"
      append_candidate "error-recovery" "$DOMAIN" "$SUMMARY" "$DETAIL" "$FP" "$TARGET"
    fi
  fi
fi

# ==========================================================
# Signal 2: Retry Storm
# 3+ failures of same base command in recent history, now succeeds
# ==========================================================
if [ "$EXIT_CODE" = "0" ] && [ -f "$LOG_FILE" ]; then
  BASE_CMD="$(echo "$COMMAND" | awk '{print $1}')"
  FAIL_COUNT="$(tail -20 "$LOG_FILE" 2>/dev/null | jq -r "select(.exit_code != \"0\") | select(.command | startswith(\"$BASE_CMD\")) | .command" 2>/dev/null | wc -l | tr -d ' ')"

  if [ "$FAIL_COUNT" -ge 3 ]; then
    FP="$(fingerprint "retry-storm" "$DOMAIN" "$CMD_SIG")"
    TARGET="$(route_target "retry-storm" "$DOMAIN")"

    if ! is_duplicate "$FP"; then
      SUMMARY="Retry storm resolved: \`$BASE_CMD\` failed $FAIL_COUNT times before succeeding"
      DETAIL="Command: $COMMAND\nFailed $FAIL_COUNT times in recent history before this success."
      append_candidate "retry-storm" "$DOMAIN" "$SUMMARY" "$DETAIL" "$FP" "$TARGET"
    fi
  fi
fi

# ==========================================================
# Signal 3: Dependency Install
# ==========================================================
case "$COMMAND" in
  "cargo add"*|"npm install"*|"npm i "*|"yarn add"*|"pnpm add"*|"pip install"*|"brew install"*|"apt install"*|"apt-get install"*)
    if [ "$EXIT_CODE" = "0" ]; then
      FP="$(fingerprint "dependency" "$DOMAIN" "$CMD_SIG")"
      TARGET="$(route_target "dependency" "$DOMAIN")"

      if ! is_duplicate "$FP"; then
        PKG="$(echo "$COMMAND" | awk '{for(i=2;i<=NF;i++) if($i !~ /^-/) {print $i; exit}}')"
        SUMMARY="Installed: \`$PKG\` via \`$(echo "$COMMAND" | awk '{print $1, $2}')\`"
        DETAIL="Full command: $COMMAND"
        append_candidate "dependency" "$DOMAIN" "$SUMMARY" "$DETAIL" "$FP" "$TARGET"
      fi
    fi
    ;;
esac

# ==========================================================
# Signal 4: Environment Discovery
# ==========================================================
case "$COMMAND" in
  *"--version"*|"which "*|"where "*|"rustup show"*|"rustup target"*|"node -v"*|"python --version"*|"dotnet --info"*)
    if [ "$EXIT_CODE" = "0" ] && [ -n "$TOOL_OUTPUT" ]; then
      FP="$(fingerprint "environment" "$DOMAIN" "$CMD_SIG")"
      TARGET="$(route_target "environment" "$DOMAIN")"

      if ! is_duplicate "$FP"; then
        SUMMARY="Environment: \`$(echo "$COMMAND" | head -c 60)\`"
        DETAIL="Output: $(echo "$TOOL_OUTPUT" | head -3 | tr '\n' ' ')"
        append_candidate "environment" "$DOMAIN" "$SUMMARY" "$DETAIL" "$FP" "$TARGET"
      fi
    fi
    ;;
esac

exit 0
