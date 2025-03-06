use crate::components::navbar::NavBar;
use crate::routes::dog::DogView;
use crate::routes::favorites::FavoritesView;
use crate::routes::root::Root;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Root,

    #[route("/favorites")]
    FavoritesView,

    #[route("/dog")]
    DogView,
    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String> },
}
