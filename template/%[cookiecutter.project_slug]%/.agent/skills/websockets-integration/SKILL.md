---
name: WebSockets Integration
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

### Infrastructure Dependencies

Add the following to `infrastructure/package.json`:
```json
"@aws-cdk/aws-apigatewayv2-alpha": "^2.133.0-alpha.0",
"@aws-cdk/aws-apigatewayv2-authorizers-alpha": "^2.133.0-alpha.0",
"@aws-cdk/aws-apigatewayv2-integrations-alpha": "^2.133.0-alpha.0",
```

## Libraries

Make sure the libraries are in place.

Copy the files from `assets/lib/` into the corresponding folders:
- `assets/lib/backend/src/cognito-authorizer.rs` -> `backend/src/cognito-authorizer.rs`
- `assets/lib/backend/src/websocket.rs` -> `backend/src/websocket.rs`
- `assets/lib/backend/src/shared/websockets.rs` -> `backend/src/shared/websockets.rs`
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

Add the `VersionedTable` import:
```typescript
import { VersionedTable } from "./dynamodb";
```

Create the `websocketConnectionsTable` inside the class:
```typescript
    const websocketConnectionsTable = new VersionedTable(this, 'WebsocketConnectionsTable', {
      tableName: 'websocket_connections',
      partitionKey: 'userId',
      sortKey: 'topicId',
      timeToLiveAttribute: 'ttl',
      removalPolicy: deploymentConfig.removalPolicy,
    });

    websocketConnectionsTable.addGlobalSecondaryIndex({
        indexName: 'connection-index',
        partitionKey: { name: 'connectionId', type: dynamodb.AttributeType.STRING },
        projectionType: dynamodb.ProjectionType.KEYS_ONLY,
    });

    const websocketConnectionsTableParam = new ssm.StringParameter(this, "WebsocketConnectionsTableParam", {
      parameterName: "/app/websocket-connections-table-name",
      stringValue: websocketConnectionsTable.tableName,
    });
```

Expose `webSocketUrl` from the Backend class and update `api` props appropriately. Pass `websocketConnectionsTable` to the API stack.

Ensure you grant the `websocketConnectionsTable` read/write access AND the `websocketConnectionsTableParam` read access to any relevant Lambda functions (like the `websocket` lambda from API or the `processorFunction` from the example).

If you deploy a background worker (e.g. `processorFunction` or `translatorLambda`) that needs to send messages back to the WebSocket, you MUST inject the `WEBSOCKET_API_URL` environment variable:
```typescript
    workerLambda.addEnvironment('WEBSOCKET_API_URL', this.webSocketUrl!);
```

### 2. CDK Construct: API
In `infrastructure/lib/constructs/backend/api.ts`, add the API Gateway v2 imports:

```typescript
import * as apigwv2 from "@aws-cdk/aws-apigatewayv2-alpha";
import { WebSocketLambdaAuthorizer } from "@aws-cdk/aws-apigatewayv2-authorizers-alpha";
import { WebSocketLambdaIntegration } from "@aws-cdk/aws-apigatewayv2-integrations-alpha";
import { backendLambda } from "./backend-lambda";
```

Define the `WebSocketApi`, set up the authorizer, and explicitly use `backendLambda` (NOT `backendLambdaApi`) for the routing:

```typescript
    // Websocket API
    const websocketApi = new apigwv2.WebSocketApi(this, "WebSocketApi");
    
    const cognitoAuthorizerFunction = backendLambda(this, "CognitoAuthorizerFunction", {
      deploymentConfig: props.deploymentConfig,
      binaryName: "cognito-authorizer",
    });
    props.userPoolParam.grantRead(cognitoAuthorizerFunction);

    const websocketAuthorizer = new WebSocketLambdaAuthorizer("WebsocketAuthorizer", cognitoAuthorizerFunction, {
      identitySource: ["route.request.header.Sec-WebSocket-Protocol"],
    });

    const websocketFunction = backendLambda(this, "WebsocketFunction", {
      deploymentConfig: props.deploymentConfig,
      binaryName: "websocket",
    });
    props.websocketConnectionsTableParam.grantRead(websocketFunction);
    props.websocketConnectionsTable.grantReadWriteData(websocketFunction);

    websocketApi.addRoute("$connect", {
      integration: new WebSocketLambdaIntegration("ConnectIntegration", websocketFunction),
      authorizer: websocketAuthorizer,
    });

    const websocketIntegration = new WebSocketLambdaIntegration("WebsocketIntegration", websocketFunction);
    websocketApi.addRoute("$disconnect", { integration: websocketIntegration });
    websocketApi.addRoute("$default", { integration: websocketIntegration });

    websocketApi.grantManageConnections(websocketFunction);
```

### 3. Frontend Integration

In `frontend/src/lib/config.ts`, add `webSocketUrl` to the Config type:

```typescript
    export type Config = {
        userPoolId: string;
        userPoolClientId: string;
        endpoint?: string;
        webSocketUrl?: string; /* NEW */
    };
```

And initialize it statically for local development at the bottom of the config setup:

```typescript
        if (dev) {
            cachedConfig = {
                userPoolId: import.meta.env.VITE_USER_POOL_ID || 'local_userPool',
                userPoolClientId: import.meta.env.VITE_USER_POOL_CLIENT_ID || 'local_userPoolClient',
                endpoint: import.meta.env.VITE_COGNITO_ENDPOINT || 'http://localhost:9229',
                webSocketUrl: import.meta.env.VITE_WEBSOCKET_URL || 'ws://localhost:3001/' /* NEW */
            };
        }
```

In `frontend/src/lib/auth.ts` or whenever user authentication succeeds, establish the WebSocket connection by calling `connectWebSocket(topicId)` from `$lib/websockets.ts`. The implementation inside the asset automatically parses the JWT and applies it strictly via `Sec-WebSocket-Protocol`.


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