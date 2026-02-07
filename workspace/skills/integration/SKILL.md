---
name: integration
description: Review and combine outputs from parallel agents into a coherent, working codebase
triggers:
  - integrate
  - combine agent outputs
  - stitch together
  - merge agent work
  - review agent results
  - assemble components
  - wire up
pipeline: [code-review]
---

# Integration Skill

## When to Use
After `sortiarius agent parallel` completes and you have multiple agent outputs that need to be combined into one working project. Also useful when you've built frontend and backend separately and need to wire them together.

## Process

### Phase 1: Inventory
1. Run `sortiarius agent status` to see all completed agents
2. Run `sortiarius agent review` for each agent output
3. Create an inventory of what each agent produced:

| Agent | Domain | Produced | Status |
|-------|--------|----------|--------|
| agent-1 | backend | API routes, models, middleware | Complete |
| agent-2 | frontend | React components, pages | Complete |
| agent-3 | testing | Test files, fixtures | Complete |

### Phase 2: Conflict Detection
Check for conflicts between agent outputs:

1. **Shared interfaces** — Do the frontend and backend agree on API contract (request/response shapes, endpoints, status codes)?
2. **Naming collisions** — Same filenames, same function names, conflicting exports?
3. **Dependency conflicts** — Incompatible package versions, duplicate dependencies?
4. **Configuration overlap** — Multiple agents writing to same config files (package.json, tsconfig, etc.)?
5. **Import paths** — Do cross-module imports use correct relative paths?

### Phase 3: Integration Plan
Before making changes, list every integration point:

```markdown
## Integration Points
1. Frontend → Backend: [how they communicate, e.g., fetch to /api/*, WebSocket]
2. Backend → Database: [ORM setup, connection config, migrations]
3. Shared types: [where type definitions live, who imports them]
4. Environment config: [.env variables needed by both]
5. Build system: [how to build/start everything together]
```

### Phase 4: Wire It Up
Execute in this order:

1. **Set up shared types/interfaces first** — Create a `types/` or `shared/` directory if agents produced overlapping type definitions. Consolidate into one source of truth.

2. **Merge package dependencies** — Combine package.json files. Resolve version conflicts (pick the newer compatible version).

3. **Place files in correct structure** — Agent outputs may have flat structures. Reorganize into the project's directory convention.

4. **Wire API endpoints** — Ensure frontend API calls match backend routes exactly (method, path, body shape, auth headers).

5. **Wire environment config** — Create/update `.env.example` with all needed variables. Ensure both frontend and backend read from the same config.

6. **Wire build/start scripts** — Update package.json scripts so `npm run dev` starts everything needed (or create a docker-compose if multiple processes).

7. **Run the test suite** — Apply the run-and-fix skill to get tests passing.

8. **Run the app** — Start it up, verify the happy path works end-to-end.

### Phase 5: Verify
Checklist before declaring integration complete:

- [ ] All agent output files are placed in the project
- [ ] No duplicate or conflicting files
- [ ] `npm install` (or equivalent) succeeds
- [ ] Build passes
- [ ] Tests pass
- [ ] App starts without errors
- [ ] Core user flow works end-to-end
- [ ] No hardcoded values that should be in config
- [ ] No TODO/FIXME left by agents that need resolution

### Rules
- **Never blindly concatenate agent outputs** — always read and understand each piece first
- **The backend is the source of truth for the API contract** — if frontend and backend disagree, fix the frontend
- **Shared types prevent integration bugs** — extract them immediately, don't let each side define their own
- **Test the integration, not just the units** — a passing unit test suite doesn't mean the parts work together
- If integration reveals a design flaw, update PLAN.md and potentially re-spec that component rather than hacking around it

## Changelog
- 2026-02-06: Initial creation
