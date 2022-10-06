use axum::{
    extract::Path,
    response::Json,
    routing::{self, get},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod users_service;
use users_service::User;


pub fn user_router() -> Router {
    Router::new()
        .route("/", get(get_all_users).post(create_user))
        .route("/:id", get(get_one_user))
}

/// GET /users/:id
<<<<<<< HEAD
pub async fn get_one_user(Path(id): Path<String>) -> Json<Value> {
=======
pub async fn get_one_user(Path(id): Path<String>) -> Json<Vec<User>> {
>>>>>>> fecd5d98dbf5bd85d59a3eaf355788a1d1d1bde3
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

<<<<<<< HEAD
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
=======
    let user = users.iter().find(|&x| x.id.to_string() == id).expect("User not found");
    
    // Json(json!(user))
    Json(users)
}

/// /users/
pub async fn get_all_users() -> Json<Vec<User>>{
    let users = vec![
        User { id: 1, name: "foo".to_owned() }, 
        User { id: 2, name: "bar".into() }
    ];
    
    Json(users)
}

pub async fn create_user(Json(user): Json<User>) ->Json<User> {
    let user = users_service::create_user(user);
    Json(user)
}
>>>>>>> fecd5d98dbf5bd85d59a3eaf355788a1d1d1bde3
