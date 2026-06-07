---
trigger: model_decision
description: Instructions for deployments
---

When deploying the infrastructure for local development or sandbox AWS environments, you must strictly follow the documentation in @../../infrastructure/README.md.

Agents frequently fail to supply necessary CDK context variables or use the wrong commands. 

Always navigate to the `infrastructure/` directory before running any `npm` or `cdk` commands.

### Profile Rules for Deployment
- **Staging**: Always deploy with `--profile @@ cookiecutter.project_slug @@-staging` and `-c environment=staging`.
- **Production**: Always deploy with `--profile @@ cookiecutter.project_slug @@-production` and `-c environment=production`.
- **Sandbox**: Ask the user for the sandbox AWS profile name (e.g. `@@ cookiecutter.project_slug @@-sandbox`) before deploying, and pass it via `--profile <profile_name>`. Ensure you include `-c mode=sandbox` and `-c build=true`.