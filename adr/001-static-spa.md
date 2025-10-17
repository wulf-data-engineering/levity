# ADR‑001 — Static SPA with Selective SSG for Web Deliverables

**Status**: Proposed  
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
| Language & tooling | TypeScript (executed via **Node.js/npm** runtime), Vite, ESLint, Prettier.                                                                     |
| QA tooling         | **vitest** for unit tests (in `src/`), **Playwright** for UI/E2E tests (in `tests/`).                                                            |

### Lambda\@Edge responsibilities

A minimal Node 18 **Lambda\@Edge** (or CloudFront Function) is associated with the default CloudFront behavior to:

* Rewrite unknown SPA routes to `/index.html` for client‑side routing fallback
* Perform canonical redirects (e.g. `example.com → www.example.com`, trailing‑slash normalization)
* Inject security headers (HSTS, CSP, X‑Frame‑Options) without rebuilding the artifact

**What we mean by *Selective SSG*.** Only routes whose HTML output is identical for every user (e.g. marketing pages, documentation, legal pages) are prerendered at build time. Any route that depends on session‑specific or rapidly changing data is served as a regular SPA route and hydrates on the client. Developers opt‑in or out per route via SvelteKit’s `export const prerender = true | false` flag.

**Why not SSR?**
SSG delivers the maximum benefit from AWS’s low‑cost static hosting model while retaining good SEO and first‑paint performance.

---

## 2. Intended Usage in Applications

Developers starting a new project MUST:

1. **Generate the project** with the root cookiecutter template (see §3).
   The template ships with `adapter-static` pre‑configured and a canonical `svelte.config.ts`.
2. **Mark prerenderable routes** in `+page.server.ts` with `export const prerender = true;` to opt into SSG.
   Omit (or set to `false`) on pages that require per‑session data.
3. **Keep all stateful API calls** in `src/lib/api/**` modules using the shared Protobuf/JSON client.
   This separation avoids accidental fetches during `svelte-kit prerender`.
4.  **Run `npm run preview:ssg`** locally; the task serves the `build/` folder via `vite preview`, showing exactly what CloudFront will serve.
5. \*\*Push to any branch → GitHub Action builds the static site and executes all tests but **does not deploy**.
   Push to **`main`** → Action builds, deploys the artifact to the **staging** S3/CloudFront stack, then runs smoke, UI, and E2E tests. If all checks pass, the *same artifact* is automatically promoted to **production** without rebuilding.

Rules of thumb:

| Situation                                     | Guidance                                                                                                          |
| --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Public marketing page, docs, landing          | **Prerender**. Improves TTFB and SEO.                                                                             |
| Auth‑gated dashboard, personalised feed       | **Hydrate only**. Mark route non‑prerendered.                                                                     |
| Variant where *some* blocks are user‑specific | Use opaque placeholder components that load data on mount; leave page prerendered to retain SEO for static parts. |

---

## 3. Tool-Set Support

The Cookiecutter template `templates/%[cookiecutter.project_slug]%` scaffolds the monorepo with the folder `frontend`
with the SvelteKit application prepared for SSG with unit using vitest and end-to-end tests using Playwright.

### Future enhancements (tracked separately):

* Incremental rebuilds via `svelte-kit sync` and CloudFront selective invalidation.
* Optional ISR (Incremental Static Regeneration) once supported natively in SvelteKit.

---

### Consequences

* Cold starts? None—the site is served as static files.
* Cost predictability: <\$1/month for S3 + CloudFront at prototype scale.
* Trade‑off: user‑specific content requires extra client fetches; server‑side rendering features (e.g., `load` with session data) are unavailable in prerendered routes.

### Alternatives considered

* **SSR on Lambda\@Edge** – increases complexity, erodes cost advantage, cold starts on every request.
* **Next.js / React** – heavier bundle size; ties to Node ecosystem instead of Svelte’s more minimal runtime.
