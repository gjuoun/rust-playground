use anyhow::{Context, Result};
use dioxus::{logger::tracing, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        DogApp{}
    }
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn DogApp() -> Element {
    // // State for the dog image URL
    // let mut img_src = use_signal(|| String::new());

    // // Event handler for fetching a new dog image
    // let fetch_new = move |_| async move {
    //     // Local async function for fetching dog images
    //     async fn fetch_dog_image() -> Result<String> {
    //         let resp = reqwest::get("https://dog.ceo/api/breeds/image/random")
    //             .await?
    //             .error_for_status()?
    //             .json::<DogApi>()
    //             .await
    //             .context("Failed to parse API response")?;

    //         // Return the image URL
    //         Ok(resp.message)
    //     }

    //     // Execute the fetch and handle the result
    //     match fetch_dog_image().await {
    //         Ok(image_url) => img_src.set(image_url),
    //         Err(e) => {
    //             tracing::error!("Error fetching dog image: {}", e);
    //             img_src.set("".into());
    //         }
    //     }
    // };

    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    // Event handlers for other buttons (currently just logging)
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
        button { id: "fetch", onclick: move |_| img_src.restart() , "fetch me!!!" }
        div { id: "buttons",
        button { id: "skip", onclick: skip, "skip" }
        button { id: "save", onclick: save, "save!" }
        // button { id: "fetch", onclick: fetch_new, "fetch!" }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
