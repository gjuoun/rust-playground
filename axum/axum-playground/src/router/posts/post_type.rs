use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub user_id: u32,
}
