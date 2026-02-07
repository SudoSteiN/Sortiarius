#!/usr/bin/env bash
# Sortiarius Pre-Push Guard: Ensure tests run before pushing code
# Hook type: PreToolUse (matcher: Bash)
#
# When detecting `git push`:
# 1. Check if a test runner exists (package.json scripts.test, pytest, etc.)
# 2. Check session log for test execution this session
# 3. If tests exist but weren't run → warn (allow but inject strong warning)
# 4. If no tests exist → silently allow
set -uo pipefail

INPUT="$(cat)"
COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"

# Only care about git push commands
echo "$COMMAND" | grep -qE '^\s*git\s+push' || exit 0

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SESSION_LOG="$SORTIARIUS_HOME/workspace/scratch/session-log.jsonl"

# --- Detect project root from CWD ---
CWD="$(echo "$INPUT" | jq -r '.cwd // empty')"
[ -z "$CWD" ] && CWD="$(pwd)"
PROJECT_ROOT=""
DIR="$CWD"
while [ "$DIR" != "/" ] && [ "$DIR" != "." ]; do
  if [ -f "$DIR/package.json" ] || [ -f "$DIR/Cargo.toml" ] || \
     [ -f "$DIR/pyproject.toml" ] || [ -f "$DIR/go.mod" ] || \
     [ -f "$DIR/requirements.txt" ] || [ -f "$DIR/Makefile" ]; then
    PROJECT_ROOT="$DIR"
    break
  fi
  DIR="$(dirname "$DIR")"
done

[ -z "$PROJECT_ROOT" ] && exit 0

# --- Detect if test runner exists ---
HAS_TESTS=false
TEST_CMD=""

if [ -f "$PROJECT_ROOT/package.json" ]; then
  test_script="$(jq -r '.scripts.test // empty' "$PROJECT_ROOT/package.json" 2>/dev/null)"
  if [ -n "$test_script" ] && [ "$test_script" != "echo \"Error: no test specified\" && exit 1" ]; then
    HAS_TESTS=true
    TEST_CMD="npm test"
  fi
fi

if [ -f "$PROJECT_ROOT/pyproject.toml" ] || [ -f "$PROJECT_ROOT/pytest.ini" ] || \
   [ -f "$PROJECT_ROOT/setup.cfg" ] || [ -d "$PROJECT_ROOT/tests" ] || \
   [ -d "$PROJECT_ROOT/test" ]; then
  if command -v pytest >/dev/null 2>&1 || [ -f "$PROJECT_ROOT/pyproject.toml" ]; then
    HAS_TESTS=true
    TEST_CMD="pytest"
  fi
fi

if [ -f "$PROJECT_ROOT/Cargo.toml" ]; then
  HAS_TESTS=true
  TEST_CMD="cargo test"
fi

if [ -f "$PROJECT_ROOT/go.mod" ]; then
  HAS_TESTS=true
  TEST_CMD="go test ./..."
fi

# No test infrastructure found — nothing to enforce
$HAS_TESTS || exit 0

# --- Check if tests were run this session ---
TESTS_RAN=false
if [ -f "$SESSION_LOG" ]; then
  # Look for test commands in session log (last 200 entries to keep fast)
  if tail -200 "$SESSION_LOG" 2>/dev/null | grep -qEi '"command".*(npm test|npx jest|pytest|cargo test|go test|vitest|mocha|jest|make test|yarn test)'; then
    TESTS_RAN=true
  fi
fi

if ! $TESTS_RAN; then
  # Warn but don't block — inject a strong warning
  WARNING="[Sortiarius Pre-Push Guard]\nWARNING: You are pushing code but tests were NOT run this session.\n\nDetected test runner: $TEST_CMD\nConsider running tests before pushing to catch issues early.\n\nTo run tests: $TEST_CMD"
  jq -n --arg msg "$(echo -e "$WARNING")" '{
    "systemMessage": $msg
  }'
fi

exit 0
