use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    let mut favorites =
        use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    let handle_dog_removal2 = {
        move |dog_id: usize| async move {
            tracing::info!("id to remove: {}", dog_id);

            match remove_dog(dog_id).await {
                Ok(_) => {
                    favorites.restart();
                }
                Err(e) => {
                    tracing::error!("Failed to remove dog: {}", e);
                }
            }
        }
    };

    

    rsx! {
      div { class: "favorites-container",
        h3 { "Recently Saved Dogs" }
        {
            match favorites.read().as_ref() {
                None => rsx! {
                  p { "Loading saved dogs..." }
                },
                Some(_dogs) => {
                    if _dogs.is_empty() {
                        rsx! {
                          p { "No saved dogs yet." }
                        }
                    } else {
                        rsx! {
                          div { class: "dog-grid",
                            for _dog in _dogs.iter().cloned() {
                              div { class: "dog-card",
                                img { src: "{_dog.url}", alt: "Dog {_dog.id}" }
                                p { "Dog ID: {_dog.id}" }
                                button {
                                  class: "remove-button",
                                  onclick: move |_| {
                                      async move {
                                          let _ = handle_dog_removal2(_dog.id).await;
                                      }
                                  },
                                  "X"
                                }
                              }
                            }
                          }
                        }
                    }
                }
            }
        }
      }
    }
}
