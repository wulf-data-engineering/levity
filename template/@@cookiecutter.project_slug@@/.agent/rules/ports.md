---
trigger: glob
globs: "docker-compose*.yml, .env*, **/vite.config.*, **/playwright.config.*, **/*.sh"
description: Ensure all service ports are configurable via environment variables and .env defaults.
---

# Port Configurability

When adding or modifying services, containers, or dev/test tools that expose ports, never hardcode ports exclusively in config files (`docker-compose.yml`, `vite.config.ts`, etc.).

Always make ports configurable via environment variables and define defaults in:
- `.env.example`
- `.env` (check with `read_file` if it exists; it is gitignored)

## Why
Developers running multiple instances simultaneously (e.g., across feature branches or apps) rely on `.env` to override ports without conflicts.
