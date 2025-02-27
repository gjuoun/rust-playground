use anyhow::{Context, Result};
use dioxus::{logger::tracing, prelude::*};
use serde::Deserialize;

//region server function
#[server]
async fn save_dot(image: String) -> Result<(), ServerFnError> {
    reqwest::Client::new()
        .post("http://localhost:8080/api/save_dot")
        .json(&image)
        .send()
        .await?;

    Ok(())
}

//endregion server function

#[derive(Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogApp() -> Element {
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

    // Event handlers for buttons
    let skip = move |_| {
        tracing::info!("Skip button clicked");
    };

    let save = move |_| {
        tracing::info!("Save button clicked");
    };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default(), id: "dog" }
        }
        div { "Current time: {time_now}" }
        button { id: "fetch", onclick: move |_| img_src.restart(), "fetch me!!!" }
        div { id: "buttons",
            button { id: "skip", onclick: skip, "skip" }
            button { id: "save", onclick: save, "save!" }
        }
    }
}
