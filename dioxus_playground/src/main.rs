use dioxus::{logger::tracing, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut breed = use_signal(|| "Labrador".into());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // Hero {}
        input {
            value: breed.clone(),
            oninput: move |e| {
                breed.set(e.value().to_string());
            },
        }
        DogApp { breed: breed.clone() }
    }
}
#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: Signal<String>,
}

#[component]
fn DogApp(props: DogAppProps) -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    let skip = move |evt: MouseEvent| {
        tracing::info!("skip");
    };
    let save = move |evt: MouseEvent| {
        tracing::info!("save");
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}", id: "dog" }
        }
        div { id: "buttons",
            button { id: "skip", onclick: skip, "skip" }
            button { id: "save", onclick: save, "save!" }
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
