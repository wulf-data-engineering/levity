---
trigger: always_on
---

This is a monorepo.

- `frontend`: Static Svelte 5 page (TypeScript)
- `backend`: Lambda functions on AWS (Rust with cargo lambda)
- `infrastructure`: AWS CDK (TypeScript)
- `protocols`: API definitions (Protocol Buffers)
- `.github`: GitHub Actions for CI/CD
- `docker-compose.yml`: for local development
- `local`: volumes for local development (includes cognito-local config)

## Development

**CRITICAL**: If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning or editing in parallel. Only modify files after the user approves the plan, and you switch to execution mode.


Consult `.agent/rules/protocols.md` for protocols between frontend and backend.

Consult `.agent/rules/backend.md` for backend development & cloud resources.

Consult `.agent/rules/frontend.md` for frontend development.

Follow `.agent/workflows/try_browser.md` how to try the changes in the browser.

Follow `.agent/workflows/test_e2e.md` how to run the end-to-end tests.

During feature development check if deployment workflow needs modifications.
