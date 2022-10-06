use axum::{
    extract::Path,
    response::Json,
    routing::{self, get},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: u32,
    name: String,
}

pub fn user_router() -> Router {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/:id", get(get_one_user))
}

/// GET /users/:id
pub async fn get_one_user(Path(id): Path<String>) -> Json<Value> {
    let users = vec![
        User {
            id: 1,
            name: "foo".to_string(),
        },
        User {
            id: 2,
            name: "bar".to_string(),
        },
    ];

    let user = users
        .iter()
        .find(|&x| x.id.to_string() == id)
        .expect("User not found");

    Json(json!(user))
}

/// GET /users/
pub async fn get_all_users() -> Json<Value> {
    let users = vec![
        User {
            id: 1,
            name: "foo".to_string(),
        },
        User {
            id: 2,
            name: "bar".to_string(),
        },
    ];

    Json(json!(users))
}

/// POST /users/
pub async fn create_user(Json(user): Json<User>) -> Json<Value> {
    Json(json!(user))
}
