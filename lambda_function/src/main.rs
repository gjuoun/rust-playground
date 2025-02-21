use lambda_http::{run, service_fn, tracing, Error};
use std::env;
mod http_handler;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load .env file if APP_ENV is "local"
    if env::var("APP_ENV").map(|v| v == "local").unwrap_or(false) {
        dotenv::dotenv().ok();
        println!("Loaded .env file for local development");
    }

    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}
