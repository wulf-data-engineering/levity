# ADR‑012 — Snappy‑Compressed Protobuf & Wire‑Compatibility Enforcement

**Status**: Proposed (Supercedes [ADR‑004](./004-protobuf-http.md))  
**Date**: 2026‑10‑28

---

## 1. Recap of ADR‑004

[ADR‑004](./004-protobuf-http.md) established Protocol Buffers (`proto3`) over HTTP with content negotiation (`application/x-protobuf` in deployed environments, `application/json` in local development) as the contract between frontend and backend, generated from `/protocols/*.proto` using `prost` (Rust) and `ts-proto` (TypeScript).

---

## 2. Focused Changes

### 2.1 Snappy Compression on Binary Payloads

When exchanging binary `application/x-protobuf` payloads in deployed environments, both requests and responses are compressed using **Snappy** (`snap` crate in `backend/src/shared/protocols.rs` and `snappyjs` in `frontend/src/lib/protocols.ts`). JSON fallback in local development remains uncompressed for straightforward inspection in browser DevTools and `curl`.

### 2.2 API Gateway Topology Split

- **Synchronous HTTP APIs**: Served via **Amazon API Gateway REST API** (`aws-apigateway`) mounted behind the CloudFront distribution under `/api/*`, sharing the same origin domain as the static SPA and eliminating CORS preflight overhead.
- **Bidirectional Real‑Time Streams**: Served via **Amazon API Gateway v2 WebSocket API** (`@aws-cdk/aws-apigatewayv2-alpha`) when the opt‑in `websockets-integration` skill is applied (see [ADR‑009](./009-websockets-async-processing.md)).

### 2.3 Direct `OUT_DIR` Inclusion & CI Wire‑Compatibility Guards

- **Rust Inclusion Pattern**: Backend modules include `prost-build` output directly via standard Rust `include!(concat!(env!("OUT_DIR"), "/<name>.rs"))` rather than custom wrapper macros.
- **Wire‑Format Breaking Change Detection**: A root `buf.work.yaml` workspace configuration and CI `check-protocols` job run both `buf lint` and `buf breaking` against the `main` branch on every pull request to prevent accidental wire‑incompatible schema changes.
