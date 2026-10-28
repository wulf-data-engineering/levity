# ADR‑011 — ARM64 Rust Lambdas, Build Profiles & Runtime SSM Parameters

**Status**: Proposed (Supercedes [ADR‑002](./002-rust-backend.md))  
**Date**: 2026‑10‑28

---

## 1. Recap of ADR‑002

[ADR‑002](./002-rust-backend.md) selected Rust AWS Lambda functions built with `cargo lambda`, least‑privilege per‑function IAM roles, and structured `tracing` logs as the default backend compute unit.

---

## 2. Focused Changes

### 2.1 Amazon Linux 2023 on ARM64

All deployed Rust Lambda functions target the **`PROVIDED_AL2023`** custom runtime on **`ARM_64`** (Graviton) architecture, cross‑compiled via `cargo lambda build --arm64`. In `local` mode (LocalStack), CDK deploys a lightweight Node.js proxy Lambda that forwards invocations over HTTP to `cargo lambda watch` on the host machine (`BACKEND_PORT`) for instant hot‑reloading.

### 2.2 Tiered Cargo Compilation Profiles

To balance local iteration speed, CI disk/cache footprint, sandbox deployment turnaround, and production cold‑start performance, `backend/Cargo.toml` defines three profiles:

| Profile | Configuration | Target Use Case |
| ------- | ------------- | --------------- |
| **`dev`** | `debug = "line-tables-only"`, `strip = "none"`, third‑party `[profile.dev.package."*"]` compiled with `opt-level = "s"` and `debug = false` | Local `cargo lambda watch`, unit tests (`cargo nextest run`), and compact CI caches |
| **`sandbox`** | Inherits `release` with `codegen-units = 16`, `lto = "none"`, `opt-level = 1`, `strip = "symbols"` | Fast personal AWS sandbox deployments (`-c mode=sandbox -c build=true`) and PR compile verification |
| **`release`** | `codegen-units = 1`, `lto = "fat"`, `strip = "debuginfo"` | Production and staging deployments in Continuous Deployment |

### 2.3 Runtime `SsmParameter` Resolution

Instead of passing infrastructure resource identifiers (DynamoDB table names, Cognito User Pool IDs, SQS queue URLs) as static Lambda environment variables, CDK stores resource identifiers in **AWS Systems Manager (SSM) Parameter Store** (`/<project_slug>/<parameter-name>`) and grants least‑privilege read access (`grantRead`).

Backend Lambdas resolve parameters at runtime through `SsmParameter` (`backend/src/shared/aws_config.rs`):
- **Deployed (`release` / `sandbox`) builds**: Values are fetched once on first access and cached in memory for the lifetime of the execution environment via `tokio::sync::OnceCell`.
- **Local (`debug` / `test`) builds**: Values are fetched fresh from LocalStack on every read (`#[cfg(any(debug_assertions, test))]`), allowing `cargo lambda watch` to survive `cdklocal` redeployments and LocalStack container recreations without restarting the Rust process.
