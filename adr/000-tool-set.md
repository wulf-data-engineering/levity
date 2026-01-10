# ADR‑000 — Structure & Test Strategy for the Tool‑Set Repository

**Status**: Proposed  
**Date**: 2025‑07‑11

---

## 1. Structure

The **tool‑set itself** (template its frontend, backend code, CDK constructs and docs) will be treated as a production‑grade software product.
We therefore adopt a multi‑layer test pyramid.

The repository contains

- cookiecutter template
- agent configuration (features for the template)
- pipeline configuration (CI/CD for the template)

The cookiecutter template is a monorepo containing

- frontend code
- backend code
- protocol definitions
- infrastructure as code
- agent configuration (features for the template instance)
- pipeline configuration (CI/CD for the template instance)
- docker compose configuration for local development

## 2. Test Strategy

The pipeline for a template instance runs

- linting: `cargo check`, `cargo clippy`, `cargo fmt -- --check`; eslint, prettier
- unit tests: `cargo nextest run`; vitest
- infrastructure tests: `aws-cdk-lib/assertions`, `cdk-nag`
- end-to-end tests: playwright tests

In the CI/CD pipeline for the template repository a template instance is generated and tested.

---

### Consequences

- **Pros**: early detection of breaking template changes, confidence that generated apps compile & test under current dependencies.
- **Cons**: template instance generation is heavy

### Alternatives Considered

- **In‑repo example app** instead of generating on the fly – simpler but doesn’t prove cookiecutter parameters.
