# @@cookiecutter.project_name@@ Backend

This directory contains the serverless backend written in **Rust** and built with **`cargo-lambda`** targeting the AWS Lambda `provided.al2023` runtime on `arm64`.

## Architecture & Binaries

- **Binaries (`src/*.rs`)**:
  - `cognito-handler`: Handles Cognito user pool lifecycle triggers (custom localized verification/reset emails, post‑confirmation user creation, and SES sandbox domain guards).
  - `password-policy`: Returns password policy requirements derived from the Cognito User Pool.
  - `user-profile`: Authenticated API handler for reading and updating user profiles.
  - `message-handler`: Example event/queue message handler.
- **Shared Library (`src/shared/`)**:
  - `aws_config.rs`: AWS SDK client loaders (automatically targeting LocalStack / `cognito-local` in debug/test builds) and `SsmParameter` (cached in `OnceCell` in release builds, fetched fresh on every call in debug mode so `cargo lambda watch` survives LocalStack redeployments).
  - `dynamodb.rs` & `users.rs`: Generic `Versioned<T>` optimistic‑locking wrapper (`last_write`, `data_version`) and `UserRepo`.
  - `protocols.rs` & `http.rs`: Content‑negotiated Protocol Buffer (`application/x-protobuf` with Snappy compression) and JSON request/response helpers plus JWT claim extraction.
  - `i18n.rs`: Localization helpers backed by `rust-i18n` (`locales/*.yml`).

## Cargo Profiles

- **`dev`**: Uses `debug = "line-tables-only"` and optimizes third‑party dependencies for size (`opt-level = "s"`, `debug = false`) for fast local compilation and compact CI caches.
- **`sandbox`**: Fast release‑like profile (`codegen-units = 16`, `lto = "none"`, `opt-level = 1`, `strip = "symbols"`) used for rapid AWS sandbox deployments and PR compile checks.
- **`release`**: Full production optimization (`codegen-units = 1`, `lto = "fat"`, `strip = "debuginfo"`).

## Local Development & Testing

- `./cargo-lambda-watch.sh`: Starts `cargo lambda watch` respecting `BACKEND_PORT` (and local endpoint variables) from the root `.env` file.
- `cargo nextest run`: Runs unit and schema‑snapshot tests (`schemars` + `expectorate` against `schema/*.schema.json`).
- `cargo clippy -- -D warnings`: Runs linter checks (also enforced automatically by `.githooks/pre-push`).
- `cargo fmt`: Formats Rust code (also run automatically on staged files by `.githooks/pre-commit`).
