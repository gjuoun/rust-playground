mod components;
mod config;
mod models;
mod route;
mod routes;

use dioxus::prelude::*;
use route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: "/path/to/your-script.js", r#async: true }
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
