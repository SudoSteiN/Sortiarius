#!/usr/bin/env bash
# Sortiarius Memory Writer: Commit pending memories at session end
# Hook type: Stop (synchronous, 10s timeout)
#
# Reads pending-memories.jsonl, deduplicates against existing memory files,
# formats and appends entries, then reports what was written.
# Absorbs all session-learn.sh functionality (domain stats, error rate
# warnings, log rotation, long-session tips).
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SCRATCH_DIR="$SORTIARIUS_HOME/workspace/scratch"
LOG_FILE="$SCRATCH_DIR/session-log.jsonl"
PENDING_FILE="$SCRATCH_DIR/pending-memories.jsonl"

INPUT="$(cat)"

# Prevent infinite loop
STOP_HOOK_ACTIVE="$(echo "$INPUT" | jq -r '.stop_hook_active // false')"
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

SESSION_ID="$(echo "$INPUT" | jq -r '.session_id // "unknown"')"
TODAY="$(date -u '+%Y-%m-%d')"
FILE_SIZE_CAP=20480  # 20KB
MAX_AUTO_ENTRIES=50
MESSAGES=""
WRITTEN_COUNT=0

# --- Write pending memories ---
if [ -f "$PENDING_FILE" ] && [ -s "$PENDING_FILE" ]; then
  while IFS= read -r entry; do
    [ -z "$entry" ] && continue

    TARGET="$(echo "$entry" | jq -r '.target_file // empty')"
    SUMMARY="$(echo "$entry" | jq -r '.summary // empty')"
    DETAIL="$(echo "$entry" | jq -r '.detail // empty')"
    CATEGORY="$(echo "$entry" | jq -r '.category // empty')"
    DOMAIN="$(echo "$entry" | jq -r '.domain // empty')"

    [ -z "$TARGET" ] || [ -z "$SUMMARY" ] && continue

    # --- Dedup layer 2: grep target file for key phrase ---
    # Extract first significant word from summary for matching
    KEY_PHRASE="$(echo "$SUMMARY" | sed 's/.*`\([^`]*\)`.*/\1/' | head -c 60)"
    if [ -f "$TARGET" ] && [ -n "$KEY_PHRASE" ]; then
      if grep -qF "$KEY_PHRASE" "$TARGET" 2>/dev/null; then
        continue  # Already in target file
      fi
    fi

    # --- Dedup layer 3: session marker check ---
    if [ -f "$TARGET" ]; then
      if grep -qF "session:$SESSION_ID" "$TARGET" 2>/dev/null; then
        # Session already wrote to this file — still allow if different category
        if grep -qF "auto-memory.*$CATEGORY.*session:$SESSION_ID" "$TARGET" 2>/dev/null; then
          continue
        fi
      fi
    fi

    # --- Bounded growth: file size cap ---
    if [ -f "$TARGET" ]; then
      FILE_SIZE="$(wc -c < "$TARGET" 2>/dev/null | tr -d ' ')"
      if [ "$FILE_SIZE" -gt "$FILE_SIZE_CAP" ]; then
        MESSAGES="${MESSAGES}SKIPPED write to $(basename "$TARGET") — file exceeds 20KB cap. Consider consolidating old entries.\n"
        continue
      fi

      # Count existing auto-entries
      AUTO_COUNT="$(grep -c '<!-- auto-memory:' "$TARGET" 2>/dev/null || echo 0)"
      if [ "$AUTO_COUNT" -ge "$MAX_AUTO_ENTRIES" ]; then
        MESSAGES="${MESSAGES}SKIPPED write to $(basename "$TARGET") — reached $MAX_AUTO_ENTRIES auto-entries. Consolidate before adding more.\n"
        continue
      fi
    fi

    # --- Format and append ---
    # Ensure target file exists
    mkdir -p "$(dirname "$TARGET")"
    if [ ! -f "$TARGET" ]; then
      echo "# $(basename "$TARGET" .md)" > "$TARGET"
      echo "" >> "$TARGET"
    fi

    {
      echo ""
      echo "### $SUMMARY"
      echo "<!-- auto-memory: $TODAY session:$SESSION_ID category:$CATEGORY domain:$DOMAIN -->"
      if [ -n "$DETAIL" ]; then
        echo "$DETAIL" | sed 's/\\n/\n/g'
      fi
      echo ""
    } >> "$TARGET"

    WRITTEN_COUNT=$((WRITTEN_COUNT + 1))
  done < "$PENDING_FILE"

  # Clean up pending file
  : > "$PENDING_FILE"
fi

# --- Absorbed from session-learn.sh: domain stats + warnings ---
if [ -f "$LOG_FILE" ]; then
  TOTAL="$(wc -l < "$LOG_FILE" | tr -d ' ')"

  if [ "$TOTAL" -ge 3 ]; then
    ERRORS="$(grep -c '"exit_code":"[1-9]' "$LOG_FILE" 2>/dev/null || echo "0")"

    # High error rate warning
    if [ "$ERRORS" -gt 0 ] && [ "$TOTAL" -gt 0 ]; then
      ERROR_RATE=$(( (ERRORS * 100) / TOTAL ))
      if [ "$ERROR_RATE" -gt 25 ]; then
        MESSAGES="${MESSAGES}High error rate (${ERROR_RATE}%) — ${ERRORS}/${TOTAL} commands failed. Consider self-improve skill to add rules preventing these mistakes.\n"
      fi
    fi

    # Long session tip
    if [ "$TOTAL" -gt 40 ]; then
      MESSAGES="${MESSAGES}Long session ($TOTAL commands). Consider worktrees for parallel work next time.\n"
    fi

    # Log rotation (keep last 100 when >500)
    if [ "$TOTAL" -gt 500 ]; then
      tail -100 "$LOG_FILE" > "$LOG_FILE.tmp" && mv "$LOG_FILE.tmp" "$LOG_FILE"
      MESSAGES="${MESSAGES}Rotated session log (was $TOTAL entries, kept last 100).\n"
    fi
  fi
fi

# --- Build output ---
if [ "$WRITTEN_COUNT" -gt 0 ]; then
  MESSAGES="Auto-memory: wrote $WRITTEN_COUNT entries to memory files.\n${MESSAGES}"
fi

if [ -n "$MESSAGES" ]; then
  FULL_MSG="[Sortiarius Memory Writer]\n${MESSAGES}"
  jq -n --arg msg "$(echo -e "$FULL_MSG")" '{"systemMessage": $msg}'
fi

exit 0
