use crate::routes::dog_server::save_dog;
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
    async fn fetch_and_save(img_src: &mut Resource<String>, skip_save: bool) {
        let current = img_src.cloned().unwrap();
        img_src.restart();
        if !skip_save {
            _ = save_dog(current).await;
        }
    }

    // Use it in both handlers
    let skip = move |_| async move {
        fetch_and_save(&mut img_src, true).await;
    };

    let save = move |_| async move {
        fetch_and_save(&mut img_src, false).await;
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
