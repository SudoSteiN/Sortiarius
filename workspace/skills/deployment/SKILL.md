---
name: deployment
description: Package an app for deployment with Docker, environment config, health checks, and runnable documentation
triggers:
  - deploy
  - deployment
  - dockerize
  - docker compose
  - package for production
  - make it runnable
  - ship it
  - CI/CD
  - production ready
pipeline: []
---

# Deployment Skill

## When to Use
When an app is functionally complete and needs to be packaged so it can actually run — locally for demo, on a server, or in production.

## Process

### Phase 1: Assess What We Have
Before generating any deployment files, check:

1. What runtime? (Node, Python, Go, etc.)
2. Does it have a database? What kind?
3. Does it need background workers or just a web server?
4. Any external service dependencies? (Redis, S3, email, etc.)
5. Does it already have a Dockerfile or docker-compose?

### Phase 2: Environment Configuration
Create `.env.example` (never `.env` — that's gitignored and user-specific):

```bash
# .env.example — Copy to .env and fill in values
# App
NODE_ENV=development
PORT=3000
APP_URL=http://localhost:3000

# Database
DATABASE_URL=postgresql://user:pass@localhost:5432/dbname

# Auth (if applicable)
JWT_SECRET=change-me-in-production
SESSION_SECRET=change-me-in-production

# External services (if applicable)
# REDIS_URL=redis://localhost:6379
# AWS_ACCESS_KEY_ID=
# AWS_SECRET_ACCESS_KEY=
```

Rules:
- Every environment variable must be documented
- Secrets get placeholder values, not real ones
- Group by category with comments
- Include sensible defaults for local development

### Phase 3: Dockerfile
Generate a multi-stage Dockerfile optimized for the stack:

**Node.js pattern:**
```dockerfile
FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=3s CMD wget -qO- http://localhost:3000/health || exit 1
CMD ["node", "dist/index.js"]
```

**Python pattern:**
```dockerfile
FROM python:3.12-slim AS builder
WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY . .

FROM python:3.12-slim
WORKDIR /app
COPY --from=builder /usr/local/lib/python3.12/site-packages /usr/local/lib/python3.12/site-packages
COPY --from=builder /app .
EXPOSE 8000
HEALTHCHECK --interval=30s --timeout=3s CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1
CMD ["python", "-m", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

Rules:
- Always multi-stage (smaller images)
- Always include HEALTHCHECK
- Always use specific version tags (not `latest`)
- Always use alpine/slim base images
- Copy dependency manifest first (layer caching)
- Never copy `.env`, `node_modules`, `__pycache__`, `.git` — use `.dockerignore`

### Phase 4: Docker Compose
If the app has dependencies (database, cache, etc.):

```yaml
services:
  app:
    build: .
    ports:
      - "${PORT:-3000}:3000"
    env_file: .env
    depends_on:
      db:
        condition: service_healthy
    restart: unless-stopped

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: ${DB_NAME:-appdb}
      POSTGRES_USER: ${DB_USER:-appuser}
      POSTGRES_PASSWORD: ${DB_PASS:-apppass}
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${DB_USER:-appuser}"]
      interval: 5s
      timeout: 3s
      retries: 5

volumes:
  pgdata:
```

Rules:
- Always use health checks with `depends_on: condition: service_healthy`
- Always use named volumes for data persistence
- Always use `env_file` instead of inline environment
- Use variable substitution with defaults for flexibility

### Phase 5: .dockerignore
```
node_modules
.git
.env
*.md
.vscode
__pycache__
*.pyc
.pytest_cache
coverage
dist
```

### Phase 6: README Setup Section
Add a "Getting Started" section to README.md:

```markdown
## Getting Started

### Prerequisites
- Docker and Docker Compose

### Run Locally
\```bash
cp .env.example .env
# Edit .env with your values
docker compose up -d
# App is running at http://localhost:3000
\```

### Development (without Docker)
\```bash
npm install
npm run dev
\```

### Run Tests
\```bash
npm test
\```
```

### Phase 7: Health Check Endpoint
If the app doesn't already have one, add a `/health` endpoint:

- Returns `200 OK` with `{"status": "ok", "timestamp": "...", "version": "..."}`
- Checks database connectivity
- Checks any critical external dependencies
- Should respond in < 1 second

### Checklist Before Declaring "Deployable"
- [ ] `.env.example` exists with all variables documented
- [ ] `Dockerfile` builds successfully
- [ ] `docker compose up` starts all services
- [ ] Health check endpoint responds
- [ ] App works end-to-end in containerized mode
- [ ] `.dockerignore` excludes dev artifacts
- [ ] README has clear setup instructions
- [ ] No secrets committed to git
- [ ] Database migrations run on startup (if applicable)

## Changelog
- 2026-02-06: Initial creation
