# Sortiarius — Product Spec

## Problem Statement
Claude Code is powerful but stateless between sessions — no persistent memory, no enforced workflow, no cross-project learning. Without structure, each session starts from zero and developers must manually enforce quality, safety, and process consistency.

## Goals
- Augment Claude Code with persistent memory, skills, and hooks
- Enforce development workflow (spec → plan → build → test → ship) via deterministic hooks
- Maintain a knowledge library of cross-project solutions for reuse
- Support multi-level agent hierarchy (super agent → project agents → sub-agents)
- Provide self-evaluation criteria so development loops know when to stop

## Non-Goals
- Replace Claude Code itself (Sortiarius is an augmentation layer, not a fork)
- Multi-platform messaging (Slack, Discord, etc.)
- Voice or mobile interfaces
- Cloud hosting or SaaS deployment

## Architecture
- **Hooks** (`.claude/hooks/`) — Deterministic enforcement, bash scripts triggered by Claude Code events
- **Skills** (`workspace/skills/`) — Domain-specific guidance with YAML frontmatter for autodiscovery
- **Memory** (`workspace/memory/`) — Persistent cross-session knowledge by domain
- **Knowledge Library** (`workspace/knowledge/`) — Cross-project solutions, patterns, components
- **Agent System** (`bin/sortiarius-agent`) — Persistent registry for parallel Claude instances
- **Web Dashboard** (`workspace/ui/`) — Local monitoring UI
- **CLI** (`bin/sortiarius`) — Command-line interface for all operations

## Tech Stack
- Bash (hooks, CLI, agent system)
- Python 3 stdlib (web dashboard — no external dependencies)
- Markdown + YAML frontmatter (skills, memory, knowledge)
- JSON (registries, settings, session logs)

## Done Criteria
- All hooks fire correctly on their registered events
- Skills are discoverable via frontmatter triggers
- Agent hierarchy supports 3 levels (super → project → sub-agent)
- Knowledge library is searchable and updated after project milestones
- Evaluation criteria can gate pipeline phases with pass/fail
- Web dashboard displays system state accurately
