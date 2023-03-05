use axum::{routing::get, Router};

use playground::router::{comments, hello, posts, users};
use playground::router::{comments::comments, hello, posts::posts, users};
use tower::ServiceBuilder;
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

// Setup tracing

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/hello", hello::routes())
        .nest("/posts", posts::posts::routes())
        .nest("/users", users::routes())
        .nest("/comments", comments::comments::routes());

        .nest("/posts", posts::routes())
        .nest("/users", users::routes())
        .nest("/comments", comments::routes())
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new()
                            .level(Level::INFO)
                            .include_headers(false),
                    )
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Millis),
                    ),
            ),
        );
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
