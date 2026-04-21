---
trigger: always_on
---

This is a monorepo.

- `.agent`: Your rules, skills and workflows.
- `frontend`: Static Svelte 5 page (TypeScript)
- `backend`: Lambda functions on AWS (Rust with cargo lambda)
- `infrastructure`: AWS CDK (TypeScript)
- `protocols`: API definitions (Protocol Buffers)
- `.github`: GitHub Actions for CI/CD
- `docker-compose.yml`: for local development
- `local`: volumes for local development (includes cognito-local config)

## Development

**CRITICAL**: You **MUST** write unit or integration tests for any newly added feature or component. Do not finish a task without providing corresponding test coverage.
If there are existing unit, integration, or end-to-end tests, extend or update them.
**CRITICAL**: You **MUST** write the test for a new pure function at the same time.

Add descriptive comments to every function, struct, class or construct that has more than 3 lines of code.
**CRITICAL:** If you change a function, struct, or construct that has a comment, you **MUST** update the comment as well. Check the comment for correctness and completeness and update it if necessary.

**CRITICAL**: If the user asks for a _plan_, **DO NOT** modify any files yet. Other agents might be planning or editing in parallel. Only modify files after the user approves the plan, and you switch to execution mode.

During feature development check if deployment workflow needs modifications.

**CRITICAL**: At the end of development run all final checks in the relevant skills (Frontend/Backend) before committing.
If the user asks for a commit, make sure your ran all the final checks on all changes first.

## Internationalization (I18N)

This application is built to be polyglot. 
**CRITICAL**: If you add or modify any end-user facing texts, you MUST add them to all existing language files.
- Frontend texts go into `frontend/messages/<lang>.json` using ParaglideJS format.
- Backend texts (like emails/notifications) go into `backend/locales/<lang>.yml` using rust-i18n format.

## MCP Tools

The `github` MCP server is available to assist with repository management.

- Use it to search issues, create comments, or manage pull requests if relevant to the task.