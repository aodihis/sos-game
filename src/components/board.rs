use crate::components::state::{BoardEvents, BoardState};
use std::rc::Rc;
use gloo_timers::future::TimeoutFuture;
use yew::{html, Component, Context, ContextProvider, Html, Properties};
use yew::platform::spawn_local;
use crate::components::cell::Cell;
use crate::engine::cell::CellValue;
use crate::engine::game::Game;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
    pub col: u16,
    pub row: u16,
}

pub enum BoardMsg {
    Selecting((u16, CellValue)),
    UnlockCells,
    ProcessUpdate,
}
pub struct Board {
    pub col: u16,
    pub row: u16,
    state:Rc<BoardState>,
    game_engine: Game,
    turn: u8,

}

impl Component for Board {
    type Message = BoardMsg;
    type Properties = BoardProps;

    fn create(ctx: &Context<Self>) -> Self {
        let state = Rc::new(BoardState{
            events: BoardEvents::Idle,
        });
        Self {
            turn: 0,
            state,
            col: ctx.props().col,
            row: ctx.props().row,
            game_engine: Game::new(ctx.props().row, ctx.props().col,2)
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Selecting((id, val)) => {
                // if let Some(window) = window() {
                //     format!("{}", id);
                //     let text = format!("{}, {}", id, match val {
                //         CellValue::S => "S",
                //         CellValue::O => "O",
                //         _ => "",
                //     });
                //     window.alert_with_message(&text).unwrap();
                // }
                // let mut game_engine = self.game_engine.clone();
                // let link = ctx.link().clone();
                Rc::make_mut(&mut self.state).events = BoardEvents::Lock;
                ctx.link().send_message(BoardMsg::ProcessUpdate);
                // spawn_local(async move {
                //     game_engine.update(0, id, val).expect("Wrong move");
                //     link.send_message(BoardMsg::UnlockCells);
                // });
                true
            },
            BoardMsg::ProcessUpdate => {
                let link = ctx.link().clone();
                spawn_local(async move {
                    TimeoutFuture::new(1_000).await;
                    link.send_message(BoardMsg::UnlockCells);
                });
                true
            }
            BoardMsg::UnlockCells => {
                Rc::make_mut(&mut self.state).events = BoardEvents::Unlock;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.state.clone();
        let onselect = ctx.link().callback(BoardMsg::Selecting);
        let cells = (0..self.col*self.row).map(|i| html! { <Cell id={i} onselect={onselect.clone()}/>  });
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