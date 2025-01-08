use crate::components::cell::Cell;
use crate::components::constants::{COMPUTER_TURN, LINE_BOTTOM_CENTER, LINE_BOTTOM_LEFT, LINE_BOTTOM_RIGHT, LINE_CENTER_LEFT, LINE_CENTER_RIGHT, LINE_TOP_CENTER, LINE_TOP_LEFT, LINE_TOP_RIGHT, PLAYER_TURN};
use crate::components::state::{BoardEvents, BoardState};
use crate::engine::cell::CellValue;
use crate::engine::game::{Game, UpdateResponse};
use std::collections::HashMap;
use std::rc::Rc;
use gloo::console::info;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsValue;
use yew::{html, Component, Context, ContextProvider, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
    pub col: u16,
    pub row: u16,
}

pub enum BoardMsg {
    Selecting((u16, CellValue)),
    LockCells,
    UnlockCells,
    ProcessUpdate(u16, CellValue),
}
pub struct Board {
    pub col: u16,
    pub row: u16,
    state:Rc<BoardState>,
    game_engine: Game,
    turn: u8,
    player_score: u16,
    bot_score: u16,

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
            game_engine: Game::new(ctx.props().row, ctx.props().col,2),
            player_score: 0,
            bot_score: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BoardMsg::Selecting((id, val)) => {
                // info!(
                //     match val {
                //         CellValue::S => "S",
                //         CellValue::O => "O",
                //         CellValue::Empty => "Empty",
                //     }
                // );
                if val == CellValue::Empty {
                    return false;
                }
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local( async move {
                    link.send_message(BoardMsg::LockCells);
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::ProcessUpdate(id, val));
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::UnlockCells);
                });

                // ctx.link().send_message(BoardMsg::LockCells);
                // ctx.link().send_message(BoardMsg::ProcessUpdate(id, val));
                // ctx.link().send_message(BoardMsg::UnlockCells);
                false
            },
            BoardMsg::LockCells => {
                Rc::make_mut(&mut self.state).events = BoardEvents::Lock;
                true
            }
            BoardMsg::ProcessUpdate(id, val) => {
                let res = self.game_engine.update(self.turn, id, val);
                match res {
                    Ok(result) => {
                        let mut map = HashMap::new();
                        map.insert(id, (0,Some(val)));
                        self.updating_class(&result, &mut map);
                        Rc::make_mut(&mut self.state).events = BoardEvents::Update(map);
                        true
                    },
                    Err(_) => {
                        false
                    }
                }
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
            <>
            <div class="scoreboard">
                <span class="your-score">{ "Your Score: " } {self.player_score}</span>
                <span class="computer-score">{ "Computer Score: "} {self.bot_score}</span>
            </div>
            <div class="turn center">
            {
                if self.turn == 0 {
                    html!("Your turn")
                }else {
                    html!("Enemy turn")
                }
            }
            </div>
            <ContextProvider<Rc<BoardState>> context={state}>
                <div class="grid center" style={style}>
                    {for cells}
                </div>
             </ContextProvider<Rc<BoardState>>>
            </>
        }
    }
}

impl Board {
    fn updating_class(&mut self, result: &UpdateResponse, map: &mut HashMap<u16, (u8, Option<CellValue>)>) {
        self.player_score = result.scores[0];
        self.bot_score = result.scores[1];
        self.turn = result.next_turn;
        for (x,y,z) in &result.new_sos {
            if !map.contains_key(&x) {
                map.insert(*x, (0, None));
            }
            if !map.contains_key(&y) {
                map.insert(*y, (0, None));
            }
            if !map.contains_key(&z) {
                map.insert(*z, (0, None));
            }

            let i = *x as i16;
            let j = *y as i16;
            let k = *z as i16;

            match (x, y, z) {
                (x,y,z) if (i + 2) + (self.col as i16 )* 2 == k => { //diagonal left to right
                    let (mut a, val) = map[&x];
                    a |= LINE_BOTTOM_RIGHT;
                    map.insert(*x, (a, val));

                    let (mut b,val) = map[&y];
                    b |= LINE_TOP_LEFT | LINE_BOTTOM_RIGHT;
                    map.insert(*y, (b, val));

                    let (mut c,val) = map[&z];
                    c |= LINE_TOP_LEFT;
                    map.insert(*z, (c, val));
                },
                (x,y,z) if i + (self.col as i16 ) * 2 == k => {  // vertical
                    let (mut a, val) = map[&x];
                    a |= LINE_BOTTOM_CENTER;
                    map.insert(*x, (a, val));

                    let (mut b,val) = map[&y];
                    b |= LINE_BOTTOM_CENTER | LINE_TOP_CENTER;
                    map.insert(*y, (b, val));

                    let (mut c,val) = map[&z];
                    c |= LINE_TOP_CENTER;
                    map.insert(*z, (c, val));
                },
                (x,y,z) if (i-2) + (self.col as i16 )* 2 == k => { //diagonal right to left
                    let (mut a, val) = map[&x];
                    a |= LINE_BOTTOM_LEFT;
                    map.insert(*x, (a, val));

                    let (mut b,val) = map[&y];
                    b |= LINE_BOTTOM_LEFT | LINE_TOP_RIGHT;
                    map.insert(*y, (b, val));

                    let (mut c,val) = map[&z];
                    c |= LINE_TOP_RIGHT;
                    map.insert(*z, (c, val));
                },
                (x,y,z) if (i+2) == k =>{ // horizontal
                    let (mut a, val) = map[&x];
                    a |= LINE_CENTER_RIGHT;
                    map.insert(*x, (a, val));

                    let (mut b,val) = map[&y];
                    b |= LINE_CENTER_RIGHT | LINE_CENTER_LEFT;
                    map.insert(*y, (b, val));

                    let (mut c,val) = map[&z];
                    c |= LINE_CENTER_LEFT;
                    map.insert(*z, (c, val));
                },
                _ => {
                }
            }

        }
    }
}