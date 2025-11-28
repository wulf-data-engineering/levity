This repository is an **opinionated tool‑set** for building and operating

* **Static PWAs** with Svelte & TypeScript delivered from AWS edge,
* **Serverless backends** in Rust on AWS and
* **Tauri‑wrapped mobile apps** for iOSs & Android.

It bundles a template with IaC constructs, CI pipelines and support for agentic coding so teams can ship prototypes **fast**
without sacrificing production readiness.

---

## 🤖 AI Agent Instructions
If you are an AI agent assisting with this repository, you **MUST** read `CONTEXT.md` before planning & generating code.

## Technology palette

| Layer         | Languages / runtimes                               | Core AWS / OSS tech                                                          | External accounts      |
|---------------|----------------------------------------------------|------------------------------------------------------------------------------|------------------------|
| Web client    | Svelte + TypeScript                                | Static‑site generation, S3 + CloudFront ([ADR‑001](/adrs/001-static-spa.md)) | —                      |
| Compute       | **Rust 2021**                                      | AWS Lambda custom Rust runtime ([ADR‑002](/adrs/002-rust-backend.md))        | —                      |
| Mobile apps   | Rust sidecar + Swift / Kotlin via **Tauri‑Mobile** | WebView shell, Fastlane lanes ([ADR‑003](/adrs/003-mobile-tauri-wrapper.md)) | Apple Dev, Google Play |
| API contract  | Proto3 (JSON fallback)                             | HTTP 1.1 on API Gateway v2 ([ADR‑004](/adrs/004-protobuf-http.md))           | —                      |
| Data store    | JSON documents                                     | Amazon DynamoDB tables ([ADR‑005](/adrs/005-dynamodb.md))                    | —                      |
| IaC           | TypeScript                                         | AWS CDK v2, multi‑account Org bootstrap                                      | AWS                    |
| CI / CD       | —                                                  | GitHub Actions, Dependabot                                                   | GitHub                 |
| Observability | —                                                  | CloudWatch, X‑Ray, (future) dashboards & alerts                              | Slack (alert channel)  |
| AI assistance | —                                                  | Gemini CLI, Claude Code prompt packs                                         | Google / Anthropic     |

> **Note:** Each ADR file documents the full rationale, trade‑offs and tool‑set hooks for its layer.

---

## Non‑functional capabilities

* **Rapid prototyping** – cookiecutter templates, hot‑reload (`vite`, `cargo lambda watch`).
* **Scalability & cost‑efficiency** – serverless‑first architecture, edge‑cached SPA, pay‑per‑invocation compute.
* **Performance** – Rust binaries yield <150 ms cold starts; static assets served from CloudFront POPs.
* **Reliability** – optimistic‑locking (`last_write`), staged CD (main → staging → prod) with smoke, UI & E2E gates.
* **Security** – Cognito auth, IAM least privilege, Secrets Manager, dependency scanners (`cargo‑audit`, `npm audit`,
  `trivy`).
* **Quality** – unit, integration (LocalStack), UI & E2E (Playwright), load (k6), a11y (axe‑playwright), licence &
  advisory checks (`cargo‑deny`).
* **Observability** – structured `tracing`, CloudWatch metrics, (phase 2) dashboards & alert routing to Slack.
* **Maintainability** – single language per layer, shared code libraries, Semantic Versioning, schema‑drift guard for
  DynamoDB.

---

## Repository layout (high level)

```
/adr/                     Architectural decision records
/template/                Cookiecutter blueprint
/template/frontend/       - Static Svelte application in TypeScript
/template/backend/        - Lambdas in Rust
/template/infrastructure/ - CDK constructs (static SPA stack, RustFunction, VersionedTable, …) 
/template/.github/        - CI/CD pipelines 
/docs/                    How‑to guides, style guides, troubleshooting
```

For a deeper dive, start with the ADRs linked above, then explore the matching template or library directory.
