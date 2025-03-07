use crate::models::dog::Dog;
use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    // Use a signal to track when to refresh
    let mut refresh_count = use_signal(|| 0);
    // Signal to hold the error message if removal fails
    let mut removal_error = use_signal(|| None);

    // Use resource with the refresh count as a dependency
    let mut dogs = use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    rsx! {
      div { class: "favorites-container",
        h3 { "Recently Saved Dogs" }
        {
            match dogs.read().as_ref() {
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
                                    // move this to a separate function in the component, ai!
                                      to_owned![refresh_count, removal_error];
                                      async move {
                                          match remove_dog(_dog.id).await {
                                              Ok(_) =>{
                                                refresh_count += 1;
                                                dogs.restart();
                                              }
                                              Err(e) => {
                                                  tracing::error!("Failed to remove dog: {}", e);
                                                  removal_error.set(Some(format!("Failed to remove dog: {}", e)));
                                              }
                                          }
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
                None => rsx! {
                  p { "Loading saved dogs..." }
                },
            }
        }
      }
    }
}
