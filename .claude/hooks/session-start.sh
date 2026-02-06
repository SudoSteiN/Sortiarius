#!/usr/bin/env bash
# SteinBot Context Injection Hook: Load skill manifest + memory on session start
# Hook type: SessionStart
#
# Injects a compact skill manifest and memory summary into every session
# so Claude has warm context without needing to scan the filesystem.
set -uo pipefail

STEINBOT_HOME="${STEINBOT_HOME:-$HOME/SteinBot}"
SKILLS_DIR="$STEINBOT_HOME/workspace/skills"
MEMORY_DIR="$STEINBOT_HOME/workspace/memory"

# Build skill manifest
SKILL_MANIFEST=""
if [ -d "$SKILLS_DIR" ]; then
  for skill_dir in "$SKILLS_DIR"/*/; do
    [ -d "$skill_dir" ] || continue
    skill_file="$skill_dir/SKILL.md"
    [ -f "$skill_file" ] || continue

    name="$(awk '/^name:/{gsub(/^name: */, ""); print; exit}' "$skill_file")"
    desc="$(awk '/^description:/{gsub(/^description: */, ""); print; exit}' "$skill_file")"
    triggers="$(awk '/^triggers:/{found=1; next} found && /^  - /{gsub(/^  - /, ""); printf "%s, ", $0; next} found{exit}' "$skill_file" | sed 's/, $//')"
    pipeline="$(awk '/^pipeline:/{gsub(/^pipeline: */, ""); print; exit}' "$skill_file")"

    SKILL_MANIFEST="${SKILL_MANIFEST}  - ${name}: ${desc} [triggers: ${triggers}]"
    [ -n "$pipeline" ] && [ "$pipeline" != "[]" ] && SKILL_MANIFEST="${SKILL_MANIFEST} [pipeline: ${pipeline}]"
    SKILL_MANIFEST="${SKILL_MANIFEST}\n"
  done
fi

# Build memory summary (just the index, keep it compact)
MEMORY_SUMMARY=""
if [ -f "$MEMORY_DIR/index.md" ]; then
  # Extract the Learned Preferences section and Cross-Domain Notes
  MEMORY_SUMMARY="$(awk '/^## Learned Preferences/,/^## /{if(/^## / && !/^## Learned Preferences/)exit; print}' "$MEMORY_DIR/index.md" | grep -E '^- ' | head -10)"
fi

# Build workspace dirty status
WORKSPACE_DIRTY=""
if [ -d "$STEINBOT_HOME/.git" ]; then
  dirty_count="$(git -C "$STEINBOT_HOME" diff --name-only workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  untracked_count="$(git -C "$STEINBOT_HOME" ls-files --others --exclude-standard workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  total=$((dirty_count + untracked_count))
  if [ "$total" -gt 0 ]; then
    WORKSPACE_DIRTY="WARNING: $total unsaved workspace changes. Remind Justin to run 'stein sync'."
  fi
fi

# Compose the system context message
CONTEXT="[SteinBot Session Context — injected by hook]
Available skills (autodiscovered from ~/SteinBot/workspace/skills/):
$(echo -e "$SKILL_MANIFEST")
Memory preferences:
${MEMORY_SUMMARY:-  (no preferences recorded yet)}
${WORKSPACE_DIRTY:+
${WORKSPACE_DIRTY}}"

# Output as JSON for Claude Code to consume
jq -n --arg ctx "$CONTEXT" '{
  "systemMessage": $ctx
}'

exit 0
