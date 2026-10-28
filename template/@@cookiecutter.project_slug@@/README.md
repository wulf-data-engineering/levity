# @@cookiecutter.project_name@@

@@cookiecutter.project_description@@

## Structure

The project is structured as follows:

- `frontend`: SvelteKit static single‑page application in TypeScript.
- `backend`: AWS Lambda functions written in Rust (`cargo-lambda`).
- `protocols`: Protocol Buffer API definitions (`.proto`).
- `infrastructure`: AWS CDK stacks (`FoundationStack`, `CertificateStack`, `AppStack`).
- `.agent`: Rules, core & opt‑in Skills, and Workflows for **Google Antigravity**.
- `.githooks`: Automatic `pre-commit` formatting and `pre-push` linting/type‑checking hooks.
- `local`: Volumes for local development with Docker Compose (`cognito-local`).
- `docker-compose.yml`: Services (`localstack` and `cognito-local`) for local development.

## Agentic Workflows & Opt‑In Skills

This repository includes a comprehensive rule and skill pack in `.agent/` for **Google Antigravity**:

- **Key Workflows**:
  - `/onboarding`: Checks local tool requirements, activates git hooks, and guides MCP server setup.
  - `/initial-setup` & `/configure-github-repo`: Guides AWS multi‑account setup (`staging` and `production`), DNS delegation, certificate deployment, and GitHub repository settings/branch protection.
  - `/run-locally` & `/test-e2e`: Starts the local development stack and runs Playwright end‑to‑end tests.
  - `/translate-app`: Translates frontend (`frontend/messages/*.json`) and backend (`backend/locales/*.yml`) dictionaries into a new language.
  - `/upgrade`: Batches, rebases, verifies, and fixes open Dependabot pull requests into a unified upgrade branch.
- **Opt‑In Feature Skills** (ask the agent to apply any of these when needed):
  - `websockets-integration` (`.agent/skills/websockets-integration/SKILL.md`): Adds real‑time API Gateway WebSockets, Cognito WebSocket authorizer, DynamoDB connection tracking, SQS background workers, and local `websocket-mock` emulation.
  - `wasm-integration` (`.agent/skills/wasm-integration/SKILL.md`): Adds client‑side Rust WebAssembly (`frontend-wasm/`) with `wasm-pack` hot‑reloading and CDK/CI bundling.
  - `antigravity-dependabot` (`.agent/skills/antigravity-dependabot/SKILL.md`): Adds autonomous CI workflows to assess passing Dependabot PRs for auto‑merge, auto‑fix failing Dependabot PRs, and sequentially rebase queued PRs.

## Development

### Getting Started

To get started, launch the onboarding workflow in Antigravity:

Just type `/onboarding` in the chat. The agent will guide you through tool verification, git hooks (`git config core.hooksPath .githooks`), and MCP setup.

### Port Configuration (`.env`)

All local service ports (`FRONTEND_PORT`, `BACKEND_PORT`, `LOCALSTACK_PORT`, `COGNITO_LOCAL_PORT`) are configurable so multiple instances or branches can run simultaneously without port collisions. Copy `.env.example` to `.env` if you want to customize ports before starting services.

### Docker Services

First, start the local Docker containers in one terminal:

```bash
docker compose up
```

It starts **localstack** and **cognito-local**.

### Backend

See [backend/README.md](backend/README.md) for requirements and details.
Start the backend in another terminal:

```bash
cd backend
./cargo-lambda-watch.sh
```

The Lambda functions are automatically rebuilt on code changes and respect the port configured in `.env`.

### Infrastructure

See [infrastructure/README.md](infrastructure/README.md) for requirements and details.
Deploy the infrastructure against **localstack** in another terminal:

```bash
cd infrastructure
npm run cdklocal:bootstrap # once (or after recreating localstack)
npm run cdklocal:deploy
```

Infrastructure is deployed against **localstack**.  
Cognito is not deployed to LocalStack because **cognito-local** is used for local development, pre‑seeded with an initial user pool, client, and test user _@@cookiecutter.test_user_email@@_ with password _@@cookiecutter.test_user_password@@_.  
Local Lambda invocations are forwarded to **`backend/cargo-lambda-watch.sh`**.

### Frontend

See [frontend/README.md](frontend/README.md) for requirements and details.
Start the frontend in another terminal:

```bash
cd frontend
npm run dev
```
