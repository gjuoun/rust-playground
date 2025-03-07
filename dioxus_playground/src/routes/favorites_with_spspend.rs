use crate::routes::favorites_server::list_favourite_dogs;
use dioxus::prelude::*;

#[component]
pub fn FavoritesViewWithSuspend() -> Element {
    // suspend cause blocking of the page, not recommended for production
    let favorites = use_resource(list_favourite_dogs).suspend()?;

    rsx! {
        div { class: "favorites-container",
            h3 { "Recently Saved Dogs" }
            div { id: "favorites-container",
                div { id: "favorites",
                    for dog in favorites.read().iter().flatten() {
                        div { key: "{dog.id}", class: "dog-card",
                            img { src: "{dog.url}", alt: "Dog {dog.id}" }
                            p { "Dog ID: {dog.id}" }
                        }
                    }
                }

            }
        }
    }
}
