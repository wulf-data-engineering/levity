#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { AppStack } from '../lib/app-stack';
import { FoundationStack } from '../lib/foundation-stack';
import { CertificateStack } from '../lib/certificate-stack';
import { loadDeploymentConfig } from '../lib/config';

const app = new cdk.App();
const deploymentConfig = loadDeploymentConfig(app);

const env = {
  account: deploymentConfig.mode === 'local' ? '000000000000' : process.env.CDK_DEFAULT_ACCOUNT,
  region: deploymentConfig.mode === 'local' ? '%[ cookiecutter.default_region ]%' : process.env.CDK_DEFAULT_REGION,
};

const githubRepo = app.node.tryGetContext('githubRepo');
if (githubRepo) {
  new FoundationStack(app, 'FoundationStack', {
    env,
    deploymentConfig,
    githubRepo,
  });
}

let certificateArn: string | undefined = undefined;

// Create the cross-region dependencies if we are provided a domain configuration
// For staging and production, we typically pass the context flags needed.
if (deploymentConfig.domainName) {
  const certStack = new CertificateStack(app, 'CertificateStack', {
    env: {
      account: env.account,
      region: 'us-east-1', // CloudFront strictly enforces ACM certificates to be in us-east-1
    },
    crossRegionReferences: true,
    domainName: deploymentConfig.domainName!,
  });
  certificateArn = certStack.certificateArn;
}

new AppStack(app, 'AppStack', {
  env,
  crossRegionReferences: true,
  deploymentConfig,
  certificateArn,
});
