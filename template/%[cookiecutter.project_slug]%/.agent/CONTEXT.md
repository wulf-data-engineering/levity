# Repository Context

This is a monorepo.

- `frontend`: Static Svelte 5 page (TypeScript)
- `backend`: Lambda functions on AWS (Rust with cargo lambda)
- `infrastructure`: AWS CDK (TypeScript)
- `protocols`: API definitions (Protocol Buffers)
- `.github`: GitHub Actions for CI/CD
- `docker-compose.yml`: for local development
- `local`: volumes for local development (includes cognito-local config)

## Development Workflow

**CRITICAL**: If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning in parallel. Only modify files after the user approves the plan and you switch to execution mode.

1. Consult `workflow/change_protocols.md` how to add/change the protocols.
2. Consult `workflow/change_backend.md` how to make changes to the backend & cloud resources.
3. Consult `workflow/change_frontend.md` how to make changes to the frontend.
4. Consult `workflow/try_browser.md` how to try the changes in the browser.
5. Consult `workflow/test_e2e.md` how to run the end-to-end tests
6. Check if deployment workflow needs modifications.
