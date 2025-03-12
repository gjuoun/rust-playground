use crate::components::navbar::NavBar;
use crate::dogs::dog_view::DogView;
use crate::dogs::favorites_view::FavoritesView;
use crate::dogs::favorites_with_spspend_view::FavoritesViewWithSuspend;
use crate::root::root_view::Root;
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
