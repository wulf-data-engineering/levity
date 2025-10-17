#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { CdkLocalstackDemoStack } from '../lib/cdk-stack';

const app = new cdk.App();
new CdkLocalstackDemoStack(app, 'CdkLocalstackDemoStack', {
    env: { account: '000000000000', region: 'eu-central-1' },
});

