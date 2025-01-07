use crate::components::cell::{Cell, CellValue};
use crate::components::state::{BoardEvents, BoardState};
use std::rc::Rc;
use yew::{html, Component, Context, ContextProvider, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
    pub col: u32,
    pub row: u32,
}

pub enum BoardMsg {
    LockVal((u16, CellValue)),
}
pub struct Board {
    pub col: u32,
    pub row: u32,
    state:Rc<BoardState>

}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let state = Rc::new(BoardState{
            events: BoardEvents::Idle,
        });
        Self {
            state,
            col: ctx.props().col,
            row: ctx.props().row,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::LockVal((_id, _val)) => {
                // if let Some(window) = window() {
                //     format!("{}", id);
                //     let text = format!("{}, {}", id, match val {
                //         CellValue::S => "S",
                //         CellValue::O => "O",
                //         _ => "",
                //     });
                //     window.alert_with_message(&text).unwrap();
                // }
                Rc::make_mut(&mut self.state).events = BoardEvents::Lock;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.state.clone();
        let onselect = ctx.link().callback(BoardMsg::LockVal);
        let cells = (0..self.col*self.row).map(|i| html! { <Cell id={i as u16} onselect={onselect.clone()}/>  });
        let style = format!("grid-template-columns: repeat({}, 1fr);grid-template-rows: repeat({}, 1fr);", self.col, self.row);
        html! {
            <ContextProvider<Rc<BoardState>> context={state}>
                <div class="grid center" style={style}>
                    {for cells}
                </div>
             </ContextProvider<Rc<BoardState>>>
        }
    }
}