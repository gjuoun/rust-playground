use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    /// JSONB field
    pub geo: Geo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    #[serde(rename = "catchPhrase")]
    pub catch_phrase: String,
    pub bs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    /// JSONB field
    pub address: Address,
    pub phone: String,
    pub website: String,
    /// JSONB field
    pub company: Company,
    pub created_at: String,
    pub updated_at: String,
}
