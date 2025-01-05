mod components;

use yew::{function_component, html, Html};
use crate::components::cell::Cell;

#[function_component]
fn App() -> Html {
    let row = 10;
    let column = 10;
    let width = 30*column;

    let cells = (0..column*row).map(|_| html! { <Cell /> });

    let style = format!("grid-template-columns: repeat({}, 1fr);grid-template-rows: repeat({}, 1fr);", column, row);
    let width_style = format!("width:{}px;", width+column-1);
    html! {
        <>
            <div class="container">
                <div class="center"  style={width_style}>
                    <h1 class="center header">{"SOS Game"}</h1>
                    <div class="scoreboard">
                        <span class="your-score">{ "Your Score: 0" }</span>
                        <span class="computer-score">{ "Computer Score: 0" }</span>
                    </div>
                    <div class="turn center">
                        {"Your turn."}
                    </div>
                    <div class="grid center" style={style}>
                        {for cells}
                    </div>
                </div>
            </div>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
