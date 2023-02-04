mod router;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
<<<<<<< HEAD
=======

    let users_router = Router::new()
        .route(
            "/",
            get(router::users::get_all_users)
            .post(router::users::create_user),
        )
        .route("/:id", get(router::users::get_one_user));

>>>>>>> fecd5d98dbf5bd85d59a3eaf355788a1d1d1bde3
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
