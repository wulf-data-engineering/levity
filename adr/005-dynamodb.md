# ADR-005 — DynamoDB as the Primary Data Store for Backend Services

**Status**: Superceded by [ADR‑013](./013-dynamodb-composite-keys-ttl-and-serde-flatten.md)  
**Date**: 2025‑07‑11

---

## 1. Design & Technology Decision

### 1.1 Table‑per‑entity strategy

Each domain entity gets its **own DynamoDB table** and a **single repository struct** that encapsulates CRUD logic:

| Operation      | Method             | Consistency                 |
| -------------- | ------------------ | --------------------------- |
| Read (default) | `read(&pk)`        | Eventually consistent       |
| Strong read    | `read_strong(&pk)` | `ConsistentRead=true`       |
| Insert         | `insert(entity)`   | Fails if key exists         |
| Update         | `update(entity)`   | Conditional on `last_write` |
| Delete         | `delete(&pk)`      | Conditional on `last_write` |

### 1.2 Version wrapper & optimistic locking

To separate bookkeeping from domain data we introduce a generic wrapper:

```rust
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Versioned<T> {
    pub data: T,
    /// Schema migration marker – bump only when the JSON shape of `T` changes.
    pub data_version: u16,
    /// Last successful write in milliseconds since Unix epoch.  Acts as an optimistic‑lock token.
    pub last_write: i64,
}
```

Every table stores a `Versioned<T>` item where `T` is the business struct (e.g., `AccountData`).

| Field                       | Purpose                            | Behavior                                                        |
| --------------------------- | ---------------------------------- | --------------------------------------------------------------- |
| `last_write` _(i64 millis)_ | Avoid lost updates + aid debugging | Set to `now()` on insert/update; used in `ConditionExpression`. |
| `data_version` _(u16)_      | Track schema migrations            | Increment **only** on incompatible JSON shape changes.          |

**Update flow**

1. `read()` (eventual) → get item + `last_write`.
2. Attempt `update()` with condition `last_write = :expected`.
3. On `ConditionalCheckFailed`, retry once with `read_strong()`; otherwise return 409 Conflict.

### 1.3 Data format

_Items are stored as a flat JSON map attribute._ The tool set favors JSON because it is easy to inspect in the AWS Console while remaining close to the API’s optional JSON representation (ADR‑004).

Schema‑evolution guardrails:

- Every entity struct derives `Serialize`, `Deserialize`, **and** `JsonSchema` (via `schemars`).
- A **snapshot JSON Schema** is checked into `/backend/schema/{entity}.schema.json`.
- A unit test regenerates the schema at compile time and fails if it differs. The failure message reminds the developer to:
  1. Increment `data_version`.
  2. Add a lazy‑migration handler if the change is _not_ backward compatible.

---

## 2. Intended Usage in Applications

### 2.1 Repository skeleton

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AccountData {
    pub pk: String,
    pub name: String,
    pub balance: i64,
}

/// Type alias used by business code
pub type Account = Versioned<AccountData>;

pub struct AccountRepo { /* table & client */ }
impl AccountRepo {
    pub async fn read(&self, pk: &str) -> Result<Option<Account>> { … }

    pub async fn update(&self, acc: &AccountData) -> Result<()> {
        let now = chrono::Utc::now().timestamp_millis();
        self.ddb
            .update_item()
            .table_name(&self.table)
            .key("pk", pk.into_attr())
            .condition_expression("last_write = :v")
            .expression_attribute_values(":v", acc_wrapper.last_write.into_attr())
            .item("data", serde_dynamo::to_attribute_value(acc)? )
            .item("last_write", now.into_attr())
            .send()
            .await?;
        Ok(())
    }
}
```

### 2.2 Schema‑snapshot test (per entity)

```rust
#[test]
fn schema_has_not_changed() {
    let schema = schemars::schema_for!(AccountData);
    let current = serde_json::to_string_pretty(&schema).unwrap();
    let stored = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema/account_data.schema.json"));
    expectorate::assert_strings_equal!(stored, &current, "\n\nSchema changed! → Update data_version and provide migration logic.");
}
```

---

## 3. Tool‑Set Support

- `backend/lib/` contains the versioned trait
- `infrastructure/` has CDK construct for versioned table with PITR and least‑privilege IAM for a repo function
- example table, repo and unit test including schema check
- agent config how to create tables and repos

---

### Consequences

- **Pros**: easy console debugging, automatic detection of breaking schema changes, built‑in last‑write optimistic locking avoids overwrites.
- **Cons**: JSON storage is larger than binary Avro; strong reads on retry path add latency under contention but only when conflicts occur.

### Alternatives Considered

- Binary Avro blobs with Glue Schema Registry—better storage efficiency but poor debuggability and extra infra.
- External migration service instead of lazy upgrades—adds complexity and deployment ordering.
