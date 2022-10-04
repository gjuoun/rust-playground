use axum::{
 routing,
 response::Json, extract::Path
};
use serde_json::{Value, json};
use serde:: {Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: u32,
    name: String,
}


/// /users/:id
pub async fn get_one_user(Path(id): Path<String>) -> Json<Value> {
    let users = vec![
        User { id: 1, name: "foo".to_string() }, 
        User { id: 2, name: "bar".to_string() }
    ];

    let user = users.iter().find(|&x| x.id.to_string() == id).expect("User not found");
    
    Json(json!(user))
}

/// /users/
pub async fn get_all_users() -> Json<Value>{
    let users = vec![
        User { id: 1, name: "foo".to_string() }, 
        User { id: 2, name: "bar".to_string() }
    ];
    
    Json(json!(users))
}

pub async fn create_user(Json(user): Json<User>) ->Json<Value> {

    Json(json!(user))

}