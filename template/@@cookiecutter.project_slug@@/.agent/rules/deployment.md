---
trigger: model_decision
description: AWS CLI and AWS CDK rules, mandatory --profile flags, FoundationStack deployment order, and CDK context variables (-c) whenever running aws or cdk commands or deploying.
---

When deploying infrastructure for local development or AWS environments, strictly follow @../../infrastructure/README.md.

Always run `npm` and `cdk` commands from inside `infrastructure/`.

## Mandatory AWS Profile Rules

**CRITICAL**: All `aws` CLI and `cdk` commands **MUST** include an explicit `--profile` flag:
- **Staging**: `--profile @@ cookiecutter.project_slug @@-staging` with `-c environment=staging`
- **Production**: `--profile @@ cookiecutter.project_slug @@-production` with `-c environment=production`
- **Sandbox**: Ask the user for their sandbox AWS profile name (e.g. `@@ cookiecutter.project_slug @@-sandbox`) and pass `--profile <profile_name>` with `-c mode=sandbox -c build=true`

## FoundationStack Ordering

`FoundationStack` must be deployed to **`staging` before `production`** so `HostedZoneNameServers` from staging can be passed to production via `-c stagingNameServers=...` for DNS delegation. Follow the `foundation-stack` skill (`@../skills/foundation-stack/SKILL.md`) for the full runbook.
