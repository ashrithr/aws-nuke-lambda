# aws-nuke-lambda

## Install Serverless Framework for Deployment

> NOTE: The native build of the Rust Lambda requires Docker to be running in the background.

```
sudo npm install --global npx
sudo npm install --global serverless
npm install --save-dev serverless-rust
npm install --save-dev serverless-iam-roles-per-function
```

## Deploy

Deploying the lambda is via serverless framework.

```
npx serverless deploy
```

Invoking the function remotely:

```
npx serverless invoke -f anl -d '{"BucketName":"awsnuke", "ObjectKey": "config/dev/sample.toml", "Region": "us-east-1", "LogLevel": 3}'
```

## Debug

Checking the logs

```
npx serverless logs -f anl
```
