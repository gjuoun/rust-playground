use crate::models::dog::Dog;
use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    // Use a signal to track when to refresh
    let mut refresh_count = use_signal(|| 0);

    // Use resource with the refresh count as a dependency
    let dogs = use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    // Handler for removing a dog
    let remove_handler = move |id: usize| {
        let refresh = refresh_count.clone();
        async move {
            if let Err(e) = remove_dog(id).await {
                tracing::error!("Failed to remove dog: {}", e);
            }
            // Increment to trigger refresh
            refresh_count += 1;
        }
    };

    rsx! {
      div { class: "favorites-container",
        h2 { "Favorite Dogs" }
        {
            match dogs.clone().read_unchecked().as_ref() {
                Some(dog_list) => {
                    if dog_list.is_empty() {
                        rsx! {
                          p { "No dogs found in the database." }
                        }
                    } else {
                        rsx! {
                          div { class: "dog-grid",
                            for dog in dog_list {
                              div { class: "dog-card",
                                img { src: "{dog.url}", alt: "Dog {dog.id}" }
                                p { "Dog ID: {dog.id}" }
                                button { class: "remove-btn", onclick: move |_| remove_handler(dog.id), "Remove" }
                              }
                            }
                          }
                        }
                    }
                }
                None => rsx! {
                  p { "Loading dogs..." }
                },
            }
        }
        // Hidden element that forces re-render when refresh_count changes
        div { style: "display: none", "{refresh_count}" }
      }
    }
}
