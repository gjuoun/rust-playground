use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
fn Button(props: &Props) -> Html {
    html! {
        <button disabled={props.is_loading}>{
            if props.is_loading {
                "Loading..."
            } else {
                "Click me!"
            }
         }</button>
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let is_loading = use_state(|| false);
    let onclick = {
        let counter = counter.clone();
        let is_loading = is_loading.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            is_loading.set(true);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p style="">{ *counter }</p>
            <Button is_loading={*is_loading} />

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
