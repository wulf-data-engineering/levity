# Levity

[![Rust](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Fbackend%2Frust-toolchain.toml&query=%24.toolchain.channel&label=Rust&logo=rust&color=DEA584)](template/@@cookiecutter.project_slug@@/backend/rust-toolchain.toml)
[![Svelte](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2Fsvelte%27%5D.version&label=Svelte&logo=svelte&color=FF3E00)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![SvelteKit](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2F%40sveltejs%2Fkit%27%5D.version&label=SvelteKit&logo=svelte&color=FF3E00)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![TypeScript](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2Ftypescript%27%5D.version&label=TypeScript&logo=typescript&color=3178C6)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![Vite](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2Fvite%27%5D.version&label=Vite&logo=vite&color=646CFF)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![Tailwind CSS](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2Ftailwindcss%27%5D.version&label=Tailwind%20CSS&logo=tailwindcss&color=06B6D4)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![Playwright](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Ffrontend%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2F%40playwright%2Ftest%27%5D.version&label=Playwright&logo=playwright&color=2EAD33)](template/@@cookiecutter.project_slug@@/frontend/package.json)
[![AWS CDK](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fwulf-data-engineering%2Flevity%2Fmain%2Ftemplate%2F%40%40cookiecutter.project_slug%40%40%2Finfrastructure%2Fpackage-lock.json&query=%24.packages%5B%27node_modules%2Faws-cdk-lib%27%5D.version&label=AWS%20CDK&logo=amazonwebservices&color=FF9900)](template/@@cookiecutter.project_slug@@/infrastructure/package.json)

This repository is an **opinionated tool‑set** for building and operating

- **Static PWAs** with Svelte & TypeScript delivered from AWS edge (with optional **Rust WebAssembly** modules),
- **Serverless backends** in Rust on AWS (with optional **WebSockets & SQS background workers**) and
- **Tauri‑wrapped mobile apps** for iOS & Android.

It bundles a Cookiecutter template with IaC constructs, CI/CD pipelines, and first‑class support for agentic coding (**modular Agent Skills (`SKILL.md`), autonomous CI agents, meta‑workflows (`/derive-skill`), and MCP servers for Google Antigravity**) so teams can ship prototypes **fast** without sacrificing production readiness.

---

## Technology palette

| Layer          | Languages / runtimes                               | Core AWS / OSS tech                                                                                                                 | External accounts      |
| -------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| Web client     | Svelte + TypeScript (+ optional Rust WASM)         | Static‑site generation, S3 + CloudFront Function ([ADR‑001](/adr/001-static-spa.md), [ADR‑010](/adr/010-cloudfront-spa-routing-and-split-stacks.md)) | —                      |
| Compute        | **Rust**                                           | AWS Lambda ARM64 custom runtime via `cargo-lambda`, SSM config ([ADR‑002](/adr/002-rust-backend.md), [ADR‑011](/adr/011-arm64-rust-lambdas-and-ssm-config.md)) | —                      |
| Realtime / Job | Rust + TypeScript                                  | API Gateway WebSockets + SQS background workers, local `websocket-mock` ([ADR‑009](/adr/009-websockets-async-processing.md))        | —                      |
| Mobile apps    | Rust sidecar + Swift / Kotlin via **Tauri‑Mobile** | WebView shell, Fastlane lanes ([ADR‑003](/adr/003-mobile-tauri-wrapper.md))                                                         | Apple Dev, Google Play |
| API contract   | Protobuf (JSON fallback in dev)                    | Snappy‑compressed Protobuf over HTTP, `buf` lint & breaking guards ([ADR‑004](/adr/004-protobuf-http.md), [ADR‑012](/adr/012-snappy-protobuf-and-breaking-checks.md)) | —                      |
| Data store     | JSON documents                                     | Amazon DynamoDB versioned tables with composite keys & TTL ([ADR‑005](/adr/005-dynamodb.md), [ADR‑013](/adr/013-dynamodb-composite-keys-ttl-and-serde-flatten.md)) | —                      |
| Localization   | Message JSONs and YMLs                             | ParaglideJS (SPA) & `rust-i18n` ([ADR‑006](/adr/006-i18n.md))                                                                       | —                      |
| IaC            | TypeScript                                         | AWS CDK three‑stack split (`Foundation`, `Certificate`, `App`), multi‑account bootstrap ([ADR‑010](/adr/010-cloudfront-spa-routing-and-split-stacks.md)) | AWS                    |
| CI / CD        | GitHub Actions, Python                             | GitHub Actions, Dependabot, **Antigravity autonomous assessment, auto‑fix & rebase dispatcher** ([ADR‑007](/adr/007-antigravity-dependabot-automation.md)) | GitHub                 |
| Observability  | —                                                  | CloudWatch, X‑Ray, (future) dashboards & alerts                                                                                     | Slack (alert channel)  |
| AI assistance  | Markdown + parameterized assets                    | **Google Antigravity (`agy`)** — Rules, Skills, Workflows & MCP servers ([ADR‑008](/adr/008-modular-agent-skills.md))               | Google                 |

> **Note:** Each ADR file documents the full rationale, trade‑offs and tool‑set hooks for its layer (see also [ADR‑000](/adr/000-tool-set.md) for the repository structure and test strategy).

---

## Agentic coding & modular skills

