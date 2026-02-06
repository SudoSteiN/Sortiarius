#!/usr/bin/env bash
# Sortiarius Workflow Guard: Enforce spec-before-code and plan-before-build
# Hook type: PreToolUse (matcher: Edit|Write)
#
# When writing code files in a project directory:
# - No SPEC.md → hard block (spec first!)
# - No PLAN.md → soft warn (allow but remind)
# - Stale PLAN.md (>7 days) → soft warn
#
# Excludes:
# - Sortiarius workspace files (skills, memory, hooks, etc.)
# - Non-code files (markdown, config, gitignore, etc.)
# - Files outside of project directories
set -uo pipefail

INPUT="$(cat)"

FILE_PATH="$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')"
[ -z "$FILE_PATH" ] && exit 0

# --- Skip Sortiarius internal files ---
SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
if echo "$FILE_PATH" | grep -qF "$SORTIARIUS_HOME"; then
  exit 0
fi

# --- Skip non-code files ---
# Allow markdown, config, gitignore, env examples, dockerfiles, yaml, json, toml, lock files
if echo "$FILE_PATH" | grep -qEi '\.(md|txt|yml|yaml|toml|json|lock|gitignore|dockerignore|env\.example|csv|svg|png|jpg|gif|ico|woff|ttf|eot)$|Dockerfile|Makefile|LICENSE|README|SPEC|PLAN|CHANGELOG'; then
  exit 0
fi

# --- Skip if the file is SPEC.md or PLAN.md itself (allow creating them) ---
BASENAME="$(basename "$FILE_PATH")"
if [ "$BASENAME" = "SPEC.md" ] || [ "$BASENAME" = "PLAN.md" ]; then
  exit 0
fi

# --- Determine project root ---
# Walk up from file path looking for project markers
DIR="$(dirname "$FILE_PATH")"
PROJECT_ROOT=""
while [ "$DIR" != "/" ] && [ "$DIR" != "." ]; do
  # Check for common project root markers
  if [ -f "$DIR/package.json" ] || [ -f "$DIR/Cargo.toml" ] || \
     [ -f "$DIR/pyproject.toml" ] || [ -f "$DIR/go.mod" ] || \
     [ -f "$DIR/requirements.txt" ] || [ -f "$DIR/pom.xml" ] || \
     [ -f "$DIR/build.gradle" ] || [ -f "$DIR/Gemfile" ] || \
     [ -f "$DIR/composer.json" ] || [ -f "$DIR/.git" ] || [ -d "$DIR/.git" ]; then
    PROJECT_ROOT="$DIR"
    break
  fi
  DIR="$(dirname "$DIR")"
done

# If not in a project directory, skip
[ -z "$PROJECT_ROOT" ] && exit 0

# --- Check for SPEC.md ---
if [ ! -f "$PROJECT_ROOT/SPEC.md" ]; then
  # Hard block: No spec means no code
  cat << DENY
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny","permissionDecisionReason":"BLOCKED: No SPEC.md found in $PROJECT_ROOT.\n\nSortiarius workflow requires a product spec before writing code.\nRun the product-spec skill first: ask the 4 questions, generate SPEC.md, get approval.\n\nIf this is a quick fix to an existing project, create a minimal SPEC.md with the project description and goals."}}
DENY
  exit 0
fi

# --- Check for PLAN.md (soft warn — allow but remind) ---
WARNINGS=""
if [ ! -f "$PROJECT_ROOT/PLAN.md" ]; then
  WARNINGS="WARNING: No PLAN.md found. Consider creating one to track progress across sessions.\n"
fi

# --- Check for stale PLAN.md (>7 days since last modification) ---
if [ -f "$PROJECT_ROOT/PLAN.md" ]; then
  if command -v stat >/dev/null 2>&1; then
    # Get file modification time
    MTIME="$(stat -c %Y "$PROJECT_ROOT/PLAN.md" 2>/dev/null || stat -f %m "$PROJECT_ROOT/PLAN.md" 2>/dev/null || echo "0")"
    NOW="$(date +%s)"
    AGE_DAYS=$(( (NOW - MTIME) / 86400 ))
    if [ "$AGE_DAYS" -gt 7 ]; then
      WARNINGS="${WARNINGS}WARNING: PLAN.md is ${AGE_DAYS} days old. Update it with current progress before continuing.\n"
    fi
  fi
fi

# --- If we have warnings, inject them as context (don't block) ---
if [ -n "$WARNINGS" ]; then
  jq -n --arg msg "$(echo -e "[Sortiarius Workflow Guard]\n${WARNINGS}")" '{
    "systemMessage": $msg
  }'
fi

exit 0
