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

    // Using use_hook for timestamp
    let time_now = use_hook(|| {
        // return current timestamp
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    });

    // Create a reusable async function
    async fn fetch_and_save(img_src: String, skip_save: bool) {
        let current = img_src.clone();

        if !skip_save {
            save_dog(current).await;
        }
    }

    // Use it in both handlers
    let skip = {
        // let saved_dogs = saved_dogs.clone();
        move |_| async move {
          // fix this function, it should refresh the image only, ai!
          let current = img_src.read().clone();
            img_src.restart();
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
      }
    }
}
