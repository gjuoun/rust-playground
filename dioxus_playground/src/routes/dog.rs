use crate::models::dog::Dog;
use crate::routes::dog_server::{list_dogs, save_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    // State for the dog image URL using use_resource
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    // Resource for fetching saved dogs
    let mut saved_dogs = use_resource(|| async move { list_dogs().await.unwrap_or_default() });

    // Using use_hook for timestamp
    let time_now = use_hook(|| {
        // return current timestamp
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });

    // Create a reusable async function
    async fn fetch_and_save(
        img_src: &mut Resource<String>,
        saved_dogs: &mut Resource<Vec<Dog>>,
        skip_save: bool,
    ) {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        if !skip_save {
            if let Err(e) = save_dog(current).await {
                tracing::error!("Failed to save dog: {}", e);
            } else {
                // Refresh the dog list
                saved_dogs.restart();
            }
        }
    }

    // Use it in both handlers
    let skip = {
        // let saved_dogs = saved_dogs.clone();
        move |_| async move {
            fetch_and_save(&mut img_src, &mut saved_dogs, true).await;
        }
    };

    let save = {
        // let saved_dogs = saved_dogs.clone();
        move |_| async move {
            fetch_and_save(&mut img_src, &mut saved_dogs, false).await;
        }
    };

    rsx! {
      div { class: "dog-container",
        h2 { "Dog View" }
        div { id: "dogview",
          img { src: img_src.cloned().unwrap_or_default(), id: "dog" }
        }
        div { "Current time: {time_now}" }
        div { class: "button-group",
          button { id: "fetch", onclick: move |_| img_src.restart(), "Fetch New Dog" }
          button { id: "skip", onclick: skip, "Skip" }
          button { id: "save", onclick: save, "Save" }
        }

        h3 { "Recently Saved Dogs" }
        {
            match saved_dogs.read().as_ref() {
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
