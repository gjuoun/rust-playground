use super::dog_server::save_dog;
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

    let skip = {
        move |_| {
            img_src.restart();
        }
    };

    let save = {
        move |_| async move {
            let current = img_src.cloned().unwrap_or_default();
            img_src.restart();
            let _ = save_dog(current).await;
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
          button { id: "skip", onclick: skip, "Skip" }
          button { id: "save", onclick: save, "Save" }
        }
      }
    }
}
