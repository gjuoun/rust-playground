mod router;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .nest("/users", router::users::user_router())
        .route("/health", get(|| async { Html("ok") }));

    println!("Listening on http://localhost:3000");
    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
