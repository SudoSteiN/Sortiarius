#!/usr/bin/env bash
# Sortiarius Code Validator: Run language-appropriate checks after Write/Edit
# Used by builder agent PostToolUse hooks
#
# Detects file type from the tool input and runs the appropriate linter/checker.
# Returns JSON: {"decision": "block", "reason": "..."} on failure to force fix.
set -uo pipefail

INPUT="$(cat)"
FILE_PATH="$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')"

# Skip if no file path
[ -z "$FILE_PATH" ] && exit 0

# Skip non-code files
case "$FILE_PATH" in
  *.md|*.txt|*.json|*.yaml|*.yml|*.toml|*.lock|*.css|*.html|*.svg) exit 0 ;;
esac

# Detect language and run appropriate validator
case "$FILE_PATH" in
  *.py)
    # Python: ruff lint check
    if command -v ruff >/dev/null 2>&1; then
      RESULT="$(ruff check "$FILE_PATH" 2>&1)"
      if [ $? -ne 0 ]; then
        jq -n --arg reason "Python lint failed for $(basename "$FILE_PATH"):\n$RESULT\nFix the lint errors before proceeding." \
          '{"decision": "block", "reason": $reason}'
        exit 0
      fi
    fi
    # Python: type check
    if command -v mypy >/dev/null 2>&1; then
      RESULT="$(mypy "$FILE_PATH" --no-error-summary 2>&1)"
      if [ $? -ne 0 ]; then
        jq -n --arg reason "Type check failed for $(basename "$FILE_PATH"):\n$RESULT" \
          '{"decision": "block", "reason": $reason}'
        exit 0
      fi
    fi
    ;;

  *.rs)
    # Rust: cargo check (only if in a cargo project)
    CARGO_DIR="$(dirname "$FILE_PATH")"
    while [ "$CARGO_DIR" != "/" ]; do
      if [ -f "$CARGO_DIR/Cargo.toml" ]; then
        RESULT="$(cargo check --manifest-path "$CARGO_DIR/Cargo.toml" 2>&1)"
        if [ $? -ne 0 ]; then
          jq -n --arg reason "Rust compile check failed:\n$(echo "$RESULT" | tail -20)" \
            '{"decision": "block", "reason": $reason}'
          exit 0
        fi
        break
      fi
      CARGO_DIR="$(dirname "$CARGO_DIR")"
    done
    ;;

  *.ts|*.tsx)
    # TypeScript: tsc --noEmit (only if tsconfig exists)
    TS_DIR="$(dirname "$FILE_PATH")"
    while [ "$TS_DIR" != "/" ]; do
      if [ -f "$TS_DIR/tsconfig.json" ]; then
        RESULT="$(npx tsc --noEmit --project "$TS_DIR/tsconfig.json" 2>&1)"
        if [ $? -ne 0 ]; then
          jq -n --arg reason "TypeScript check failed:\n$(echo "$RESULT" | tail -20)" \
            '{"decision": "block", "reason": $reason}'
          exit 0
        fi
        break
      fi
      TS_DIR="$(dirname "$TS_DIR")"
    done
    ;;

  *.sh)
    # Shell: basic syntax check
    RESULT="$(bash -n "$FILE_PATH" 2>&1)"
    if [ $? -ne 0 ]; then
      jq -n --arg reason "Shell syntax error in $(basename "$FILE_PATH"):\n$RESULT" \
        '{"decision": "block", "reason": $reason}'
      exit 0
    fi
    ;;
esac

# All checks passed (or no checker available)
exit 0
