mod components;
mod engine;

use yew::{function_component, html, Html};
use crate::components::board::Board;

#[function_component]
fn App() -> Html {
    let row = 10;
    let column = 10;
    let width = 50*column;

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
                    <Board row={row} col={column}/>
                </div>
            </div>
        </>
    }
}


fn main() {

    yew::Renderer::<App>::new().render();
}
