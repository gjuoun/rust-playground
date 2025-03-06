use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::config::db_server::DB;

// Server function for favorites
#[server]
pub async fn list_favourite_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|conn| {
        conn.prepare("SELECT id, url FROM dogs ORDER BY id DESC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[component]
pub fn FavoritesView() -> Element {
    // Use the favorites-specific server function
    let dogs = use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    rsx! {
      div { class: "favorites-container",
        h2 { "Favorite Dogs" }
        if let Some(dog_list) = dogs.read().as_ref() {
          if dog_list.is_empty() {
            p { "No dogs found in the database." }
          } else {
            div { class: "dog-grid",
              for (id , url) in dog_list {
                div { class: "dog-card",
                  img { src: "{url}", alt: "Dog {id}" }
                  p { "Dog ID: {id}" }
                }
              }
            }
          }
        } else {
          p { "Loading dogs..." }
        }
      }
    }
}
