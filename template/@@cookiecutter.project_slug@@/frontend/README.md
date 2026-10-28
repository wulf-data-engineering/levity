# @@cookiecutter.project_name@@ Frontend

This directory contains the frontend application using **SvelteKit** with **TypeScript**.
It is designed to be a self-contained static single-page application.

The application is rendered with **static-site generation (SSG)** via `@sveltejs/adapter-static`.

**Node.js** with **npm** is used as runtime.

**Vitest** is used for unit tests placed alongside the source code in `src/`.

**Playwright** is used for end-to-end and UI testing in `tests/e2e/`.

**shadcn/svelte** is used for UI components and styling installing its components in `src/lib/components/ui/`.

**Storybook** is used for a UI component library, see `.storybook`.

**ParaglideJS** (`@inlang/paraglide-js`) is used for SPA-compatible internationalization with message dictionaries in `messages/*.json` (use the `/translate-app` workflow in Antigravity to add new languages).

## Developing

- `npm install`: Installs the dependencies.
- `npm run dev`: Starts the development server (respects `FRONTEND_PORT`, `BACKEND_PORT`, and `COGNITO_LOCAL_PORT` from the root `.env` file).
- `npm run storybook`: Starts Storybook for UI component development.
- `npx shadcn-svelte@latest add ...`: Adds new components from shadcn/svelte.

> **Tip:** Need real-time WebSockets or client-side Rust WebAssembly? Ask Antigravity to apply the `.agent/skills/websockets-integration/SKILL.md` or `.agent/skills/wasm-integration/SKILL.md` skill.

## Building

- `npm run build`: Builds the application for production and injects the pre-hydration splash screen into `fallback.html`.
- `npm run preview`: Previews the production build locally.
- `npm run prepare`: Compiles ParaglideJS messages and synchronizes SvelteKit's generated types (usually run automatically by npm).
- `npm run format`: Formats all files with Prettier (also run automatically on staged files by `.githooks/pre-commit`).
- `npm run gen:proto`: Generates TypeScript code from Protocol Buffer definitions in `../protocols/`.

## Testing

- `npm run test`: Runs all unit and end-to-end tests once.
- `npm run test:unit`: Runs unit tests with Vitest.
- `npm run test:e2e`: Runs end-to-end tests with Playwright (requires `npx playwright install`).
- `npm run check`: Compiles ParaglideJS messages and runs `svelte-check` to find type issues.
- `npm run lint`: Checks for linting errors with ESLint (enforced with `npm run check` by `.githooks/pre-push`).
