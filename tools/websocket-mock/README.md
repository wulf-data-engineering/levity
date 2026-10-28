# Local WebSocket API Gateway Mock (`websocket-mock`)

`websocket-mock` is a lightweight Rust/Axum server that emulates **Amazon API Gateway v2 WebSocket APIs** locally and bridges WebSocket events to **`cargo lambda watch`**.

It is used in generated projects when the opt‑in **WebSockets Integration** skill (`template/@@cookiecutter.project_slug@@/.agent/skills/websockets-integration/SKILL.md`) is applied (see [ADR‑009](../../adr/009-websockets-async-processing.md)).

## Features

1. **WebSocket Lifecycle Emulation (`ws://localhost:3001/:lambda`)**:
   - Invokes the `cognito-authorizer` Lambda on connection upgrade when a `Sec-WebSocket-Protocol` header is present, propagating the returned `principalId` and authorizer context.
   - Translates WebSocket lifecycle events into API Gateway v2 proxy payloads (`$connect`, `$default`, `$disconnect`) and forwards them via HTTP POST to `cargo lambda watch` (`http://$CARGO_LAMBDA_WATCH_TARGET/2015-03-31/functions/:lambda/invocations`).
2. **Management API Emulation (`http://localhost:3001/@connections/:id`)**:
   - `POST /@connections/:id`: Pushes raw payloads from backend/worker Lambdas (using `aws-sdk-apigatewaymanagement`) over the active WebSocket connection matching `:id` (returning `410 Gone` if disconnected).
   - `DELETE /@connections/:id`: Force‑closes an active connection.

## Usage in Docker Compose

The container image is published automatically to GitHub Container Registry (`ghcr.io/wulf-data-engineering/levity-websocket-mock:latest`) for both `linux/amd64` and `linux/arm64` via [`.github/workflows/publish-websocket-mock.yml`](../../.github/workflows/publish-websocket-mock.yml):

```yaml
  websocket-mock:
    image: ghcr.io/wulf-data-engineering/levity-websocket-mock:latest
    ports:
      - "${WEBSOCKET_MOCK_PORT:-3001}:3001"
    environment:
      - CARGO_LAMBDA_WATCH_TARGET=host.docker.internal:${BACKEND_PORT:-9000}
    extra_hosts:
      - "host.docker.internal:host-gateway"
```

## Building Locally

```bash
cd tools/websocket-mock
cargo run
```
