import { Construct } from 'constructs';
import * as cdk from 'aws-cdk-lib';

export interface BuildConfig {
  build: boolean;
  backendPath?: string;
  frontendPath?: string;
}

export interface DeploymentConfig {
  mode: 'local' | 'sandbox' | 'environment';
  environment?: 'staging' | 'production';
  aws: boolean;
  removalPolicy: cdk.RemovalPolicy;
  autoDeleteObjects: boolean;
  terminationProtection: boolean;
  domainName?: string;
  buildConfig: BuildConfig;
}

/**
 * Checks the current environment and loads the appropriate mode configuration.
 *
 * local mode for localstack is indicated by the presence of AWS_ENDPOINT_URL starting with "http://":
 * sandbox mode is indicated by the CDK context variable "mode" set to "sandbox". (`-c mode=sandbox`)
 * environment mode is the default for AWS deployments and REQUIRES an "environment" flag (staging|production).
 *
 * local & sandbox modes use resource removal policies that allow easy cleanup.
 *
 * environment mode requires an environment flag and a domain configuration via CDK context variables:
 * `-c environment=staging -c domain=staging.example.com -c hostedZoneId=Z123456ABCDEFG`.
 * The domain will be used for CloudFront distribution, API Gateway & Cognito user pool.
 *
 * In constructs check for `aws` flag to decided whether resources could & should be deployed to localstack.
 * - Cognito is omitted (replaced by cognito-local)
 * - CloudFront & frontend bucket is omitted (replaced by npm run dev)
 * - Lambdas are proxied to local cargo lambda watch server
 * - API Gateway is omitted (replaced by direct calls to cargo lambda watch)
 */
export function loadDeploymentConfig(scope: Construct): DeploymentConfig {
  const mode = scope.node.tryGetContext('mode') || 'environment';

  // Identify the build mode
  const buildConfig: BuildConfig = {
    build:
      scope.node.tryGetContext('build') === 'true' || scope.node.tryGetContext('build') === true,
    backendPath: scope.node.tryGetContext('backendPath'),
    frontendPath: scope.node.tryGetContext('frontendPath'),
  };

  // Check for Localstack & Lambda Proxy mode
  const awsEndpointUrl = process.env.AWS_ENDPOINT_URL;
  const dev = awsEndpointUrl && awsEndpointUrl.startsWith('http://');
  if (dev) {
    return {
      mode: 'local',
      aws: false,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      buildConfig: { build: false, backendPath: undefined, frontendPath: undefined },
    };
  }

  if (mode === 'sandbox') {
    return {
      mode: 'sandbox',
      aws: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      buildConfig,
    };
  }

  // mode === "environment"
  const environment = scope.node.tryGetContext('environment');
  if (environment !== 'staging' && environment !== 'production') {
    throw new Error(
      '❌ Context variable "environment" is required and must be either "staging" or "production" when mode is "environment" (default).',
    );
  }

  const domainName = scope.node.tryGetContext('domain');
  if (!domainName) {
    throw new Error('❌ Context variable "domain" is required for staging/production deployments.');
  }

  return {
    mode: 'environment',
    environment,
    aws: true,
    // Protects data on delete/update, but cleans up if initial creation fails (rollback).
    removalPolicy: cdk.RemovalPolicy.RETAIN_ON_UPDATE_OR_DELETE,
    autoDeleteObjects: false,
    terminationProtection: true,
    domainName,
    buildConfig,
  };
}
