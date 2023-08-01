import * as cdk from 'aws-cdk-lib';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { Construct } from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class CdkStack extends cdk.Stack {
  private readonly stageName = "dev"

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const helloLambda = new lambda.Function(this, "rust-hello", {
      functionName: "hello-rust",
      code: lambda.Code.fromAsset(
        "../target/lambda/demo-lambda"
      ),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: "not.required",
      environment: {
        RUST_BACKTRACE: "1",
        APP_APPLICATION__STAGE_NAME: this.stageName,
      },
      logRetention: RetentionDays.ONE_DAY,
    });


    new apigateway.LambdaRestApi(this, "api", {
      restApiName: "rust-api",
      deployOptions: {
        stageName: this.stageName,
        loggingLevel: apigateway.MethodLoggingLevel.INFO,
      },
      handler: helloLambda,
      proxy: true,
    });
  }
}
