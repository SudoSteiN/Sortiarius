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

# --- Check for project context (SPEC.md / PLAN.md in CWD) ---
PROJECT_CONTEXT=""
CWD="${CLAUDE_CWD:-$(pwd)}"

# Walk up from CWD to find project root
PROJECT_ROOT=""
CHECK_DIR="$CWD"
while [ "$CHECK_DIR" != "/" ] && [ "$CHECK_DIR" != "." ]; do
  if [ -f "$CHECK_DIR/package.json" ] || [ -f "$CHECK_DIR/Cargo.toml" ] || \
     [ -f "$CHECK_DIR/pyproject.toml" ] || [ -f "$CHECK_DIR/go.mod" ] || \
     [ -f "$CHECK_DIR/requirements.txt" ] || [ -d "$CHECK_DIR/.git" ]; then
    PROJECT_ROOT="$CHECK_DIR"
    break
  fi
  CHECK_DIR="$(dirname "$CHECK_DIR")"
done

if [ -n "$PROJECT_ROOT" ] && [ "$PROJECT_ROOT" != "$SORTIARIUS_HOME" ]; then
  PROJECT_NAME="$(basename "$PROJECT_ROOT")"
  PROJECT_CONTEXT="Project: $PROJECT_NAME ($PROJECT_ROOT)"

  # Check SPEC.md
  if [ -f "$PROJECT_ROOT/SPEC.md" ]; then
    SPEC_SUMMARY="$(awk '/^## Problem Statement/{found=1; next} found && /^$/{if(p)exit; next} found && /^##/{exit} found{p=1; print}' "$PROJECT_ROOT/SPEC.md" | head -2)"
    [ -n "$SPEC_SUMMARY" ] && PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  Spec: ${SPEC_SUMMARY}"
  else
    PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  WARNING: No SPEC.md — run product-spec skill before writing code"
  fi

  # Check PLAN.md and extract CURRENT task
  if [ -f "$PROJECT_ROOT/PLAN.md" ]; then
    CURRENT_TASK="$(grep -m1 'CURRENT' "$PROJECT_ROOT/PLAN.md" | sed 's/.*CURRENT[^]]*\]\s*//' | sed 's/\*//g' | head -c 120)"
    [ -n "$CURRENT_TASK" ] && PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  Current task: ${CURRENT_TASK}"

    # Check for stale PLAN.md (>7 days)
    if command -v stat >/dev/null 2>&1; then
      PLAN_MTIME="$(stat -c %Y "$PROJECT_ROOT/PLAN.md" 2>/dev/null || stat -f %m "$PROJECT_ROOT/PLAN.md" 2>/dev/null || echo "0")"
      NOW="$(date +%s)"
      PLAN_AGE_DAYS=$(( (NOW - PLAN_MTIME) / 86400 ))
      [ "$PLAN_AGE_DAYS" -gt 7 ] && PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  WARNING: PLAN.md is ${PLAN_AGE_DAYS} days stale — update it"
    fi

    # Count remaining tasks
    REMAINING="$(grep -c '^\s*- \[ \]' "$PROJECT_ROOT/PLAN.md" 2>/dev/null || echo "0")"
    COMPLETED="$(grep -c '^\s*- \[x\]' "$PROJECT_ROOT/PLAN.md" 2>/dev/null || echo "0")"
    PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  Progress: ${COMPLETED} done, ${REMAINING} remaining"
  else
    PROJECT_CONTEXT="${PROJECT_CONTEXT}\n  WARNING: No PLAN.md — consider creating one to track progress"
  fi
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
${PROJECT_CONTEXT:+
$(echo -e "$PROJECT_CONTEXT")}
Context management: Keep responses focused. For complex tasks, delegate to sub-agents via 'sortiarius agent'. Read skill files only when triggered, not preemptively."

# Output as JSON for Claude Code to consume
jq -n --arg ctx "$CONTEXT" '{
  "systemMessage": $ctx
}'

exit 0
