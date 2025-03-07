use crate::models::dog::Dog;
use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    let mut refresh_count = use_signal(|| 0);
    let mut removal_error = use_signal(|| None);
    let mut dogs = use_resource(|| async move { list_favourite_dogs().await.unwrap_or_default() });

    // New function to handle dog removal
    async fn handle_dog_removal(dog_id: usize, refresh_count: &UseSignal<i32>, removal_error: &UseSignal<Option<String>>) {
        match remove_dog(dog_id).await {
            Ok(_) => {
                refresh_count.set(*refresh_count.get() + 1);
            }
            Err(e) => {
                tracing::error!("Failed to remove dog: {}", e);
                removal_error.set(Some(format!("Failed to remove dog: {}", e)));
            }
        }
    }

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
                                                    to_owned![refresh_count, removal_error];
                                                    async move {
                                                        handle_dog_removal(_dog.id, &refresh_count, &removal_error).await;
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
