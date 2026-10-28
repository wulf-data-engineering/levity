# ADR‑013 — Composite Keys, TTL & Serde Flatten Evolution in Versioned Tables

**Status**: Proposed (Supercedes [ADR‑005](./005-dynamodb.md))  
**Date**: 2026‑10‑28

---

## 1. Recap of ADR‑005

[ADR‑005](./005-dynamodb.md) established the table‑per‑entity DynamoDB design using the generic `Versioned<T>` wrapper (`data`, `data_version`, `last_write` optimistic locking), flat JSON map storage via `serde_dynamo`, and compile‑time JSON Schema snapshot tests via `schemars` and `expectorate`.

---

## 2. Focused Changes

### 2.1 Composite Sort Keys, TTL & GSIs in `VersionedTable`

The `VersionedTable` CDK construct (`infrastructure/lib/constructs/backend/dynamodb.ts`) supports:
- **Optional Sort Key (`sortKey`)**: Enables 1‑to‑many entity relationships within a single domain table (e.g., `PK: userId`, `SK: topicId` in `websocket_connections`).
- **Optional Time‑to‑Live (`timeToLiveAttribute`)**: Enables automatic DynamoDB expiration of ephemeral records (such as stale WebSocket connections or session state).
- **Global Secondary Indexes (`addGlobalSecondaryIndex`)**: Allows inverse lookups (e.g., `connection-index` on `connectionId` with `KEYS_ONLY` projection).

### 2.2 Serde `#[serde(flatten)]` Schema‑Evolution Guardrail

`Versioned<T>` serializes the inner domain struct `data: T` into top‑level DynamoDB attributes using `#[serde(flatten)]`. When `T` is a `prost`-generated Protobuf struct annotated with Serde attributes in `backend/build.rs`:
- Due to a known Serde limitation with `#[serde(flatten)]`, container‑level `#[serde(default)]` attributes on `T` are ignored when deserializing flattened maps.
- Consequently, adding a new field to a persisted `.proto` message will cause deserialization of older DynamoDB records lacking that attribute to fail with a `missing field` error unless an explicit **field‑level** default attribute is registered in `backend/build.rs`:
  ```rust
  config.field_attribute("package_name.MessageName.new_field", "#[serde(default)]");
  ```
- This requirement is enforced via agent instructions in `.agent/skills/backend/SKILL.md` alongside the `schemars` snapshot tests.
