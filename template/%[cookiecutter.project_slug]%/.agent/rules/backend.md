---
trigger: always_on
description: Develop and test features in the backend
---

# Change Backend

## Concepts

The `backend/` folder contains AWS Lambda functions written in Rust.

There are two types of lambdas:

- API lambdas exposed to the frontend via API Gateway
- Event handlers or lifecycle hooks

The API lambdas receive and respond with Protocol Buffer messages.
Handling is implemented in `backend/shared/protocols.rs`.

## Development

Define newly required or change configuration of Cloud resources in
`infrastructure/lib/backend.ts` and its constructs in `infrastructure/lib/constructs/backend/`
For complex changes create new constructs.

Make modifications to the shared libraries in `backend/src/shared/`.
Register them in `backend/src/lib.rs`.

Add or change Rust lambdas in the `backend/src/` folder.
Use `lambda_http` for Lambdas exposed via API Gateway.
Use `lambda_runtime` for event handlers or lifecycle hooks.

To use a protocol defined in `protocols/<name>.proto`:
1.  Import the `protocol_macro`:
    ```rust
    use protocol_macro::protocols;
    ```
2.  Define a module for the protocol:
    ```rust
    #[protocols("<name>")]
    pub mod protocols {}
    ```
The macro will make the Protocol Buffer types in the module available with `use`.

Register new lambdas in Cargo.toml.

Define new API lambdas in `infrastructure/lib/constructs/backend/api.ts`.

Run `cargo test` during development.

If applicable consult `.agent/workflows/run-locally.md` to test the changes in the browser.

### Final Checks

If you modified `infrastructure/package.json`, run `npm install` in `infrastructure/` to update the lock file.

At the end of development run `cargo format`, `cargo check` & `cargo clippy`.