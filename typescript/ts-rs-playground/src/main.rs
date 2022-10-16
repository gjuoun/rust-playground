use serde::{self, Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, TS, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    birth_date: Option<String>,
}

fn main() {
    println!("Jun's playground");
}
