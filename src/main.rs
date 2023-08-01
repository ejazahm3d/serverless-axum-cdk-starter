use axum::Router;
use demo_lambda::{configuration::get_configuration, startup::make_app};
use lambda_http::{run, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = get_configuration().expect("Config parsing error");

    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new().nest(
        config.application.get_stage_name().as_str(),
        make_app(config),
    );

    run(app).await
}
