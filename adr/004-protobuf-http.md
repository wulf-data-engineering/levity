# ADR-004 — Protobuf‑over‑HTTP/1.1 as the Remote‑API Contract

**Status**: Superceded by [ADR‑012](./012-snappy-protobuf-and-breaking-checks.md)  
**Date**: 2025‑07‑11  
**Deciders**: Tool‑set maintainers

---

## 1. Design & Technology Decision

Client‑to‑service APIs should exchange messages encoded with Protocol Buffers (proto3) over plain HTTP/1.1. The tool set favors Protobuf because it yields compact payloads, strict typing, and painless forward‑compatibility. Other formats might make sense on a per‑endpoint basis when human readability, third‑party interoperability, or data‑warehouse integration outweighs binary efficiency.

| Aspect          | Choice                                                                                              |
| --------------- | --------------------------------------------------------------------------------------------------- |
| Wire format     | **`application/x‑protobuf`** preferred; `application/json` in local development                     |
| Transport       | HTTP/1.1 via Amazon API Gateway v2 (REST) for request/response                                      |
| Spec location   | `/protocols/*.proto` files versioned in monorepo                                                    |
| Code generation | **Prost** for Rust (backend), **ts‑proto** for TypeScript (frontend & Tauri)                        |
| Versioning      | Backward‑compatible field numbering; additive changes preferred; breaking changes bump major SemVer |

**Why not gRPC?**
AWS Lambda lacks native gRPC support and we avoid the extra complexity of sidecar proxies. HTTP/1.1 works with all edge services (CloudFront, API Gateway), browser Fetch, and mobile WebView.

---

## 2. Tool‑Set Support

- `/protocols` folder
- Script to generate code for frontend
- Cargo build script to generate code for backend
- Frontend library code to send/receive protobuf messages
- Backend library code to receive/respond protobuf messages
- JSON for local development
- CI runs `buf lint` and `buf breaking`

---

### Consequences

- **Pros**: compact messages, language‑neutral contracts, single schema source, easy JSON debugging path.
- **Cons**: no bidirectional streaming; binary payloads require custom tooling in Postman‑style debuggers.

### Alternatives Considered

- **gRPC over ALB** – would require containerizing services and maintaining sidecars; breaks Lambda‑first mindset.
- **Pure JSON/REST** – easier to sniff but larger payloads, looser contracts, and no forward‑compatibility guarantees.
