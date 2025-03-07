use crate::models::dog::Dog;
use dioxus::prelude::*;
use tokio::time::{sleep, Duration};

#[cfg(feature = "server")]
use crate::config::db_server::DB;

// Server function to list dogs
#[server]
pub async fn list_favourite_dogs() -> Result<Vec<Dog>, ServerFnError> {
    sleep(Duration::from_secs(2)).await;

    let dogs = DB.with(|conn| {
        conn.prepare("SELECT id, url FROM dogs ORDER BY id DESC")
           .unwrap()
           .query_map([], |row| {
                let id: usize = row.get(0)?;
                let url: String = row.get(1)?;
                Ok(Dog::new(id, url))
            })
           .unwrap()
           .collect::<Result<Vec<Dog>, _>>()
    })?;

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
