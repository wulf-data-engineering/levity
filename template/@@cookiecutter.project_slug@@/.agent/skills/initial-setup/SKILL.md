---
name: initial-setup
description: Initial project setup guide covering npm dependencies, GitHub repository, AWS multi-account setup, CDK Foundation/Certificate stacks, and GitHub Actions variables.
---

# Initial Setup

Guide the user step-by-step through the initial setup of the generated project.

## 1. Install Dependencies & Format

Run `npm install` in `frontend/` and `infrastructure/`.
Suggest running formatters in `backend/`, `frontend/`, and `infrastructure/` because Cookiecutter value substitution may introduce formatting diffs. (You can do this on `main`, or suggest checking out `initial-setup` if changes appear.)

## 2. Connect to GitHub Repository

Check if a GitHub remote is configured:

```bash
git remote get-url origin
```

If not:
- Ask the user to create a new repository and provide its URL.
- Connect it without pushing yet:
  ```bash
  git remote add origin <repo-url>
  ```
- Suggest activating git hooks:
  ```bash
  git config core.hooksPath .githooks
  ```

## 3. AWS Account Setup

Explain that **two AWS accounts** are required for staging and production isolation:
1. A **Staging** AWS account
2. A **Production** AWS account

Ask the user to configure SSO profiles and log in:

```bash
aws configure sso --profile @@ cookiecutter.project_slug @@-staging
aws configure sso --profile @@ cookiecutter.project_slug @@-production
aws sso login --profile @@ cookiecutter.project_slug @@-staging
aws sso login --profile @@ cookiecutter.project_slug @@-production
```

Verify login status and capture both account IDs:

```bash
aws sts get-caller-identity --profile @@ cookiecutter.project_slug @@-staging
aws sts get-caller-identity --profile @@ cookiecutter.project_slug @@-production
```

## 4. Bootstrap Both Accounts

Bootstrap CDK in both `@@ cookiecutter.default_region @@` (or `eu-central-1`) and `us-east-1` (required for CloudFront ACM certificates):

```bash
cd infrastructure
npx cdk bootstrap aws://<staging-account-id>/eu-central-1 aws://<staging-account-id>/us-east-1 \
  --profile @@ cookiecutter.project_slug @@-staging
npx cdk bootstrap aws://<production-account-id>/eu-central-1 aws://<production-account-id>/us-east-1 \
  --profile @@ cookiecutter.project_slug @@-production
```

## 5. Deploy FoundationStack (Cross-Account Setup)

Deploy `FoundationStack` (`staging` first, then `production`) following `@../foundation-stack/SKILL.md`.
Capture `HostedZoneNameServers` from the **production** deployment outputs.

## 6. Configure DNS at Registrar

Provide the 4 **production** NS records and ask the user to set them as Custom DNS for `@@ cookiecutter.domain_name @@` at their registrar. Explain that staging NS records are *not* entered at the registrar because production delegates `staging.@@ cookiecutter.domain_name @@` automatically.

## 7. Configure GitHub Repository

While waiting for DNS propagation, guide the user through `/configure-github-repo` (`@../configure-github-repo/SKILL.md`).

## 8. Deploy CertificateStack

Verify DNS propagation:

```bash
dig +short NS staging.@@ cookiecutter.domain_name @@
dig +short NS @@ cookiecutter.domain_name @@
```

Deploy `CertificateStack` in both environments (`us-east-1` certificates for CloudFront):

```bash
cd infrastructure
npx cdk deploy CertificateStack \
  --profile @@ cookiecutter.project_slug @@-staging \
  --require-approval never \
  -c environment=staging \
  -c domain=staging.@@ cookiecutter.domain_name @@

npx cdk deploy CertificateStack \
  --profile @@ cookiecutter.project_slug @@-production \
  --require-approval never \
  -c environment=production \
  -c domain=@@ cookiecutter.domain_name @@
```

Capture `CertificateArnOutput` from both deployments.

## 9. Configure GitHub Variables

Offer to set repository variables using `gh` CLI (check `gh --version` and `gh auth status` first):

```bash
gh variable set DOMAIN_STAGING -b "staging.@@ cookiecutter.domain_name @@" -R <org/repo>
gh variable set DOMAIN_PRODUCTION -b "@@ cookiecutter.domain_name @@" -R <org/repo>
gh variable set CERTIFICATE_ARN_STAGING -b "<CertificateArnStaging>" -R <org/repo>
gh variable set CERTIFICATE_ARN_PRODUCTION -b "<CertificateArnProduction>" -R <org/repo>
gh variable set AWS_ROLE_ARN_STAGING -b "<GitHubRoleArnStaging>" -R <org/repo>
gh variable set AWS_ROLE_ARN_PRODUCTION -b "<GitHubRoleArnProduction>" -R <org/repo>
```

Keep these exact variable names (any extra app-specific variables should follow the `_STAGING` / `_PRODUCTION` suffix convention).

## 10. Verify

Check SES email identity verification status:

```bash
aws sesv2 get-email-identity \
  --email-identity @@ cookiecutter.domain_name @@ \
  --profile @@ cookiecutter.project_slug @@-production

aws sesv2 get-email-identity \
  --email-identity staging.@@ cookiecutter.domain_name @@ \
  --profile @@ cookiecutter.project_slug @@-staging
```

- **Expected**: `SUCCESS` (Verified).
- **If `PENDING`**: Check `DkimAttributes.Status` and DNS propagation via `dig`, then retry later.
- Once verified, suggest pushing to `main` to trigger the first deployment.
