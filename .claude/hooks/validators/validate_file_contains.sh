#!/usr/bin/env bash
# Sortiarius Validator: Check that a file in a directory contains required content
# Used by self-validating commands (e.g., /plan_w_team Stop hook)
#
# Usage: validate_file_contains.sh --directory <dir> --extension <ext> --contains '<text>' [--contains '<text2>']
#
# Exit 0 = pass, Exit 1 = fail (forces agent to continue)
set -uo pipefail

DIRECTORY=""
EXTENSION=""
CONTAINS=()

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --directory) DIRECTORY="$2"; shift 2 ;;
    --extension) EXTENSION="$2"; shift 2 ;;
    --contains) CONTAINS+=("$2"); shift 2 ;;
    *) shift ;;
  esac
done

[ -z "$DIRECTORY" ] && { echo "error: --directory required" >&2; exit 1; }
[ -z "$EXTENSION" ] && { echo "error: --extension required" >&2; exit 1; }
[ ${#CONTAINS[@]} -eq 0 ] && { echo "error: at least one --contains required" >&2; exit 1; }

# Find the most recently modified file matching the pattern
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
TARGET_DIR="$PROJECT_DIR/$DIRECTORY"

if [ ! -d "$TARGET_DIR" ]; then
  echo "VALIDATION FAILED: Directory '$DIRECTORY' does not exist."
  echo "ACTION REQUIRED: Create the directory and write the required file."
  exit 1
fi

# Find newest file with matching extension
LATEST_FILE="$(find "$TARGET_DIR" -maxdepth 1 -name "*${EXTENSION}" -type f -printf '%T@ %p\n' 2>/dev/null | sort -rn | head -1 | cut -d' ' -f2-)"

if [ -z "$LATEST_FILE" ]; then
  echo "VALIDATION FAILED: No *${EXTENSION} file found in ${DIRECTORY}/."
  echo "ACTION REQUIRED: Use the Write tool to create a file in the ${DIRECTORY}/ directory."
  echo "Do not stop until the file has been created."
  exit 1
fi

# Check each required string
MISSING=()
for required in "${CONTAINS[@]}"; do
  if ! grep -qF "$required" "$LATEST_FILE"; then
    MISSING+=("$required")
  fi
done

if [ ${#MISSING[@]} -gt 0 ]; then
  echo "VALIDATION FAILED: File $(basename "$LATEST_FILE") is missing required sections:"
  for m in "${MISSING[@]}"; do
    echo "  - $m"
  done
  echo ""
  echo "ACTION REQUIRED: Add the missing sections to $LATEST_FILE."
  echo "Do not stop until all sections are present."
  exit 1
fi

# All checks passed
exit 0
