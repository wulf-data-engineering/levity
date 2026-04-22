---
trigger: model_decision
description: Instructions for deployments
---

When deploying the infrastructure for local development or sandbox AWS environments, you must strictly follow the documentation in @../../infrastructure/README.md.

Agents frequently fail to supply necessary CDK context variables or use the wrong commands. 

Always navigate to the `infrastructure/` directory before running any `npm` or `cdk` commands.

For Sandbox deployments, ask the user for the AWS profile of the sandbox, pass it to cdk and check that you have included the `-c mode=sandbox` and `-c build=true` flags as defined in the README.