import { Construct } from 'constructs';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import * as cognito from 'aws-cdk-lib/aws-cognito';
import { Api } from './backend/api';
import { Identity } from './backend/identity';
import { DeploymentConfig } from '../config';
import * as route53 from 'aws-cdk-lib/aws-route53';
import * as ses from 'aws-cdk-lib/aws-ses';
import { VersionedTable } from './backend/dynamodb';
import { AttributeType, ProjectionType } from 'aws-cdk-lib/aws-dynamodb';
import * as ssm from 'aws-cdk-lib/aws-ssm';

export interface BackendProps {
  config: DeploymentConfig;
  hostedZone?: route53.IHostedZone;
}

/**
 * Sets up the backend resources.
 * Exposes the /api entrypoint for CloudFront.
 * Exposes the Cognito User Pool for authentication from frontend.
 */
export class Backend extends Construct {
  public readonly restApi?: apigateway.RestApi;
  public readonly userPool?: cognito.UserPool;
  public readonly userPoolClient?: cognito.UserPoolClient;

  constructor(scope: Construct, id: string, props: BackendProps) {
    super(scope, id);
    const deploymentConfig = props.config;

    const usersTable = new VersionedTable(this, 'UsersTable', {
      tableName: 'users',
      removalPolicy: deploymentConfig.removalPolicy,
    });

    usersTable.addGlobalSecondaryIndex({
      indexName: 'email-index',
      partitionKey: {
        name: 'email',
        type: AttributeType.STRING,
      },
      projectionType: ProjectionType.ALL,
    });

    const usersTableParam = new ssm.StringParameter(this, 'UsersTableParam', {
      parameterName: '/@@ cookiecutter.project_slug @@/users-table',
      stringValue: usersTable.tableName,
    });

    // Locally cognito-local and cargo lambda watch are used instead
    if (deploymentConfig.aws) {
      const identity = new Identity(this, 'Identity', {
        deploymentConfig,
        usersTable,
        usersTableParam,
        hostedZone: props.hostedZone,
      });

      this.userPool = identity.userPool;
      this.userPoolClient = identity.userPoolClient;

      const userPoolParam = new ssm.StringParameter(this, 'UserPoolParam', {
        parameterName: '/@@ cookiecutter.project_slug @@/user-pool-id',
        stringValue: this.userPool.userPoolId,
      });

      const api = new Api(this, 'Api', {
        deploymentConfig,
        userPool: this.userPool,
        userPoolParam,
        usersTable,
        usersTableParam,
      });
      this.restApi = api.gateway;
    }
  }
}
