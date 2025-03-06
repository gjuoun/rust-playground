use crate::models::dog::Dog;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::config::db_server::DB;

// Server function to list dogs
#[server]
pub async fn list_favourite_dogs() -> Result<Vec<Dog>, ServerFnError> {
    let rows = DB.with(|conn| {
        conn.prepare("SELECT id, url FROM dogs ORDER BY id DESC")
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

// Server function to remove a dog by ID
#[server]
pub async fn remove_dog(id: usize) -> Result<(), ServerFnError<String>> {
    DB.with(|conn| {
        conn.execute("DELETE FROM dogs WHERE id = ?1", &[&id])
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
        Ok(())
    })
}