Levity is designed around **Google Antigravity** and ships with a **comprehensive rule and skill pack for development and operations** ([ADR‑008](/adr/008-modular-agent-skills.md)):

- **Core domain skills**: Built‑in architectural guardrails and patterns guiding day‑to‑day development across the frontend, backend, database, and protocol layers, paired with pre‑configured MCP servers (`svelte`, `aws-knowledge-mcp-server`, `context7`, `github`).
- **Opt‑in feature skills**: Complex cross‑cutting capabilities are kept out of the base blueprint so new projects stay lean, and can be integrated on demand by the agent using self‑contained skills (`SKILL.md` + parameterized `assets/`):
  - **`websockets-integration`**: Real‑time bidirectional WebSockets (API Gateway WebSocket API + Cognito custom authorizer + DynamoDB connection registry), asynchronous SQS background processing, and local emulation via `tools/websocket-mock` ([ADR‑009](/adr/009-websockets-async-processing.md)).
  - **`wasm-integration`**: High‑performance client‑side Rust WebAssembly crates (`frontend-wasm/`) compiled with `wasm-pack`, hot‑reloaded alongside Vite, and bundled in CDK and CI/CD.
  - **`antigravity-dependabot`**: Autonomous CI workflows where headless Antigravity agents assess passing Dependabot PRs for domain safety and auto‑merge them, automatically repair failing Dependabot PRs from filtered CI logs, remediate open Dependabot and CodeQL security alerts via consolidated version‑upgrade PRs and per‑finding code PRs, sequentially rebase‑chain queued PRs, and track token costs ([ADR‑007](/adr/007-antigravity-dependabot-automation.md), [ADR‑014](/adr/014-antigravity-security-remediation.md)).
- **Skill derivation meta‑workflow (`/derive-skill`)**: Levity itself includes a meta‑workflow ([`.agent/workflows/derive-skill.md`](/.agent/workflows/derive-skill.md)) to extract complex features developed and tested in a template instance into new reusable, parameterized opt‑in skills.
- **Guided developer workflows**: Interactive workflows guide developers through environment verification and AWS/GitHub bootstrapping (`/onboarding`, `/initial-setup`) as well as one‑command polyglot localization across frontend and backend dictionaries (`/translate-app`), alongside operational workflows for local execution, E2E testing, and dependency upgrades.

---

## Non‑functional capabilities

- **Agentic coding** – Comprehensive rules, core & opt‑in `SKILL.md` packs, `/derive-skill` meta‑workflow, autonomous Dependabot CI agents, and MCP servers for **Antigravity**.
- **Rapid prototyping** – Cookiecutter templates, hot‑reload (`vite`, `cargo lambda watch`), `.env` dynamic port isolation for running multiple instances in parallel, and automatic `.githooks`.
- **Scalability & cost‑efficiency** – Serverless‑first architecture, edge‑cached SPA with CloudFront Functions, pay‑per‑invocation ARM64 compute.
- **Performance** – Rust ARM64 binaries yield <150 ms cold starts; tiered Cargo profiles (`dev`, `sandbox`, `release`); Snappy‑compressed Protobuf payloads; static assets served from CloudFront POPs.
- **Reliability** – Optimistic‑locking (`last_write`), staged CD (`main` → `staging` → `production`) with concurrency protection, source‑hash artifact caching, and smoke/UI/E2E gates.
- **Security** – Cognito auth (with SES sandbox pre‑signup guards), IAM least privilege, SSM Parameter Store & Secrets Manager, OIDC deployment roles supporting immutable GitHub subject claims.
- **Quality** – Unit (`cargo-nextest`, `vitest`), integration (LocalStack), UI & E2E (Playwright), protocol wire‑compatibility (`buf lint`, `buf breaking`), CDK synthesis (`cdk-nag`), and automated `pre-commit` formatting / `pre-push` linting git hooks.
- **Observability** – Structured `tracing`, CloudWatch metrics, (phase 2) dashboards & alert routing to Slack.
- **Maintainability** – Single language per layer, shared code libraries, Semantic Versioning, schema‑drift guard for DynamoDB (`schemars` + `expectorate`).

---

## Repository layout (high level)

```
/adr/                       - Architectural decision records (ADR-000 .. ADR-014)
/scripts/                   - Template verification utilities (check_defaults.py)
/tools/websocket-mock/      - Local WebSocket API Gateway emulator in Rust/Axum
/template/                  - Cookiecutter blueprint
/template/…/frontend/       - Static SvelteKit application in TypeScript
/template/…/backend/        - AWS Lambda functions in Rust (cargo-lambda)
/template/…/protocols/      - Protocol Buffer API contracts (.proto)
/template/…/infrastructure/ - AWS CDK constructs (Foundation, Certificate & App stacks)
/template/…/.githooks/      - Pre-commit formatting and pre-push linting hooks
/template/…/.agent/         - Rules, core & opt-in Skills, and Workflows for Antigravity
/template/…/.github/        - CI/CD pipelines, Rust cache warming, dependency management
/.agent/                    - Rules, Skills, and Workflows (/derive-skill, etc.) for template evolution
/.github/                   - CI/CD pipelines and autonomous Dependabot workflows for template itself
```

For a deeper dive, start with the ADRs linked above, then explore the matching template or tool directory.
