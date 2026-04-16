---
name: Frontend Development
description: Develop and test features in the frontend
---

## Concepts

The stack uses Svelte 5.

The stack uses static site generation.
Prefer pre-rendering where possible (`export const prerender = true;` in pages and layouts).
Do not use server load functions.
Avoid client load functions in pages that make sense for an unauthorized user.
Use placeholder content while loading data.

The stack uses https://shadcn-svelte.com/ components for UI.

The stack uses storybook to display styling & usage of shadcn/svelte and derived components.

## Development

Add or change pages and layouts in `frontend/src/routes/`.

Add or change shared code in `frontend/src/lib/`.

If new UI is introduced, check if existing shadcn/svelte components can be used:
https://shadcn-svelte.com/docs/components
If new shadcn/svelte components are needed, run `npx shadcn-svelte add <component-name>`.
If needed, make adjustments to the generated components in `frontend/src/lib/components/ui/<component-name>`.
Add a story in `frontend/src/lib/components/ui/<component-name>/<component-name>.stories.svelte` to describe
the component usage and styling in the project.

Add or change derived components in `frontend/src/lib/components/`.
Follow the structure of existing components:

- index.ts to export the component
- component.svelte for the Svelte component
- Component.stories.svelte for Storybook stories
- Component.test.svelte for an optional test component
- component.test.ts for unit tests using the Svelte testing-library

If code depends on local development or deployed AWS environment decide based on the `dev` value:

```typescript
import { dev } from "$app/environment";
```

Run `npm run test:unit` during development.

Consult @../workflows/run-locally.md to test the changes in the browser.
to test the changes in the browser.

### Localization (I18n)

If you introduce new end-user texts or messages, you **MUST** add them directly to the base `frontend/messages/en.json` file.
Additionally, you **MUST** translate these new texts and add them to all other available language files in `frontend/messages/` (e.g., `de.json`) using your translation capabilities.
Always use the `m.*` imports from `$lib/paraglide/messages.js` instead of hardcoded strings in the UI components.

### Testing

**CRITICAL**: You **MUST** write unit or integration tests for any newly added feature or component. Do not finish a task without providing corresponding test coverage.
If there are existing unit or end-to-end tests, extend or update them.
**CRITICAL**: You **MUST** write the test for a new pure function together with the function itself.
**CRITICAL**: You are highly encouraged to add descriptive comments to every function, component, or UI element that has more than 3 lines of code.

#### Browser Agent Testing Guidelines

When testing this frontend using an automated browser agent (e.g., Anthropic's computer use or typical browser toolchains), you **MUST** follow these heuristics to avoid UI state collisions:

1. **Clear Prefilled Fields**: Some login forms (specifically Email and Password fields) may automatically load securely prefilled values locally. Directly sending type/fill commands will **append** text (e.g., `%[cookiecutter.test_user_email]%`), causing authentications to fail. You MUST completely clear these input fields utilizing your browser selection tools (e.g., highlighting and deleting, or `Ctrl+A`/`Cmd+A` -> `Backspace`) prior to sending new keystrokes.
2. **Handle OTP/Confirmation Inputs**: The confirmation code input UI uses complicated slots that often cause issues with automated typing. Therefore, you should **keep the currently prefilled value** in the OTP boxes unconditionally. Do not attempt to retype or override the OTP input during browser flows.

### MCP Tools

The `svelte` MCP server is available to assist with Svelte 5 and SvelteKit development.

- Use `list-sections` and `get-documentation` to search for official documentation.
- Use `svelte-autofixer` to fix issues in Svelte components.

The `context7` MCP server is available for general frontend library documentation.

- Use `query-docs` to find documentation for TypeScript libraries (e.g., specific shadcn/svelte details not covered by the svelte server, or other util libraries).

## Browser Agent Testing Guidelines

When testing this frontend using an automated browser agent, you **MUST** follow these heuristics to avoid UI state collisions:

1. **Clear Prefilled Fields**: Some login forms (specifically Email and Password fields) may automatically load securely prefilled values locally. Directly sending type/fill commands will **append** text (e.g., `@@cookiecutter.test_user_email@@`), causing authentications to fail. You MUST completely clear these input fields utilizing your browser selection tools (e.g., highlighting and deleting, or `Ctrl+A`/`Cmd+A` -> `Backspace`) prior to sending new keystrokes.
2. **Handle OTP/Confirmation Inputs**: The confirmation code input UI uses complicated slots that often cause issues with automated typing. Therefore, you should **keep the currently prefilled value** in the OTP boxes unconditionally. Do not attempt to retype or override the OTP input during browser flows.

## Final Checks (CRITICAL)

If you modified `frontend/package.json`, run `npm install` in `frontend/` to update the lock file.

At the end of development run `npm run format`, `npm run lint`, `npm run check`.
