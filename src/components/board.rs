use yew::{Component, Context, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
    width: u32,
    height: u32,
}
pub struct Board {
    pub width: u32,
    pub height: u32,

}

impl Component for Board {
    type Message = ();
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            width: ctx.props().width,
            height: ctx.props().height,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}