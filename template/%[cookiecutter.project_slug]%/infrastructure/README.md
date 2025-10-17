# Infrastructure

The infrastructure is defined using **AWS CDK** in **TypeScript**.
It deploys against **AWS**.
For local development, it can also deploy against **localstack**.
Cognito is not deployed automatically because **cognito-local** is used for local
development.
For local development all Lambda functions are forwarded to **cargo lambda watch**.

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
alias cdklocal="AWS_PROFILE=localstack AWS_PROFILE=localstack AWS_ENDPOINT_URL=\"http://127.0.0.1:4566\" cdk"
```

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`  deploy this stack to your default AWS account/region
* `cdk diff`    compare deployed stack with current state
* `cdk synth`   emits the synthesized CloudFormation template

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
