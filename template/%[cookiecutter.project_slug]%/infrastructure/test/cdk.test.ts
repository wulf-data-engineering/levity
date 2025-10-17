import * as cdk from "aws-cdk-lib";
import { Template } from "aws-cdk-lib/assertions";
import * as Cdk from "../lib/cdk-stack";

test("Lambda Function Created", () => {
  const app = new cdk.App();
  // WHEN
  const stack = new Cdk.CdkLocalstackDemoStack(app, "MyTestStack");
  // THEN
  const template = Template.fromStack(stack);

  template.hasResourceProperties("AWS::Lambda::Function", {
    FunctionName: "hello-lambda",
    Handler: "hello.handler",
    Runtime: "nodejs18.x",
    Description: "A simple Lambda function that returns a greeting",
    MemorySize: 128,
    Timeout: 10,
  });
});
