---
name: foundation-stack
description: Step-by-step runbook for deploying or updating the AWS CDK FoundationStack across staging and production accounts with DNS delegation.
---

# FoundationStack Deployment

When deploying the application for the first time or changing `FoundationStack` in `infrastructure/`, deploy **`staging` first** and **`production` second** in that exact order.

## 1. Verify AWS CLI Authentication

Ensure both profiles are logged in:

```bash
aws sts get-caller-identity --profile @@ cookiecutter.project_slug @@-staging
aws sts get-caller-identity --profile @@ cookiecutter.project_slug @@-production
```

## 2. Deploy Staging Account

Domain for staging: `staging.@@ cookiecutter.domain_name @@`.

```bash
git remote get-url origin
cd infrastructure
npx cdk deploy FoundationStack \
  --profile @@ cookiecutter.project_slug @@-staging \
  --require-approval never \
  -c environment=staging \
  -c domain=staging.@@ cookiecutter.domain_name @@ \
  -c githubRepo=<org/repo>
```

Capture `HostedZoneId`, `GitHubRoleArn`, and **`HostedZoneNameServers`** from the staging stack outputs.

## 3. Deploy Production Account (with DNS Delegation)

Domain for production: `@@ cookiecutter.domain_name @@`.

1. Check the diff first (and verify the hosted zone will not be replaced — if it would be replaced, notify the user that registrar updates will be required):

   ```bash
   cd infrastructure
   npx cdk diff FoundationStack \
     --profile @@ cookiecutter.project_slug @@-production \
     -c environment=production \
     -c domain=@@ cookiecutter.domain_name @@ \
     -c githubRepo=<org/repo> \
     -c stagingNameServers="ns-XXXX.awsdns-XX.org, ns-YYYY.awsdns-YY.co.uk, ..."
   ```

2. Deploy `FoundationStack` to production, passing `HostedZoneNameServers` from Step 2:

   ```bash
   cd infrastructure
   npx cdk deploy FoundationStack \
     --profile @@ cookiecutter.project_slug @@-production \
     --require-approval never \
     -c environment=production \
     -c domain=@@ cookiecutter.domain_name @@ \
     -c githubRepo=<org/repo> \
     -c stagingNameServers="ns-XXXX.awsdns-XX.org, ns-YYYY.awsdns-YY.co.uk, ..."
   ```
