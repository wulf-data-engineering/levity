import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Cdk from '../lib/app-stack';
import { FoundationStack } from '../lib/foundation-stack';

test('Infrastructure Created', () => {
  delete process.env.AWS_ENDPOINT_URL;
  const app = new cdk.App({
    context: {
      environment: 'test',
      project_slug: 'tool-set-iac-test',
      package_name: 'tool_set_iac_test',
      domain_name: 'example.com',
      hosted_zone_id: 'Z1234567890',
      email_sender_address: 'noreply@example.com',
      email_sender_name: 'Test Sender',
      email_replyto: 'noreply@example.com',
      aws: true,
      build: false,
    },
  });
  const stack = new Cdk.AppStack(app, 'CdkTestStack', {
    deploymentConfig: {
      mode: 'environment',
      environment: 'staging',
      aws: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      buildConfig: { build: false },
    },
  } );
  const template = Template.fromStack(stack);

  // Verify API Gateway
  template.hasResourceProperties('AWS::ApiGateway::RestApi', {
    Name: 'RestApi',
  });
});

test('FoundationStack OIDC Trust Policy Supports Standard and Immutable Subjects', () => {
  const app = new cdk.App();
  const stack = new FoundationStack(app, 'FoundationTestStack', {
    deploymentConfig: {
      mode: 'environment',
      environment: 'staging',
      aws: true,
      domainName: 'example.com',
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true,
      terminationProtection: false,
      buildConfig: { build: false },
    },
    githubRepo: 'wulf-data-engineering/levity',
  });
  const template = Template.fromStack(stack);

  template.hasResourceProperties('AWS::IAM::Role', {
    RoleName: 'GitHubActionRole',
    AssumeRolePolicyDocument: {
      Statement: [
        {
          Action: 'sts:AssumeRoleWithWebIdentity',
          Effect: 'Allow',
          Condition: {
            StringLike: {
              'token.actions.githubusercontent.com:sub': [
                'repo:wulf-data-engineering/levity:*',
                'repo:wulf-data-engineering@*/levity@*:*',
              ],
            },
          },
        },
      ],
    },
  });
});
