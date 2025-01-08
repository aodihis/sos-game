use crate::components::state::{BoardEvents, BoardState};
use std::cmp::PartialEq;
use std::rc::Rc;
use gloo::console::info;
use wasm_bindgen::JsValue;
use yew::{html, Callback, Component, Context, ContextHandle, Html, MouseEvent, Properties};
use crate::components::constants::{LINE_BOTTOM_CENTER, LINE_BOTTOM_LEFT, LINE_BOTTOM_RIGHT, LINE_CENTER_LEFT, LINE_CENTER_RIGHT, LINE_TOP_CENTER, LINE_TOP_LEFT, LINE_TOP_RIGHT};
use crate::engine::cell::CellValue;

pub enum Msg {
    UpdateValue,
    ContextChanged(Rc<BoardState>),

}


#[derive(Properties, PartialEq, Clone)]
pub struct CellProps {
    pub id: u16,
    pub onselect: Callback<(u16,CellValue)>,
}

pub struct Cell {
    id: u16,
    sys_lock: bool,
    state:Rc<BoardState>,
    _listener:ContextHandle<Rc<BoardState>>,
    // locking_state: Rc<LockingState>,
    is_lock: bool,
    value: CellValue,
    top_left: bool,
    top_center: bool,
    top_right: bool,
    center_left: bool,
    center_right: bool,
    bottom_left: bool,
    bottom_center: bool,
    bottom_right: bool,
}


impl Component for Cell {
    type Message = Msg;
    type Properties = CellProps;

    fn create(ctx: &Context<Self>) -> Self {

        let (state, _listener) = ctx.link()
            .context::<Rc<BoardState>>(ctx.link().callback(Msg::ContextChanged))
            .expect("Failed to update state");
        // let (locking_state, _) = ctx.link().context::<Rc<LockingState>>(ctx.link().callback(Msg::SetLock)).expect("failed to get locking state");
        Self {
            id: ctx.props().id,
            _listener,
            state,
            sys_lock: false,
            is_lock: false,
            value: CellValue::Empty,
            top_left: false,
            top_center: false,
            top_right: false,
            center_left: false,
            center_right: false,
            bottom_left: false,
            bottom_center: false,
            bottom_right: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ContextChanged(state) => {
                match &state.events {
                    BoardEvents::Lock =>  {
                        self.sys_lock = true;
                    },
                    BoardEvents::Unlock => {
                        self.sys_lock = false;
                    },
                    BoardEvents::Update(map) => {
                        if !map.contains_key(&self.id) && !self.is_lock {
                            self.value = CellValue::Empty;
                            return true;
                        }

                        if !map.contains_key(&self.id) {
                            return false;
                        }
                        let (pattern, val) = map[&self.id];
                        if let Some(val) = val {
                            self.value = val;
                            self.is_lock = true;
                        }

                        if pattern & LINE_TOP_LEFT == LINE_TOP_LEFT {
                            self.top_left = true;
                        }
                        if pattern & LINE_TOP_CENTER == LINE_TOP_CENTER {
                            self.top_center = true;
                        }
                        if pattern & LINE_TOP_RIGHT == LINE_TOP_RIGHT {
                            self.top_right = true;
                        }
                        if pattern & LINE_CENTER_LEFT == LINE_CENTER_LEFT {
                            self.center_left = true;
                        }
                        if pattern & LINE_CENTER_RIGHT == LINE_CENTER_RIGHT {
                            self.center_right = true;
                        }
                        if pattern & LINE_BOTTOM_LEFT == LINE_BOTTOM_LEFT {
                            self.bottom_left = true;
                        }
                        if pattern & LINE_BOTTOM_CENTER == LINE_BOTTOM_CENTER {
                            self.bottom_center = true;
                        }
                        if pattern & LINE_BOTTOM_RIGHT == LINE_BOTTOM_RIGHT {
                            self.bottom_right = true;
                        }

                    },
                    _ => {}
                }
                self.state = state;
                true
            },
            Msg::UpdateValue => {
                if self.sys_lock {
                    return false;
                }
                if self.is_lock {
                    return false;
                }
                if self.value == CellValue::S {
                    self.value = CellValue::O;
                } else if  self.value == CellValue::O {
                    self.value = CellValue::Empty;
                } else {
                    self.value = CellValue::S;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let on_choose = ctx.link().callback(move |event: MouseEvent| {
            event.prevent_default();
            Msg::UpdateValue
        });
        // let on_lock = ctx.link().callback(move |_| Msg::SetLock());
        let id = self.id;
        let selected = self.value.clone();
        let onselect = ctx.props().onselect.reform(move |_| {(id,selected)});
        let text_class = if self.is_lock {"cell-text"} else {"selecting cell-text"};
        html!{
            <div class="cell unselectable" oncontextmenu={on_choose} onclick={
                    if !self.sys_lock {
                       onselect
                    } else {
                        Callback::noop()
                    }
                }>
                <div class={text_class}>{
                    match self.value {
                        CellValue::S => html!{"S"},
                        CellValue::O => html!{"O"},
                        CellValue::Empty => html!{""}
                    }
                }</div>
                {self.top_left.then(|| html!{<div class="line top-left"></div>})}
                {self.top_center.then(|| html!{<div class="line top-center"></div>})}
                {self.top_right.then(|| html!{<div class="line top-right"></div>})}
                {self.center_left.then(|| html!{<div class="line center-left"></div>})}
                {self.center_right.then(|| html!{<div class="line center-right"></div>})}
                {self.bottom_left.then(|| html!{<div class="line bottom-left"></div>})}
                {self.bottom_center.then(|| html!{<div class="line bottom-center"></div>})}
                {self.bottom_right.then(|| html!{<div class="line bottom-right"></div>})}
            </div>
        }
    }
}