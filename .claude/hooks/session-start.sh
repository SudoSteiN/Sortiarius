#!/usr/bin/env bash
# Sortiarius Context Injection Hook: Load skill manifest + memory + agent status on session start
# Hook type: SessionStart
#
# Progressive disclosure: Only inject skill NAMES + triggers (not full content).
# Claude reads full SKILL.md on demand when a trigger matches.
# Also reports orphaned/running agents from the persistent registry.
set -uo pipefail

SORTIARIUS_HOME="${SORTIARIUS_HOME:-$HOME/Sortiarius}"
SKILLS_DIR="$SORTIARIUS_HOME/workspace/skills"
MEMORY_DIR="$SORTIARIUS_HOME/workspace/memory"
REGISTRY="$SORTIARIUS_HOME/workspace/scratch/agent-registry.json"

# --- Build skill manifest (compact: name + triggers only) ---
SKILL_MANIFEST=""
SKILL_COUNT=0
if [ -d "$SKILLS_DIR" ]; then
  for skill_dir in "$SKILLS_DIR"/*/; do
    [ -d "$skill_dir" ] || continue
    skill_file="$skill_dir/SKILL.md"
    [ -f "$skill_file" ] || continue

    name="$(awk '/^name:/{gsub(/^name: */, ""); print; exit}' "$skill_file")"
    triggers="$(awk '/^triggers:/{found=1; next} found && /^  - /{gsub(/^  - /, ""); printf "%s, ", $0; next} found{exit}' "$skill_file" | sed 's/, $//')"
    pipeline="$(awk '/^pipeline:/{gsub(/^pipeline: */, ""); print; exit}' "$skill_file")"

    # Progressive disclosure: only name + triggers, NOT full description
    SKILL_MANIFEST="${SKILL_MANIFEST}  - ${name} [${triggers}]"
    [ -n "$pipeline" ] && [ "$pipeline" != "[]" ] && SKILL_MANIFEST="${SKILL_MANIFEST} → ${pipeline}"
    SKILL_MANIFEST="${SKILL_MANIFEST}\n"
    SKILL_COUNT=$((SKILL_COUNT + 1))
  done
fi

# --- Build memory summary (just preferences, keep compact) ---
MEMORY_SUMMARY=""
if [ -f "$MEMORY_DIR/index.md" ]; then
  MEMORY_SUMMARY="$(awk '/^## Learned Preferences/,/^## /{if(/^## / && !/^## Learned Preferences/)exit; print}' "$MEMORY_DIR/index.md" | grep -E '^- ' | head -10)"
fi

# --- Check workspace dirty status ---
WORKSPACE_DIRTY=""
if [ -d "$SORTIARIUS_HOME/.git" ]; then
  dirty_count="$(git -C "$SORTIARIUS_HOME" diff --name-only workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  untracked_count="$(git -C "$SORTIARIUS_HOME" ls-files --others --exclude-standard workspace/ 2>/dev/null | wc -l | tr -d ' ')"
  total=$((dirty_count + untracked_count))
  if [ "$total" -gt 0 ]; then
    WORKSPACE_DIRTY="WARNING: $total unsaved workspace changes. Run 'sortiarius sync' when ready."
  fi
fi

# --- Check agent registry for running/orphaned agents ---
AGENT_STATUS=""
if [ -f "$REGISTRY" ] && command -v jq >/dev/null 2>&1; then
  running_agents="$(jq -r '.agents[] | select(.status == "running") | .id' "$REGISTRY" 2>/dev/null)"
  if [ -n "$running_agents" ]; then
    live_count=0
    orphan_count=0
    agent_lines=""
    while IFS= read -r agent_id; do
      [ -z "$agent_id" ] && continue
      pid="$(jq -r --arg id "$agent_id" '.agents[] | select(.id == $id) | .pid' "$REGISTRY" 2>/dev/null)"
      prompt="$(jq -r --arg id "$agent_id" '.agents[] | select(.id == $id) | .prompt' "$REGISTRY" 2>/dev/null | head -c 80)"
      if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        live_count=$((live_count + 1))
        agent_lines="${agent_lines}    [running] ${agent_id}: ${prompt}\n"
      else
        orphan_count=$((orphan_count + 1))
        # Mark orphaned agents as failed in registry
        jq --arg id "$agent_id" '(.agents[] | select(.id == $id)).status = "failed (orphaned)"' "$REGISTRY" > "${REGISTRY}.tmp" 2>/dev/null && mv "${REGISTRY}.tmp" "$REGISTRY"
        agent_lines="${agent_lines}    [orphaned] ${agent_id}: ${prompt}\n"
      fi
    done <<< "$running_agents"

    if [ $((live_count + orphan_count)) -gt 0 ]; then
      AGENT_STATUS="Background agents:
$(echo -e "$agent_lines")"
      [ "$orphan_count" -gt 0 ] && AGENT_STATUS="${AGENT_STATUS}  ($orphan_count orphaned — run 'sortiarius agent cleanup' to prune)"
    fi
  fi
fi

# --- Compose the system context message ---
CONTEXT="[Sortiarius Session Context]
Skills ($SKILL_COUNT available — read ~/Sortiarius/workspace/skills/<name>/SKILL.md for full instructions):
$(echo -e "$SKILL_MANIFEST")
Memory preferences:
${MEMORY_SUMMARY:-  (none recorded yet — run 'sortiarius memory' to add)}
${WORKSPACE_DIRTY:+
${WORKSPACE_DIRTY}}
${AGENT_STATUS:+
${AGENT_STATUS}}
Context management: Keep responses focused. For complex tasks, delegate to sub-agents via 'sortiarius agent'. Read skill files only when triggered, not preemptively."

# Output as JSON for Claude Code to consume
jq -n --arg ctx "$CONTEXT" '{
  "systemMessage": $ctx
}'

exit 0
