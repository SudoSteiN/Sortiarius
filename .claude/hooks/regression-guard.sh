#!/usr/bin/env bash
# Sortiarius Regression Guard: Remind to re-run tests after code modifications
# Hook type: PostToolUse (matcher: Edit|Write)
#
# After code files are modified, checks if tests exist for the project
# and injects a reminder to re-run them. Tracks modified files in a
# session scratch file so session-stop can do a final quality gate.
#
# Async so it doesn't block the editing flow.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
MODIFIED_LOG="$SCRATCH_DIR/session-modified-files.log"
INPUT="$(cat)"

FILE_PATH="$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')"
[ -z "$FILE_PATH" ] && exit 0

# Skip non-code files
if echo "$FILE_PATH" | grep -qEi '\.(md|txt|yml|yaml|toml|json|lock|gitignore|csv|svg|png|jpg|log)$|SPEC|PLAN|README|LICENSE|CHANGELOG|Dockerfile|Makefile'; then
  exit 0
fi

# Skip Sortiarius internal files
if echo "$FILE_PATH" | grep -qF "$SORTIARIUS_HOME"; then
  exit 0
fi

# Log the modification for quality gate at session end
mkdir -p "$SCRATCH_DIR"
echo "$(date -u '+%Y-%m-%dT%H:%M:%SZ') $FILE_PATH" >> "$MODIFIED_LOG" 2>/dev/null

# Count how many code files modified this session
MOD_COUNT="$(wc -l < "$MODIFIED_LOG" 2>/dev/null | tr -d ' ')"
MOD_UNIQUE="$(awk '{print $2}' "$MODIFIED_LOG" 2>/dev/null | sort -u | wc -l | tr -d ' ')"

# Only inject reminders at thresholds (not every edit — that would be annoying)
# Thresholds: 5, 15, 30 modifications
case "$MOD_COUNT" in
  5|15|30)
    # Check if tests exist in the project
    DIR="$(dirname "$FILE_PATH")"
    PROJECT_ROOT=""
    while [ "$DIR" != "/" ] && [ "$DIR" != "." ]; do
      if [ -f "$DIR/package.json" ] || [ -f "$DIR/Cargo.toml" ] || \
         [ -f "$DIR/pyproject.toml" ] || [ -d "$DIR/tests" ] || \
         [ -d "$DIR/test" ] || [ -d "$DIR/__tests__" ]; then
        PROJECT_ROOT="$DIR"
        break
      fi
      DIR="$(dirname "$DIR")"
    done

    if [ -n "$PROJECT_ROOT" ]; then
      WARNING="[Sortiarius Regression Guard]\n${MOD_UNIQUE} code files modified this session (${MOD_COUNT} total edits).\nConsider running tests to catch regressions before they compound."
      jq -n --arg msg "$(echo -e "$WARNING")" '{
        "systemMessage": $msg
      }'
    fi
    ;;
esac

exit 0
