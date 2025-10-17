import {Function} from "aws-cdk-lib/aws-lambda";
import {RustFunction} from "cargo-lambda-cdk";
import {Construct} from "constructs";
import {RustFunctionProps} from "cargo-lambda-cdk/lib/function";
import {aws_lambda as lambda} from "aws-cdk-lib";

const awsEndpointUrl = process.env.AWS_ENDPOINT_URL;

/**
 * In a regular environment creates a Rust-based Lambda function using cargo-lambda-cdk.
 *
 * In a local environment (when AWS_ENDPOINT_URL is set), creates a Node.js Lambda function
 * that forwards requests to the local cargo-lambda HTTP server (cargo lambda watch).
 * That allows hot-reloading, easier log access and faster development cycles.
 */
export function cargoLambdaFunction(scope: Construct, resourceName: string, props: RustFunctionProps): Function {
    if (!awsEndpointUrl) {
        return new RustFunction(scope, resourceName, props)
    } else {
        const lambdaPath = props.binaryName || props.functionName || resourceName;
        const url = `http:///host.docker.internal:9000/2015-03-31/functions/${lambdaPath}/invocations`
        return new lambda.Function(scope, resourceName, {
            functionName: props.functionName,
            runtime: lambda.Runtime.NODEJS_22_X,
            handler: 'index.handler',
            code: lambda.Code.fromInline(code(url))
        })
    }
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
};`
}
