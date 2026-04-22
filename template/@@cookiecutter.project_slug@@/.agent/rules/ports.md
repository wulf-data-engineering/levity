# Port Configurability

When developing features that introduce new ports (e.g. adding a new docker container, internal service, or testing tool), you **MUST** ensure the port configuration is scalable.

Never hardcode ports exclusively in configuration files (like `docker-compose.yml`, `vite.config.ts`, etc.).
Always make ports configurable via environment variables and define defaults in:
- `.env.example`
- `.env` (if it exists)

### Why?
When developers run multiple instances of the application simultaneously (e.g. working on two feature branches or multiple apps), they need to rely on the `.env` file to easily switch ports and avoid port occupancy conflicts.
