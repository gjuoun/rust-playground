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

pub fn create_user(user: User) -> User{
  user
}