use axum::{routing::get, Router};

use playground::router::{comments, hello, posts, users};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/hello", hello::routes())
        .nest("/posts", posts::posts::routes())
        .nest("/users", users::routes())
        .nest("/comments", comments::comments::routes());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
