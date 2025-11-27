---
description: Develop and test features in the frontend
---

## Concepts

The stack uses Svelte 5.
Use `onclick`, `onfoo` as attribute instead of `on:click`, `on:foo` before.
Use Svelte 5 runes `$state`, `$derived` and `$effect`.
If Svelte problems are encountered consider the migration guide: https://svelte.dev/docs/svelte/v5-migration-guide

The stack uses static site generation.
Prefer pre-rendering where possible (`export const prerender = true;` in pages and layouts).
Do not use server load functions.
Avoid client load functions in pages that make sense for an unauthorized user.
Use placeholder content while loading data.

The stack uses https://shadcn-svelte.com/ components for UI.

The stack uses storybook to display styling & usage of shadcn/svelte and derived components.

## Workflow

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

Run `npm run test:unit` during development.

Consult `run_locally.md` to test the changes in the browser.

At the end of development run `npm run format`, `npm run lint`.
