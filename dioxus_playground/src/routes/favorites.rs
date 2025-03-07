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
            // Show the error message if there is one
            {
                if let Some(error) = removal_error.get().as_ref() {
                    rsx! {
                        p { class: "error-message", "{error}" }
                    }
                }
            }
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
                                            button {
                                                class: "remove-button",
                                                onclick: move |_| {
                                                    to_owned![refresh_count, removal_error];
                                                    async move {
                                                        match remove_dog(dog.id).await {
                                                            Ok(_) => {
                                                                refresh_count.set(*refresh_count.get() + 1);
                                                                removal_error.set(None);
                                                            },
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

