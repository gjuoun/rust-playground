#![allow(non_snake_case)]

mod components;
mod config;
mod dogs;
mod root;
mod route;

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
    // getting PORT and HOST, print message before starting server
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    println!("Starting server at http://{}:{}", host, port);

    dioxus::launch(App);
}
