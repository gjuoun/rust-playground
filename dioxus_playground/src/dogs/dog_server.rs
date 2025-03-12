use super::dog_model::Dog;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::config::db_server::DB;

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|connection| connection.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<Dog>, ServerFnError> {
    let rows = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect::<Vec<(usize, String)>>()
    });

    // Convert database rows to Dog models
    let dogs = rows.into_iter().map(Dog::from_row).collect();

    Ok(dogs)
}
