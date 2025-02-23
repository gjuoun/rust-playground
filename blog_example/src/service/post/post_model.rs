use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(default)]
    pub body: Option<String>,
}
