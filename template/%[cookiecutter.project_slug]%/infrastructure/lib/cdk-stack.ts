import * as cdk from "aws-cdk-lib";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as sqs from "aws-cdk-lib/aws-sqs";
import { Construct } from "constructs";
import { join } from "path";
import { cargoLambdaFunction } from "./cargo-lambda-function";

export class CdkLocalstackDemoStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new s3.Bucket(this, "SomeBucket", {
      bucketName: "some-bucket",
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    cargoLambdaFunction(this, "HelloFunction", {
      functionName: "hello-function",
      manifestPath: join(__dirname, "..", "backend"),
      binaryName: "hello",
    });

    const queue = new sqs.Queue(this, "SomeQueue", {
      queueName: "some-queue",
    });

    const messageHandler = cargoLambdaFunction(this, "MessageHandlerFunction", {
      functionName: "message-handler-function",
      // Path to the root directory.
      manifestPath: join(__dirname, "..", "backend"),
      binaryName: "message-handler",
    });

    queue.grantConsumeMessages(messageHandler);

    messageHandler.addEventSource(
      new cdk.aws_lambda_event_sources.SqsEventSource(queue, {}),
    );
  }
}
