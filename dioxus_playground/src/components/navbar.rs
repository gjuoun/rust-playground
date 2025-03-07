use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
      nav { class: "navbar",
        div { class: "navbar-container",
          div { class: "navbar-logo",
            Link { to: Route::Root {},
              h1 { "Dog App" }
            }
          }
          div { class: "navbar-links",

            Link { to: Route::DogView {},
              div { class: "nav-item", "Dogs" }
            }
            Link { to: Route::FavoritesView {},
              div { class: "nav-item", "Favorites" }
            }
            Link { to: Route::FavoritesViewWithSuspend {},
              div { class: "nav-item", "FavoritesWithSuspend" }
            }
          }
        }
      }
      Outlet::<Route> {}
    }
}
