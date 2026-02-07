# Sortiarius Knowledge Library — Patterns

> Architectural patterns and design decisions that apply across projects.
> These are higher-level than solutions — patterns describe *approaches*, not specific implementations.

## Template for New Entries

```markdown
## [Pattern Name]
<!-- learned: YYYY-MM-DD -->
<!-- tags: api, architecture -->
<!-- reuse: HIGH | MEDIUM | LOW -->

**When to use:** Context where this pattern applies.
**Structure:** How the pattern is organized.
**Trade-offs:** What you gain and what you give up.
**Example projects:** Where this was used successfully.
```

---

## Project Structure — Full-Stack TypeScript App
<!-- learned: 2026-02-06 -->
<!-- tags: architecture, typescript, api, ui -->
<!-- reuse: HIGH -->

**When to use:** Any new web app with React frontend + Node.js backend.

**Structure:**
```
project/
├── src/
│   ├── server/          # Express/Fastify backend
│   │   ├── routes/      # API route handlers
│   │   ├── models/      # Database models/schemas
│   │   ├── middleware/   # Auth, validation, error handling
│   │   ├── services/    # Business logic
│   │   └── index.ts     # Server entry point
│   ├── client/          # React frontend
│   │   ├── components/  # UI components
│   │   ├── hooks/       # Custom React hooks
│   │   ├── pages/       # Route-level components
│   │   ├── services/    # API client functions
│   │   └── App.tsx      # Root component
│   └── shared/          # Shared types between client/server
│       └── types.ts     # API request/response types
├── tests/
│   ├── unit/
│   └── integration/
├── SPEC.md
├── PLAN.md
├── CRITERIA.md
└── Dockerfile
```

**Trade-offs:** Monorepo simplicity vs deploy flexibility. Good for MVPs. Split into separate repos when team grows.

---

## API Design — Resource-Based REST
<!-- learned: 2026-02-06 -->
<!-- tags: api, rest -->
<!-- reuse: HIGH -->

**When to use:** Any CRUD-based API.

**Structure:**
- `GET /resources` — List (with pagination: `?page=1&limit=20`)
- `GET /resources/:id` — Get one
- `POST /resources` — Create
- `PUT /resources/:id` — Full update
- `PATCH /resources/:id` — Partial update
- `DELETE /resources/:id` — Delete

**Conventions:**
- Consistent error format: `{ error: { code: "NOT_FOUND", message: "...", details: {} } }`
- 200 for success, 201 for created, 204 for deleted
- 400 for validation, 401 for unauthed, 403 for forbidden, 404 for not found
- Pagination: `{ data: [], meta: { page, limit, total, totalPages } }`

**Trade-offs:** REST is well-understood but verbose for complex queries. Switch to GraphQL if clients need flexible field selection.

---

## Error Handling — Layered Error Strategy
<!-- learned: 2026-02-06 -->
<!-- tags: architecture, api, security -->
<!-- reuse: HIGH -->

**When to use:** Any backend service.

**Structure:**
1. **Domain errors** — Custom error classes (`NotFoundError`, `ValidationError`, `AuthError`)
2. **Error middleware** — Catches all errors, maps to HTTP responses
3. **Client errors** — Structured error display components
4. **Logging** — Errors logged with context (request ID, user, stack trace)

**Rules:**
- Never expose stack traces to clients in production
- Always include a request ID for debugging
- Log the full error server-side, return sanitized version to client
- Validation errors include field-level details

**Trade-offs:** More upfront setup, but debugging is 10x easier. Worth it for any project beyond a prototype.
