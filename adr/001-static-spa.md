# ADR‑001 — Static SPA with Selective SSG for Web Deliverables

**Status**: Superceded by [ADR‑010](./010-cloudfront-spa-routing-and-split-stacks.md)  
**Date**: 2025‑07‑11

---

## 1. Design & Technology Decision

We commit to building every web client as a **self‑contained Single‑Page Application (SPA) written in SvelteKit (Svelte + TypeScript)**.
Key characteristics:

| Aspect             | Choice                                                                                                                                           |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Rendering model    | **Static‑Site Generation (SSG)** for all routes that do not require per‑request personalization; **client‑side hydration** for dynamic behavior. |
| Routing            | SvelteKit’s file‑based router; browser history API.                                                                                              |
| Build output       | Pure static assets (HTML + JS + CSS + images) emitted by `@sveltejs/adapter-static`.                                                             |
| Deployment target  | Amazon S3 bucket + CloudFront (compression & caching); **optional Lambda\@Edge** for SPA fallback & redirects.                                   |
| Language & tooling | TypeScript (executed via **Node.js/npm** runtime), Vite, ESLint, Prettier.                                                                       |
| QA tooling         | **vitest** for unit tests (in `src/`), **Playwright** for UI/E2E tests (in `tests/`).                                                            |

A minimal **Lambda\@Edge** (or CloudFront Function) is associated with the default CloudFront behavior to rewrite path
`/foo` to `/foo.html`.

**What we mean by _Selective SSG_.** Only routes whose HTML output is identical for every user (e.g. marketing pages, documentation, legal pages) are prerendered at build time. Any route that depends on session‑specific or rapidly changing data is served as a regular SPA route and hydrates on the client. Developers opt‑in or out per route via SvelteKit’s `export const prerender = true | false` flag.

**Why not SSR?**
SSG delivers the maximum benefit from AWS’s low‑cost static hosting model while retaining good SEO and first‑paint performance.

---

## 3. Tool-Set Support

- `frontend/` folder with scaffolded SvelteKit app prepared for SSG with unit using vitest and end-to-end tests using Playwright.
- `infrastructure/` folder with scaffolded CDK app prepared to deploy the SvelteKit app to S3 + CloudFront.
- `agent/` folder with agent configuration to add features to the frontend.
- CI/CD pipeline to build and test the SvelteKit app.

---

### Consequences

- Cold starts? None—the site is served as static files.
- Cost predictability: <\$1/month for S3 + CloudFront at prototype scale.
- Trade‑off: user‑specific content requires extra client fetches; server‑side rendering features (e.g., `load` with session data) are unavailable in prerendered routes.

### Alternatives considered

- **SSR on Lambda\@Edge** – increases complexity, erodes cost advantage, cold starts on every request.
- **Next.js / React** – heavier bundle size; ties to Node ecosystem instead of Svelte’s more minimal runtime.
