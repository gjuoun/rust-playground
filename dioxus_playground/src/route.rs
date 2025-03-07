use crate::components::navbar::NavBar;
use crate::routes::dog::DogView;
use crate::routes::favorites::FavoritesView;
use crate::routes::favorites_with_spspend::FavoritesViewWithSuspend;
use crate::routes::root::Root;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Root,

    #[route("/favorites")]
    FavoritesView,

    #[route("/favorites-with-suspend")]
    FavoritesViewWithSuspend,

    #[route("/dog")]
    DogView,
    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String> },
}
