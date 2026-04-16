---
description: Instructions and assets for adding WebSockets and background processing with SQS.
---

# Adding WebSockets and Background Processing

This skill adds WebSocket support to your application, allowing bidirectional communication between the frontend and the backend. It also demonstrates how to use SQS to offload background tasks to a secondary `processor` lambda, which can then asynchronously push results back to the frontend via WebSockets.

## Dependencies

Make sure the required dependencies are added.

### Backend Dependencies

Add the following to `backend/Cargo.toml` under `[dependencies]`:
```toml
uuid = { version = "1.10", features = ["v4"] }
aws-sdk-sqs = "1"
aws-sdk-apigatewaymanagement = "1"
rand = "0.8"
```

Use `cargo update` to get the latest versions of the dependencies.

Update the `aws_lambda_events` dependency to include `apigw` and `builders`:
```toml
aws_lambda_events = { version = "1", default-features = false, features = ["sqs", "cognito", "apigw", "builders"] }
```

Add the base WebSocket handling targets:
```toml
[[bin]]
name = "cognito-authorizer"
path = "src/cognito-authorizer.rs"

[[bin]]
name = "websocket"
path = "src/websocket.rs"
```

## Libraries

Make sure the libraries are in place.

Copy the files from `assets/lib/` into the corresponding folders:
- `assets/lib/backend/src/cognito-authorizer.rs` -> `backend/src/cognito-authorizer.rs`
- `assets/lib/backend/src/websocket.rs` -> `backend/src/websocket.rs`
- `assets/lib/backend/src/shared/websockets.rs` -> `backend/src/shared/websockets.rs`
- `assets/lib/frontend/src/lib/config.ts` -> `frontend/src/lib/config.ts`
- `assets/lib/frontend/src/lib/websockets.ts` -> `frontend/src/lib/websockets.ts`

Update `backend/src/shared/mod.rs` to declare the module:
```rust
pub mod websockets;
```

Update `backend/src/lib.rs` to export the websockets module:
```rust
pub use shared::websockets::{self, *};
```

## Local Development tools

In `docker-compose.yml`, add the `websocket-mock` service so that local endpoints receive WebSocket handshakes:

```yaml
  websocket-mock:
    image: ghcr.io/wulf-data-engineering/levity-websocket-mock:latest
    container_name: %[ cookiecutter.project_slug ]%-websocket-mock
    ports:
      - "3001:3001"
    extra_hosts:
      - "host.docker.internal:host-gateway"
```

**Note:** The websocket-mock is NOT defined in this project.

## Architecture Change

Make sure the architecture is ready.

### 1. CDK Construct: Backend
In `infrastructure/lib/constructs/backend.ts`:

- Create the `websocketConnectionsTable` with a partition key of `userId` and a sort key of `topicId` (using the `VersionedTable` construct that enables TTL and billing modes out-of-the-box):
```typescript
    const websocketConnectionsTable = new VersionedTable(this, 'WebsocketConnectionsTable', {
      tableName: 'websocket_connections',
      partitionKey: 'userId',
      sortKey: 'topicId',
      timeToLiveAttribute: 'ttl',
      removalPolicy: deploymentConfig.removalPolicy,
    });
```

- Add `connection-index` to the `websocketConnectionsTable` so `$disconnect` handlers can actively clean up sessions:
```typescript
websocketConnectionsTable.addGlobalSecondaryIndex({
    indexName: 'connection-index',
    partitionKey: { name: 'connectionId', type: AttributeType.STRING },
    projectionType: ProjectionType.KEYS_ONLY,
});
```

- Create an SSM parameter for the websocket connections table name so the backend lambdas can look it up during execution:
```typescript
    const websocketConnectionsTableParam = new ssm.StringParameter(this, "WebsocketConnectionsTableParam", {
      parameterName: "/app/websocket-connections-table-name",
      stringValue: websocketConnectionsTable.tableName,
    });
```

- Expose `webSocketUrl` from the Backend class and update `api` props appropriately. 
- Ensure you pass `websocketConnectionsTable` to the API stack.
- **CRITICAL**: Ensure you grant the `websocketConnectionsTable` read/write access AND the `websocketConnectionsTableParam` read access to any relevant Lambda functions (like the `websocket` lambda from API or the `processorFunction` from the example).

If you are using the processor example, also instantiate `processQueue` and its SSM queue URL parameter, and grant the `processorFunction` read access to that parameter.

### 2. CDK Construct: API
In `infrastructure/lib/constructs/backend/api.ts`:
- Define the `WebSocketApi` (from `@aws-cdk/aws-apigatewayv2-alpha`).
- Create a `backendLambda` for `cognito-authorizer` and `websocket`.
- Set up a `$connect`, `$disconnect`, and `$default` route integrated with the `websocket` lambda.
- Bind the authorizer to the `$connect` route.
- Make sure to grant the websocket lambda permissions to `manageConnections` on the API, and `readWriteData` on the `websocketConnectionsTable`.

### 3. Frontend Integration

In `frontend/vite.config.ts`, update the development proxy section to handle WebSocket requests (`ws: true`) if you are testing locally against a websocket mock or actual local API gateway.

In `frontend/src/lib/auth.ts`, after user authentication, establish the WebSocket connection by calling `connectWebSocket()`. Make sure to pass the JWT token in `Sec-WebSocket-Protocol` header.

## Examples

To fully test the WebSocket implementation, you can use the `process` example, which demonstrates:
1. An endpoint (`process.rs`) validating a request and posting the job to SQS.
2. A worker (`processor.rs`) pulling from SQS, doing work, and utilizing the WebSocket connections table to push results directly to the user.
3. The frontend setup (`api/process.ts` and `process.proto`).

Copy the files from `assets/examples/` to use them. Make sure to define the route and SQS queue appropriately in `infrastructure`. Also add the following config to your `backend/Cargo.toml`:

```toml
[[bin]]
name = "process"
path = "src/process.rs"

[[bin]]
name = "processor"
path = "src/processor.rs"
```

In `backend/locales/en.yml`, add the new text used by the processor:
```yaml
websockets:
  processor:
    processing: "Processing %{length} %{step}/%{total}..."
```

Make sure to translate the new text to all other available language files in `backend/locales/` (e.g., `de.yml`) using your translation capabilities.