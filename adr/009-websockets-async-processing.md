# ADR‑009 — Real‑Time WebSockets & Asynchronous Background Processing

**Status**: Proposed  
**Date**: 2026‑10‑28

---

## 1. Context & Problem Statement

Many applications need to offload long‑running or compute‑intensive tasks (e.g., document processing, AI inference, batch operations) without blocking synchronous HTTP API calls or hitting API Gateway timeouts, while streaming progress updates and results back to the browser in real time.

Additionally, LocalStack Community Edition does not natively emulate AWS API Gateway v2 WebSocket routing and `@connections` callbacks against `cargo lambda watch`, requiring a first‑class local development story.

---

## 2. Design & Technology Decision

We provide real‑time bidirectional communication and asynchronous background processing as an opt‑in skill (`websockets-integration`):

| Aspect | Choice |
| ------ | ------ |
| **Transport** | **Amazon API Gateway v2 (WebSocket API)** with `$connect`, `$default`, and `$disconnect` routes. |
| **Authentication** | Custom Rust Lambda Authorizer (`cognito-authorizer`) on `$connect` extracting the Cognito JWT from the `Sec-WebSocket-Protocol` header. |
| **Connection Registry** | DynamoDB `VersionedTable` (`websocket_connections`) with partition key `userId`, sort key `topicId`, TTL `ttl`, and a `connection-index` Global Secondary Index on `connectionId` (`KEYS_ONLY`). |
| **Background Offloading** | API Lambdas validate requests and enqueue jobs onto **Amazon SQS**; dedicated `processor` worker Lambdas consume the queue and push status updates back via the **API Gateway Management API** (`/@connections/{connectionId}`). |
| **Infrastructure Placement** | Background worker Lambdas and queues are defined in `backend.ts` (outside the `deploymentConfig.aws` guard) so SQS event sources are deployed and triggered in LocalStack, while WebSocket API Gateway constructs reside in `api.ts`. |
| **Local Emulation** | Custom Rust/Axum `websocket-mock` container (`tools/websocket-mock`, published to `ghcr.io/wulf-data-engineering/levity-websocket-mock`) emulating `$connect`/`$default`/`$disconnect`, custom authorizer invocation, and `POST/DELETE /@connections/:id` against `cargo lambda watch`. |

**Why authenticate via `Sec-WebSocket-Protocol`?**
Browser `WebSocket` APIs do not support custom `Authorization` headers during the handshake, and passing JWTs in URL query parameters leaks sensitive tokens into access logs. Using `Sec-WebSocket-Protocol` keeps tokens out of URLs while remaining compatible with API Gateway Lambda Authorizers and standard browser clients.

---

## 3. Tool‑Set Support

- `template/@@cookiecutter.project_slug@@/.agent/skills/websockets-integration/`:
  - `SKILL.md`: Step‑by‑step instructions for wiring dependencies, CDK constructs, DynamoDB schema, and `.env` ports.
  - `assets/lib/`: Reusable Rust (`cognito-authorizer.rs`, `websocket.rs`, `shared/websockets.rs`) and TypeScript (`lib/websockets.ts`) libraries.
  - `assets/examples/`: End‑to‑end `process` / `processor` SQS + WebSocket push example with Protobuf contract.
- `tools/websocket-mock/` & `.github/workflows/publish-websocket-mock.yml`: Multi‑architecture (`linux/amd64`, `linux/arm64`) local WebSocket API Gateway emulator.

---

## 4. Consequences

- **Pros**: Non‑blocking asynchronous execution, per‑user/per‑topic multiplexed real‑time updates, clean stale‑connection cleanup via GSI, and full local hot‑reload parity with `cargo lambda watch`.
- **Cons**: Requires managing connection lifecycle (`GoneException` cleanup) and running the `websocket-mock` Docker container during local development.

### Alternatives Considered

- **AWS AppSync Events / GraphQL Subscriptions** – Introduces a second API paradigm alongside Protobuf‑over‑HTTP and complicates local offline emulation.
- **Server‑Sent Events (SSE) on Lambda Response Streaming** – Unidirectional only and does not decouple long‑running background workers from HTTP connection lifetimes.
