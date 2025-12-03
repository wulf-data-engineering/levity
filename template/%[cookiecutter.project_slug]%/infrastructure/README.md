# Infrastructure

The infrastructure is defined using **AWS CDK** in **TypeScript**.

## Modes

There are three modes of deployment defined in [lib/config.ts](lib/config.ts).

### environment (default)

Deploys against an **AWS** account, typically by CI/CD into an environment like _production_ or _staging_.

Stateful resources are retained in case of stack deletion. Termination protection is enabled.

### sandbox (`cdk deploy -c mode=sandbox`)

Deploys against an **AWS** account, typically manually or by a GitHub action into a personal sandbox account.

Stateful resources are destroyed in case of stack deletion. Termination protection is disabled.

### local (`AWS_ENDPOINT_URL=http://...`)

Deploys against **localstack**.

- Cognito is not deployed automatically because **cognito-local** is used for local development.
- For local development all Lambda functions are forwarded to **cargo lambda watch**.
- Frontend is not deployed to CloudFront & S3 and served by **npm run dev** instead.

### Domain

_environment_ & _sandbox_ modes can deploy CloudFront with a custom domain.

```bash
cdk deploy -c domain=mydomain.com -c [hostedZoneId=...]
```

The hosted zone id is optional, CDK will look it up automatically if not provided.

## Requirements

CDK & Docker

```bash
npm install -g aws-cdk
cdk --version  # to verify installation
```

## Configuration

### ~/.aws/config

```
[profile localstack]
source_profile = localstack
region = eu-central-1
output = json
```

### ~/.aws/credentials

```
[localstack]
aws_access_key_id=test
aws_secret_access_key=test
```

### ~/.zshrc

There is cdklocal as a tool, but it causes some trouble on macOS.

```bash
alias cdklocal="AWS_PROFILE=localstack AWS_ENDPOINT_URL=\"http://127.0.0.1:4566\" cdk"
```

## Useful commands

- `npm run build` compile typescript to js
- `npm run watch` watch for changes and compile
- `npm run test` perform the jest unit tests
- `npm run test -- -u` update the jest snapshots
- `cdk deploy` deploy this stack to your default AWS account/region
- `cdk diff` compare deployed stack with current state
- `cdk synth` emits the synthesized CloudFormation template

## Deploy locally

Start localstack in one terminal on top level:

```bash
docker compose up
```

Deploy the infrastructure in another terminal in this directory:

```bash
npm install
cdklocal bootstrap # once
cdklocal deploy
```
