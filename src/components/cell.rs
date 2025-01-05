use std::cmp::PartialEq;
use yew::{html, Component, Context, Html};

pub enum Msg {
    SetLock,
    UpdateValue,
}
#[derive(PartialEq)]
enum CellValue {
    Empty,
    S,
    O,
}

pub struct Cell {
    id: u32,
    value: CellValue,
}


impl Component for Cell {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: 0,
            value: CellValue::Empty,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLock => {false},
            Msg::UpdateValue => {
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

        let onclick = ctx.link().callback(|_| Msg::UpdateValue);
        html!{
            <div class="cell unselectable" onclick={onclick}>
                {
                    match self.value {
                        CellValue::S => html!{"S"},
                        CellValue::O => html!{"O"},
                        CellValue::Empty => html!{""}
                    }
                }
            </div>
        }
    }
}