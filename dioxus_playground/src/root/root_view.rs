use dioxus::prelude::*;

#[component]
pub fn Root() -> Element {
    rsx! {
        div { class: "root-container",
            h1 { "Welcome to Dog App" }
            p { "This is the home page of our dog application." }
            p {
                "Use the navigation links above to explore dogs or view your favorites."
            }
        }
    }
}
