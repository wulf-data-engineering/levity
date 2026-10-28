# ADR‑010 — CloudFront Function SPA Routing & Three‑Stack Infrastructure

**Status**: Proposed (Supercedes [ADR‑001](./001-static-spa.md))  
**Date**: 2026‑10‑28

---

## 1. Recap of ADR‑001

[ADR‑001](./001-static-spa.md) established that every web client is built as a self‑contained Single‑Page Application (SPA) in SvelteKit and TypeScript using `@sveltejs/adapter-static` with selective Static‑Site Generation (SSG), deployed to Amazon S3 + CloudFront, and tested with Vitest (unit) and Playwright (end‑to‑end).

---

## 2. Focused Changes

### 2.1 CloudFront Function & `/fallback.html` SPA Routing

Instead of using a Lambda\@Edge function to rewrite `/foo` to `/foo.html`, routing is handled at the edge by a lightweight **CloudFront Function** (`RewriteFunction` on `VIEWER_REQUEST`) paired with two distinct cache policies:

| Path Pattern | CloudFront Behavior | Cache Policy |
| ------------ | ------------------- | ------------ |
| `/` or `/index.html` | Rewritten to `/index.html` | `ShortCachePolicy` (default 1 min TTL, Gzip & Brotli) |
| `/_app/immutable/*` | Passed through directly to S3 OAC origin | `ImmutableCachePolicy` (365‑day TTL, content‑hashed assets) |
| `/api/*` | Forwarded to API Gateway REST origin (`HTTPS_ONLY`) | `CACHING_DISABLED` (`ALL_VIEWER_EXCEPT_HOST_HEADER`) |
| Dynamic SPA routes (no file extension) | Rewritten to `/fallback.html` (with build‑time splash screen injection) for client‑side SvelteKit hydration | `ShortCachePolicy` |

Using a CloudFront Function eliminates Lambda\@Edge cold starts, regional replication delays, and global error‑page overrides that would otherwise intercept `/api/*` error responses.

### 2.2 Three‑Stack CDK Split & Deterministic Deployment Modes

To separate stateful DNS/identity foundations from frequent application deployments, the AWS CDK infrastructure is organized into three stacks (`infrastructure/lib/`):
1. **`FoundationStack`**: Persistent account resources — Route 53 Public Hosted Zone (with cross‑account `staging` NS delegation from production), SES Email Identity with DKIM records, and the GitHub Actions OIDC IAM Role (supporting both standard `repo:ORG/REPO:*` and immutable `repo:ORG@*/REPO@*:*` subject claims).
2. **`CertificateStack`**: Cross‑region ACM Certificate deployed in `us-east-1` for CloudFront custom domains.
3. **`AppStack`**: Application resources (S3, CloudFront, Cognito, API Gateway, DynamoDB, Lambdas) deployed on every release.

Configuration in `infrastructure/lib/config.ts` supports three explicit modes without runtime environment lookups:
- **`environment`** (`staging` | `production`): Full multi‑account deployment via GitHub Actions with custom domains and termination protection.
- **`sandbox`**: Domain‑less deployment of `AppStack` only to a personal AWS account using the default CloudFront domain and fast `sandbox` Rust build profile.
- **`local`**: LocalStack + `cognito-local` deployment for local development, where the frontend is served by Vite (`npm run dev`) with dynamic port configuration via `.env`.

### 2.3 Native Parallel E2E Testing

Playwright end‑to‑end tests reside in `frontend/tests/e2e/` and execute natively in parallel in CI against LocalStack, `cargo lambda watch`, and Vite without nested Docker‑in‑Docker browser containers.
