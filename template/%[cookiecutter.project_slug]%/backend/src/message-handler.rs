use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    tracing::info!("Received {} SQS messages", event.payload.records.len());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    backend::shared::lambda::init_logger();
    tracing::info!("Starting message handler");

    run(service_fn(function_handler)).await
}
