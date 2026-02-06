---
name: full-stack-dev
description: Full-stack development workflow covering project scaffolding, frontend/backend patterns, and deployment
triggers:
  - full stack
  - frontend
  - backend
  - react
  - next.js
  - node
  - python
  - fastapi
  - express
  - web app
  - build app
  - scaffold
  - create app
pipeline: []
---

# Full-Stack Development Skill

Practical workflows for scaffolding, building, and deploying full-stack applications.

## Project Scaffolding

**React + Node:** `mkdir -p myapp/{client,server,shared} && cd myapp && git init`, then `npx create-vite client --template react-ts` for frontend, `npm init -y && npm i express cors dotenv` in server/.

**Next.js:** `npx create-next-app@latest myapp --typescript --tailwind --eslint --app --src-dir`

**Python + FastAPI:** `mkdir -p myapp/{app/{api,models,services},tests,migrations}`, then venv + `pip install fastapi uvicorn sqlalchemy alembic pydantic-settings`

## File Structure Conventions

**Frontend (React/Next.js):** `src/components/` (shared UI), `src/features/` (feature modules with local components, hooks, api.ts, types.ts), `src/hooks/` (shared), `src/lib/` (utilities), `src/types/`.

**Backend (Node/Express):** `src/routes/` (thin), `src/controllers/` (request/response), `src/services/` (business logic), `src/repositories/` (data access), `src/middleware/`, `src/models/`, `src/config/`.

**Backend (Python/FastAPI):** `app/api/routes/`, `app/api/deps.py`, `app/models/`, `app/schemas/`, `app/services/`, `app/core/config.py`, `app/core/security.py`, `app/main.py`.

## Environment Setup

Commit `.env.example` (never `.env`). Use `docker-compose.yml` for local Postgres/Redis. Define `package.json` scripts: `dev`, `build`, `start`, `test`, `lint`, `db:migrate`, `db:seed`.

## Frontend Patterns

**State management:** `useState` (local) -> lift state / `useContext` (shared) -> `useReducer` (complex) -> Zustand/Redux Toolkit (global) -> TanStack Query (server state).

**Components:** Props interface at top, component function, styles at bottom. Export from `index.ts`.

## Backend Patterns

**Request flow:** `Route -> Middleware -> Controller -> Service -> Repository -> Database`

- **Controllers:** Parse request, call service, format response. No business logic.
- **Services:** All business logic. Plain objects in/out, no HTTP concepts.
- **Repositories:** Data access only. One per aggregate/entity.
- **Middleware:** Auth, validation, rate limiting, error handling.

**Error handling:** Catch `AppError` subclasses in middleware, return structured JSON. Log unexpected errors, return generic 500.

## Database Migration Workflow

1. Create migration: `npx knex migrate:make add_users_table` or `alembic revision --autogenerate -m "add users"`
2. Write up AND down migration
3. Test locally: migrate, verify, rollback, verify clean
4. Commit migration with the code that uses it -- never separately

## Deployment Checklist

- [ ] Env vars documented in `.env.example`
- [ ] `Dockerfile` builds and runs
- [ ] Health check endpoint (`GET /health`)
- [ ] Structured JSON logging
- [ ] Secrets in Key Vault / env, not code
- [ ] DB migrations automated or documented
- [ ] CORS restricted to production origins

## Changelog
- 2026-02-06: Initial creation
