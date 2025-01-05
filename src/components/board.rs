use yew::{html, Component, Context, Html, Properties};
use crate::components::cell::Cell;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
    pub(crate) col: u32,
    pub(crate) row: u32,
}
pub struct Board {
    pub col: u32,
    pub row: u32,

}

impl Component for Board {
    type Message = ();
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            col: ctx.props().col,
            row: ctx.props().row,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let cells = (0..self.col*self.row).map(|_| html! { <Cell /> });

        let style = format!("grid-template-columns: repeat({}, 1fr);grid-template-rows: repeat({}, 1fr);", self.col, self.row);

        html! {
            <div class="grid center" style={style}>
                {for cells}
            </div>
        }
    }
}