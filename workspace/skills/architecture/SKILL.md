---
name: architecture
description: System architecture analysis, design documents, and common patterns for APIs, databases, and distributed systems
triggers:
  - architecture
  - design
  - system design
  - database design
  - API design
  - refactor architecture
  - technical design
  - ADR
pipeline:
  - problem-modeling
  - contrastive-scoring
---

# Architecture Skill

Analyze, design, and document system architecture. Uses problem-modeling for domain understanding and contrastive-scoring to evaluate design options.

## Analyzing an Existing Codebase

1. **Entry points:** Find `main`, `index`, `app`, server startup files
2. **Directory structure:** `tree -L 2 -d` to understand module boundaries
3. **Dependencies:** Read `package.json`, `requirements.txt`, `go.mod`
4. **Data flow:** Trace entry point -> routes -> controllers -> services -> data access
5. **Configuration:** Find `.env`, config files, environment-specific settings
6. **Database schema:** Check migrations, models, or ORM definitions

Output a brief architecture map showing data flow between layers.

## Design Document Template (ADR-Style)

```markdown
# ADR: [Short Title]
**Date:** YYYY-MM-DD | **Status:** proposed | accepted | rejected | superseded

## Context
What is the problem? Why does a decision need to be made?

## Requirements
- Functional: What must the system do?
- Non-functional: Performance, scale, security targets
- Constraints: Budget, timeline, team skills, existing tech

## Options Considered
### Option A: [Name]
- Pros / Cons / Effort (S/M/L/XL)
### Option B: [Name]
- Pros / Cons / Effort

## Decision
Choosing [X] because [reasoning]. Use contrastive-scoring if options are close.

## Consequences
What becomes easier, what becomes harder, migration path, rollback plan.
```

## Common Architecture Patterns

| Pattern | Use When | Avoid When |
|---|---|---|
| **MVC/MVP** | CRUD apps, admin panels | Complex domain logic |
| **Hexagonal** | Business logic must be testable in isolation | Simple CRUD |
| **CQRS** | Read/write patterns differ significantly | Simple apps |
| **Event-Driven** | Loose coupling, async workflows | Simple synchronous flows |
| **Microservices** | Independent deploy cycles, different scaling needs | Small team, early-stage |
| **Monolith** | Small team, fast iteration, unclear boundaries | Genuinely different scaling needs |

**Default:** Start monolith. Extract services only when you can articulate WHY.

## Database Design

- **Normalize to 3NF** by default. Denormalize only with measured justification.
- **Every table:** `id` (PK), `created_at`, `updated_at`. UUIDs for public-facing IDs.
- **Indexes:** On every column in `WHERE`, `JOIN`, `ORDER BY` for tables > 10k rows.
- **Migrations:** Forward-only, idempotent, with rollback. Never modify deployed migrations.
- **Soft deletes:** `deleted_at` column for recoverable data.

## API Design (REST)

Standard CRUD: `GET /resources`, `GET /resources/:id`, `POST`, `PUT`, `PATCH`, `DELETE`.
**Pagination:** Cursor-based for large datasets, offset for small.
**Errors:** `{ "error": { "code": "VALIDATION_ERROR", "message": "...", "details": [...] } }`
**Versioning:** URL path (`/v1/resources`). Avoid header-based.

## When to Split vs Keep Simple

**Split when:** Different deploy cadence, different scaling needs, different team ownership, clean boundary.
**Keep together when:** Sharing a database, changes span multiple services, small team.

## Changelog
- 2026-02-06: Initial creation
