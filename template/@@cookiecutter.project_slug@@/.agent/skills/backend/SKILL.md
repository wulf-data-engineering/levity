---
name: Backend Development
description: Develop and test features in the backend (lambdas, protocols, infrastructure)
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

**Important**: For passing resource names or configuration from the infrastructure to Lambdas, prefer AWS Systems Manager (SSM) Parameter Store over environment variables. Lambdas should look up these values at startup using `aws_config::get_ssm_parameter`.

To get an AWS SDK client, use the backend helper that takes care of aws & localstack configuration:

```rust
let config = backend::load_aws_config().await;
```

### Localization (I18n)

If you introduce new end-user texts or messages, you **MUST** add them directly to the base `backend/locales/en.yml` file.
Additionally, you **MUST** translate these new texts and add them to all other available language files in `backend/locales/` (e.g., `de.yml`) using your translation capabilities.

### Protocols

To use a protocol defined in `protocols/<name>.proto`:

Include it directly in a `protocols` module:
```rust
pub mod protocols {
    include!(concat!(env!("OUT_DIR"), "/<name>.rs"));
}
pub use protocols::*;
```
This pattern will make the Protocol Buffer types available in the current scope.

### Local Development vs Deployed AWS Environment

If code depends on local development or deployed AWS environment separate that behavior in two functions with same name.
Annotate the local version with `#[cfg(any(debug_assertions, test))]`.
Annotate the deployed version with `#[cfg(not(any(debug_assertions, test)))]`.

Register new lambdas in Cargo.toml.

**CRITICAL:** If you add new APIs, always check them out using `curl` against the `cargo-lambda-watch.sh` server endpoint:
```bash
curl -v -H 'Accept: application/json' http://localhost:${BACKEND_PORT:-9000}/lambda-url/{lambda}/
```
After verifying the API directly, redeploy to LocalStack once (`npm run cdklocal:deploy` in the `infrastructure/` folder) so that it's also routed and available through `/api/{lambda}` on the `npm run dev` frontend server.

Define new API lambdas in `infrastructure/lib/constructs/backend/api.ts`.

**Important:** `api.ts` is not deployed locally.
Just Lambdas exposed via API Gateway are defined in `api.ts`.
Message handlers, Lifecycle hooks and other lambdas are defined in `infrastructure/lib/backend.ts`.
If you need to define resources for a specific API Gateway lambda (e.g. Parameter Value, SQS queue), you need to define them in `infrastructure/lib/backend.ts` and pass them to `api.ts`.

Run `cargo nextest run` during development.

If applicable consult `.agent/workflows/run-locally.md` to test the changes in the browser.

### Testing

**CRITICAL**: You **MUST** write unit or integration tests for any newly added feature or module. Do not finish a task without providing corresponding test coverage.
If there are existing unit or integration tests, extend or update them.
**CRITICAL**: You **MUST** write the test for a new pure function together with the function itself.
**CRITICAL**: You are highly encouraged to add descriptive comments to every function or struct, or construct that has more than 3 lines of code.

## MCP Tools

The `aws-knowledge-mcp-server` MCP server is available.

- Use `aws___get_regional_availability` to check regional availability of services.
- Use `aws___search_documentation` to search/read AWS documentation (prefer over general web search).

The `context7` MCP server is available for Rust crate documentation.

- Use `resolve-library-id` and `query-docs` to find documentation and examples for Rust crates.

### Final Checks (CRITICAL)

If you modified `infrastructure/package.json`, run `npm install` in `infrastructure/` to update the lock file.
Run `npm run format` in `infrastructure/`.

At the end of development run `cargo format`, `cargo check` & `cargo clippy`.
