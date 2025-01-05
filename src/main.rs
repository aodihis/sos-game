use yew::{function_component, html, use_state, Html};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    html! {
        <div class="container">
            <button onclick={onclick}>{"+1"}</button>
            <p>{ *counter }</p>
        </div>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
