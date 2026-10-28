# ADR-002 — Rust‑based AWS Lambda Functions as the Default Compute Unit

**Status**: Superceded by [ADR‑011](./011-arm64-rust-lambdas-and-ssm-config.md)  
**Date**: 2025‑07‑11

---

## 1. Design & Technology Decision

Every piece of backend compute **SHALL be implemented as a Rust AWS Lambda Function** built with `cargo lambda`.
The function artifacts are compiled for the `provided.al2` custom runtime (or the future official Rust runtime when available) and deployed through AWS CDK.

| Aspect             | Choice                                                                                               |
| ------------------ | ---------------------------------------------------------------------------------------------------- |
| Language           | **Rust**                                                                                             |
| Package format     | ZIP artifact uploaded by CDK; switch to container image only if >50 MB                               |
| Runtime            | Custom runtime (`bootstrap`) on Amazon Linux 2                                                       |
| Invocation sources | HTTP API Gateway v2, Cognito, EventBridge, SQS, DynamoDB Streams                                     |
| Observability      | `tracing` JSON logs → CloudWatch, X‑Ray tracing via `aws_lambda_powertools`                          |
| Configuration      | Environment variables resolved from **SSM Parameter Store**; secrets pulled from **Secrets Manager** |
| IAM                | One least‑privilege role per function, scoped to its data and AWS services                           |

**Why not always‑on containers (ECS/Fargate)?**
Lambda’s scale‑to‑zero billing and sub‑second cold start times (with Rust) align with our cost and ops goals; we will upgrade to containerized services only for workloads that exceed the 15‑minute limit or demand gRPC streaming.

---

## 2. Tool‑Set Support

- `backend/` folder with scaffolded Rust Lambda functions and shared libraries.
- `infrastructure/` folder with CDK constructs to deploy Rust Lambdas with best practices.
- `agent/` folder with agent configuration to add features to the backend.
- CI/CD pipeline to build and test Rust Lambdas.

---

### Consequences

- **Pros**: memory‑safe, small artifacts (\~5–10 MB), fast cold starts (<150 ms median), single language for all compute paths.
- **Cons**: longer compile times vs. Go; Rust learning curve; custom runtime maintenance until AWS ships an official one.

### Alternatives Considered

- **Go Lambda** – faster compile but larger binaries and no built‑in Protobuf ergonomics.
- **ECS/Fargate microservices** – predictable latency but higher idle cost and more ops surface area.
