use crate::routes::favorites_server::list_favourite_dogs;
use dioxus::prelude::*;

#[component]
pub fn FavoritesViewWithSuspend() -> Element {
    let favorites = use_resource(list_favourite_dogs).suspend()?;

    rsx! {
        div { class: "favorites-container",
            h3 { "Recently Saved Dogs" }
            div { id: "favorites-container",
                let dogs = favorites().unwrap(); // dogs is a Vec<Dog>, and it's always available, ai!
                if dogs.is_empty() {
                    rsx! {
                        p { "No favorite dogs found." }
                    }
                } else {
                    rsx! {
                        div { id: "favorites",
                            for dog in dogs {
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
    }
}
