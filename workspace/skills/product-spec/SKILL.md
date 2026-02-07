---
name: product-spec
description: Transform a vague idea into structured requirements, user stories, and acceptance criteria before writing any code
triggers:
  - product spec
  - requirements
  - user stories
  - acceptance criteria
  - spec out
  - define requirements
  - what should we build
  - app idea
  - feature request
  - PRD
pipeline: [problem-modeling]
---

# Product Spec Skill

## When to Use
When Justin describes an app idea, feature, or vague request and you need to turn it into something buildable. Always run this BEFORE writing code for a new project or major feature.

## Process

### Phase 1: Extract the Core (ask these, combine into ONE message)
- What problem does this solve? Who is it for?
- What's the ONE thing it must do on day one? (MVP scope)
- Any hard constraints? (tech stack, hosting, timeline, budget, integrations)
- What does "done" look like?

DO NOT ask more than these 4 questions. Infer the rest from context and the answers.

### Phase 2: Generate the Spec Document
After getting answers, produce this structure. Save it to `SPEC.md` in the project root:

```markdown
# [Project Name] — Product Spec

## Problem Statement
One paragraph. What problem, for whom, why now.

## Goals
- Primary: [the ONE thing]
- Secondary: [nice-to-haves, clearly marked]

## Non-Goals
What this project explicitly does NOT do (prevents scope creep).

## User Personas
| Persona | Description | Primary Need |
|---------|-------------|-------------|

## User Stories
Format: As a [persona], I want [action] so that [outcome].
Priority: P0 (must-have), P1 (should-have), P2 (nice-to-have).

### P0 — Must Have
- [ ] US-001: As a [persona], I want [action] so that [outcome]
  - AC: [acceptance criteria — testable condition]

### P1 — Should Have
- [ ] US-XXX: ...

### P2 — Nice to Have
- [ ] US-XXX: ...

## Data Model
Entities, relationships, key fields. Use a simple table format:

| Entity | Fields | Relationships |
|--------|--------|--------------|

## API Contract (if applicable)
| Method | Endpoint | Request | Response | Auth |
|--------|----------|---------|----------|------|

## UI Screens (if applicable)
List each screen with:
- Purpose
- Key elements/components
- Navigation flow

## Tech Stack Decision
| Layer | Choice | Rationale |
|-------|--------|-----------|

## Open Questions
Anything unresolved that needs Justin's input before building.
```

### Phase 3: Review and Confirm
Present the spec to Justin. Ask: "Does this capture what you want? Any changes before I start building?"

Only proceed to architecture/coding after explicit approval.

## Rules
- Never start coding without at least a minimal spec
- Keep specs in the project repo so they persist across sessions
- P0 stories define the MVP — build those first, nothing else
- If a spec already exists in the project, read it first and build from it
- Update the spec as requirements evolve (mark changes with dates)

## Changelog
- 2026-02-06: Initial creation
