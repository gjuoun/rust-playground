mod components;

use components::favorites::Favorites;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Root,

    // #[layout(NavBar)]
    #[route("/favorites")]
    Favorites,
    // #[route("/dog")]
    // DogView,
    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String> },
}

fn Root() -> Element {
    rsx! {
        div { "This is root" }
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::Root {},
                h1 { "Home " }
            }
            Link { to: Route::Favorites {},
                h1 { "Favorites" }
            }
        }
        Outlet::<Route> {}
    }
}

pub fn DogView() -> Element {
    rsx! {
        div {
            h1 { "Dog View" }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
