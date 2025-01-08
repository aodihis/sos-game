use crate::components::cell::Cell;
use crate::components::constants::{COMPUTER_TURN, LINE_BOTTOM_CENTER, LINE_BOTTOM_LEFT, LINE_BOTTOM_RIGHT, LINE_CENTER_LEFT, LINE_CENTER_RIGHT, LINE_TOP_CENTER, LINE_TOP_LEFT, LINE_TOP_RIGHT, PLAYER_TURN};
use crate::components::state::{BoardEvents, BoardState};
use crate::engine::cell::CellValue;
use crate::engine::game::Game;
use gloo_timers::future::TimeoutFuture;
use std::collections::HashMap;
use std::rc::Rc;
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
    BotMove,
    ProcessUpdate(u16, CellValue),
    CheckGameOver,
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
            turn: PLAYER_TURN,
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
                if self.turn == COMPUTER_TURN {
                    return false;
                }
                if val == CellValue::Empty {
                    return false;
                }
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local( async move {
                    link.send_message(BoardMsg::LockCells);
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::ProcessUpdate(id, val));
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::BotMove);
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::CheckGameOver);
                    TimeoutFuture::new(100).await;
                    link.send_message(BoardMsg::UnlockCells);
                });

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
                        self.player_score = result.scores[0];
                        self.bot_score = result.scores[1];
                        self.turn = result.next_turn;
                        let mut map = HashMap::new();
                        map.insert(id, (0,Some(val)));
                        self.grouping_sos(&result.new_sos, &mut map);
                        Rc::make_mut(&mut self.state).events = BoardEvents::Update(map);
                        true
                    },
                    Err(_) => {
                        false
                    }
                }
            }
            BoardMsg::UnlockCells => {
                if self.game_engine.is_game_over() {
                    return false;
                }
                Rc::make_mut(&mut self.state).events = BoardEvents::Unlock;
                true
            },
            BoardMsg::BotMove => {
                if self.turn != COMPUTER_TURN || self.game_engine.is_game_over() {
                    return false;
                }
                let response = self.game_engine.bot_move();
                match response {
                    Ok((pos, val, sos)) => {
                        self.turn = self.game_engine.get_current_turn();
                        let mut map = HashMap::new();
                        map.insert(pos, (0,Some(val)));
                        self.grouping_sos(&sos, &mut map);
                        let scores = self.game_engine.get_scores();
                        self.player_score = scores[0];
                        self.bot_score = scores[1];
                        Rc::make_mut(&mut self.state).events = BoardEvents::Update(map);
                    }
                    Err(_e) => {}
                }
                true
            },
            BoardMsg::CheckGameOver => {
                if self.game_engine.is_game_over() {
                    Rc::make_mut(&mut self.state).events = BoardEvents::Lock;
                    return true
                }
                false

            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = self.state.clone();
        let onselect = ctx.link().callback(BoardMsg::Selecting);
        let cells = (0..self.col*self.row).map(|i| html! { <Cell id={i} onselect={onselect.clone()}/>  });
        let style = format!("grid-template-columns: repeat({}, 1fr);grid-template-rows: repeat({}, 1fr);", self.col, self.row);
        let message = if self.game_engine.is_game_over() {
            match self.player_score.cmp(&self.bot_score) {
                std::cmp::Ordering::Greater => html!(<p class="win-bar">{"You win!"}</p>),
                std::cmp::Ordering::Less => html!(<p class="lose-bar">{"You lose!"}</p>),
                std::cmp::Ordering::Equal => html!(<p class="draw-bar">{"Draw"}</p>),
            }
        } else if self.turn == 0 {
            html!(<p class="turn">{"Your turn"}</p>)
        } else {
            html!(<p class="turn">{"Computer turn"}</p>)
        };

        html! {
            <>
            <div class="scoreboard">
                <span class="your-score">{ "Your Score: " } {self.player_score}</span>
                <span class="computer-score">{ "Computer Score: "} {self.bot_score}</span>
            </div>
            <div class="turn center">
            { message }
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
    fn grouping_sos(&mut self, result: &Vec<(u16, u16, u16)>, map: &mut HashMap<u16, (u8, Option<CellValue>)>) {
        for (x,y,z) in result {
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