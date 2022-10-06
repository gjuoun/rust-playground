use axum::{
 routing,
 response::Json, extract::Path
};
use serde_json::{Value, json};
use serde:: {Serialize, Deserialize};

mod users_service;
use users_service::User;



/// GET /users/:id
pub async fn get_one_user(Path(id): Path<String>) -> Json<Vec<User>> {
    let users = vec![
        User { id: 1, name: "foo".to_string() }, 
        User { id: 2, name: "bar".to_string() }
    ];

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