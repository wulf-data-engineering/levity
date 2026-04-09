import * as cdk from 'aws-cdk-lib';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as logs from 'aws-cdk-lib/aws-logs';
import { Construct } from 'constructs';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import { execSync } from 'child_process';
import { DeploymentConfig } from '../../config';

export interface BackendLambdaProps extends lambda.FunctionOptions {
  deploymentConfig: DeploymentConfig;
  binaryName: string; // The name of the [[bin]] in Cargo.toml
  environment?: { [key: string]: string }; // Environment variables
}

/**
 * In an AWS deployment creates a Rust-based Lambda function.
 *
 * In a local deployment, creates a Node.js Lambda function
 * that forwards requests to the local cargo-lambda HTTP server (cargo lambda watch).
 * That allows hot-reloading, easier log access and faster development cycles.
 */
export function backendLambda(
  scope: Construct,
  id: string,
  props: BackendLambdaProps,
): lambda.Function {
  if (props.deploymentConfig.aws) return rustLambda(scope, id, props);
  else return proxyLambda(scope, id, props);
}

export interface BackendLambdaApiProps extends BackendLambdaProps {
  apiRoot: apigateway.Resource; // The API Gateway resource to attach the lambda to
  authorizer?: apigateway.IAuthorizer; // Whether to protect the endpoint with an authorizer
  authorizationType?: apigateway.AuthorizationType; // Specifies the type of authorization (default: Cognito)
  path?: string; // The path under the apiRoot to attach the lambda to (default: binaryName)
}

/**
 * Creates a cargo lambda function which is wired as an API Gateway handler.
 * Uses the configured path or else the binary name to route requests to the lambda.
 * Lambda is optionally protected with a (Cognito) Authorizer.
 *
 * @see backendLambda
 */
export function backendLambdaApi(
  scope: Construct,
  id: string,
  props: BackendLambdaApiProps,
): lambda.Function {
  const lambdaFunction = backendLambda(scope, id, props);

  const integration = new apigateway.LambdaIntegration(lambdaFunction);

  const resource = props.apiRoot.addResource(props.path || props.binaryName);

  const methodOptions: apigateway.MethodOptions = props.authorizer
    ? {
        authorizer: props.authorizer,
        authorizationType: props.authorizationType || apigateway.AuthorizationType.COGNITO,
      }
    : {};

  resource.addMethod('ANY', integration, methodOptions);

  resource.addProxy({
    defaultIntegration: integration,
    anyMethod: true,
    defaultMethodOptions: methodOptions,
  });

  return lambdaFunction;
}

// Production deployment

function rustLambda(scope: Construct, id: string, props: BackendLambdaProps) {
  const logGroup = new logs.LogGroup(scope, `${id}LogGroup`, {
    logGroupName: `/aws/lambda/${props.functionName || props.binaryName}`,
    retention: logs.RetentionDays.ONE_WEEK,
    removalPolicy: props.deploymentConfig.removalPolicy,
  });

  let code: lambda.Code;
  if (props.deploymentConfig.buildConfig.backendPath) {
    // Find the specific binary from the pre-built backend path
    const binFolder = path.resolve(
      process.cwd(),
      props.deploymentConfig.buildConfig.backendPath,
      props.binaryName,
    );
    if (!fs.existsSync(binFolder)) throw new Error(`Pre-built binary not found: ${binFolder}`);
    code = lambda.Code.fromAsset(binFolder);
  } else if (props.deploymentConfig.buildConfig.build) {
    code = bundleRustCode(
      props.binaryName,
      props.deploymentConfig.mode === 'sandbox' ? 'sandbox' : 'release',
    );
  } else {
    // Default to stub for foundation synthesis speed
    code = lambda.Code.fromAsset(path.join(process.cwd(), 'stub'));
  }

  return new lambda.Function(scope, id, {
    ...props,
    functionName: props.functionName || props.binaryName,
    runtime: lambda.Runtime.PROVIDED_AL2023,
    handler: 'bootstrap',
    architecture: lambda.Architecture.ARM_64,
    code,
    logGroup,
  });
}

/**
 * On Linux ARM64 hosts, builds the Rust binary locally.
 * On other platforms (Mac, Windows), uses Docker to build the binary.
 */
function bundleRustCode(binName: string, profile: string): lambda.AssetCode {
  const backendPath = path.join(__dirname, '../../../../backend');
  const outputPath = path.join(backendPath, 'target', 'lambda', binName, 'bootstrap');

  const sourceHash = cdk.FileSystem.fingerprint(backendPath, { exclude: ['target/**'] });

  return lambda.Code.fromAsset(backendPath, {
    assetHash: `${binName}-${profile}-${sourceHash}`,
    assetHashType: cdk.AssetHashType.CUSTOM,
    bundling: {
      image: cdk.DockerImage.fromRegistry('alpine'),
      local: {
        tryBundle(outputDir: string) {
          console.log('Building Rust Backend...');

          execSync(`cargo lambda build --profile ${profile} --bin ${binName} --arm64`, {
            cwd: backendPath,
            stdio: 'inherit',
          });

          if (!fs.existsSync(outputPath))
            throw new Error(`Rust build output missing at ${outputPath}`);

          execSync(`cp -r ${outputPath} ${outputDir}`, { stdio: 'inherit' });
          return true;
        },
      },
    },
  });
}

// Local development

function proxyLambda(scope: Construct, id: string, props: BackendLambdaProps) {
  const lambdaPath = props.binaryName;
  const url = `http://host.docker.internal:9000/2015-03-31/functions/${lambdaPath}/invocations`;
  return new lambda.Function(scope, id, {
    functionName: props.functionName || props.binaryName,
    runtime: lambda.Runtime.NODEJS_22_X,
    handler: 'index.handler',
    code: lambda.Code.fromInline(code(url)),
  });
}

function code(url: string): string {
  return `exports.handler = async (event) => {
    
  console.log("Forwarding event to cargo-lambda server at ${url}: ", JSON.stringify(event));
    
  const response = await fetch("${url}", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(event),
  });

  // Read the response as raw bytes
  const buffer = await response.arrayBuffer();
  const body = Buffer.from(buffer).toString("base64");

  // Forward everything back as Lambda Proxy integration expects
  return {
    statusCode: response.status,
    headers: Object.fromEntries(response.headers.entries()),
    body,
    isBase64Encoded: true,
  };
};`;
}
