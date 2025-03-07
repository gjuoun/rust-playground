use crate::routes::favorites_server::{list_favourite_dogs, remove_dog};
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn FavoritesView() -> Element {
    let mut favorites = use_resource(list_favourite_dogs).suspend()?;

    rsx! {
      div { class: "favorites-container",
        h3 { "Recently Saved Dogs" }
        div {
          id: "favorites-container",
          // here dog is Vec<Dog>, how to fix it? ai?
          
          for dog in favorites().ok(){
            // let { id, url } = dog;

            div {
              key: dog.id,
              class: "dog-card",
              img { src: dog.url, alt: "Dog {dog.id}" }
            }
          }
        }
      }
    }
}
