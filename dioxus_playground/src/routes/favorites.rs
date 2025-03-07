use crate::models::dog::Dog;
use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    // Use a signal to track when to refresh
    let mut refresh_count = use_signal(|| 0);

    // Use resource with the refresh count as a dependency
    let mut dogs = use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    rsx! {
      div { class: "favorites-container",
        h3 { "Recently Saved Dogs" }
        {
            match dogs.read().as_ref() {
                Some(dogs) => {
                    if dogs.is_empty() {
                        rsx! {
                          p { "No saved dogs yet." }
                        }
                    } else {
                        rsx! {
                          div { class: "dog-grid",
                            for dog in dogs {
                              div { class: "dog-card",
                                img { src: "{dog.url}", alt: "Dog {dog.id}" }
                                p { "Dog ID: {dog.id}" }
                                  // please add a "X" button to remove the dog, ai!
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
