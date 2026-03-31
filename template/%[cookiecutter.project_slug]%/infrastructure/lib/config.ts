import { Construct } from "constructs";
import * as cdk from "aws-cdk-lib";
import * as route53 from "aws-cdk-lib/aws-route53";

export interface DomainConfig {
  domainName: string; // FQDN (e.g. "staging.example.com")
  hostedZone?: route53.IHostedZone; // optional for the FoundationStack
}

export interface DeploymentConfig {
  mode: "local" | "sandbox" | "environment";
  environment?: "staging" | "production";
  aws: boolean;
  removalPolicy: cdk.RemovalPolicy;
  autoDeleteObjects: boolean;
  terminationProtection: boolean;
  domain?: DomainConfig;
  build: boolean;
  backendPath?: string;
  frontendPath?: string;
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
  const mode = scope.node.tryGetContext("mode") || "environment";
  
  // Identify the build mode
  const backendPath = scope.node.tryGetContext("backendPath");
  const frontendPath = scope.node.tryGetContext("frontendPath");
  const explicitBuild = scope.node.tryGetContext("build");
  
  // Build defaults to FALSE
  // Priority: 
  // 1. Path provided? (build=false)
  // 2. Explicit build context?
  // 3. Default (false -> stub)
  let build = false;
  
  // Explicit override: -c build=true or -c build=false
  if (explicitBuild === "true" || explicitBuild === true) {
    build = true;
  } else if (explicitBuild === "false" || explicitBuild === false) {
    build = false;
  }

  if (build) {
    console.log(`[CDK] local build is ENABLED for construct: ${scope.node.id}`);
  }

  // Check for Localstack & Lambda Proxy mode
  const awsEndpointUrl = process.env.AWS_ENDPOINT_URL;
  const dev = awsEndpointUrl && awsEndpointUrl.startsWith("http://");
  if (dev) {
    return {
      mode: "local",
      aws: false,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      build,
    };
  }

  if (mode === "sandbox") {
    return {
      mode: "sandbox",
      aws: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      build,
    };
  }

  // mode === "environment"
  const environment = scope.node.tryGetContext("environment");
  if (environment !== "staging" && environment !== "production") {
    throw new Error('❌ Context variable "environment" is required and must be either "staging" or "production" when mode is "environment" (default).');
  }

  const domainName = scope.node.tryGetContext("domain");
  if (!domainName) {
    throw new Error('❌ Context variable "domain" is required for staging/production deployments.');
  }

  // hosted zone is required for the AppStack but has to be optional for the FoundationStack
  let hostedZone: route53.IHostedZone | undefined;
  const hostedZoneId = scope.node.tryGetContext("hostedZoneId");
  if (hostedZoneId) {
    hostedZone = route53.HostedZone.fromHostedZoneAttributes(scope, "Zone", {
      hostedZoneId,
      zoneName: domainName,
    });
  }

  const domain: DomainConfig = { domainName, hostedZone };

  return {
    mode: "environment",
    environment,
    aws: true,
    // Protects data on delete/update, but cleans up if initial creation fails (rollback).
    removalPolicy: cdk.RemovalPolicy.RETAIN_ON_UPDATE_OR_DELETE,
    autoDeleteObjects: false,
    terminationProtection: true,
    domain,
    build,
    backendPath,
    frontendPath,
  };
}
