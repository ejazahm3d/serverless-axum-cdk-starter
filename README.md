# Axum + AWS Lambda + CDK + API Gateway Template with OpenAPI Integration

This repository contains a template project for building a serverless API using Axum, AWS Lambda, AWS CDK, and API Gateway with OpenAPI integration. The template provides a starting point for building scalable, efficient, and easily deployable APIs on the AWS cloud platform.

## Features

1. Axum: Axum is a fast and efficient web framework for Rust, designed for building asynchronous APIs.
2. AWS Lambda: AWS Lambda allows you to run code without provisioning or managing servers, ensuring automatic scaling and high availability.
3. AWS CDK: AWS Cloud Development Kit simplifies the process of defining infrastructure as code using familiar programming languages.
4. API Gateway: AWS API Gateway enables you to create, publish, maintain, monitor, and secure APIs at any scale.
5. OpenAPI Integration: With OpenAPI specification, enabling automatic generation of documentation and client SDKs.


## Requirements

- Rust
- AWS CLI
- AWS CDK
- Cargo lambda


## Building and Deploying the Project

1. Clone the project and change your directory into it. Run the following command.

```bash
cargo lambda build --release
```

2. Change into cdk directory and run the following

```bash
cd cdk
npm install
npm run cdk deploy
```
This will deploy the app with all its infrastructure to aws and give you URL. 

The api docs with rapidoc will be available at `{url}/rapidoc/docs`

## For local Development
1. For local Development

```bash
export APP_APPLICATION__STAGE_NAME=""
cargo lambda watch
```

By default, the web server will run on `http://localhost:9000`. You can test the server by sending a GET request to `http://localhost:9000/api/health-check` using your preferred HTTP client.
You can go to `http://localhost:9000/rapidoc/docs` for OpenAPI docs.

