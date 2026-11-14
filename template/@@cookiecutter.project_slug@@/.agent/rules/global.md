---
trigger: always_on
---

This is a monorepo:

- `.agent`: Rules, hooks, and skills
- `frontend`: Static Svelte 5 SPA (TypeScript)
- `backend`: AWS Lambda functions (Rust with `cargo lambda`)
- `infrastructure`: AWS CDK (TypeScript)
- `protocols`: API definitions (Protocol Buffers)
- `.github`: GitHub Actions CI/CD
- `docker-compose.yml` & `local/`: Local development stack (LocalStack, `cognito-local`)

## Development

- **Tests (CRITICAL)**: You **MUST** write unit or integration tests for any newly added feature, component, or pure function, and update existing tests (including E2E tests) when behavior changes.
- **Comments**: Add descriptive comments to public/exported APIs, structs/types, CDK constructs, and non-obvious logic. Whenever you modify commented code, update the comment for correctness and completeness.
- **Planning**: If the user asks for a _plan_ (or `/plan`), do **not** modify any files until the user approves the plan.
- **Deployment**: During feature development, check if deployment workflows or CDK stacks need modifications.
- **Git Hooks**: Formatting is handled automatically by `pre-commit`, and linting/type-checking by `pre-push`. Do not run standalone formatters/linters manually before committing unless debugging a failure.

## Code Search & File Traversal (CRITICAL)

Never search, index, or read inside `target/`, `node_modules/`, `dist/`, or `.git/`. Scope `rg` (or `grep -rnI --exclude-dir=...`) directly to source directories (e.g., `backend/src/`, `frontend/src/`, `infrastructure/lib/`).

## Internationalization (I18N)

This application is polyglot. If you add or modify any end-user-facing text, add translations to all existing language files:
- Frontend: `frontend/messages/<lang>.json` (ParaglideJS)
- Backend: `backend/locales/<lang>.yml` (`rust-i18n`)

## AWS & MCP Usage

- **AWS Commands**: Never run `aws` or `cdk` without an explicit `--profile` (see `deployment.md`).
- **MCP**: Prefer configured MCP servers (e.g., `github`) over networked CLI tools like `gh` when MCP supports the action, as MCP tools run without requiring manual sandbox elevation.
