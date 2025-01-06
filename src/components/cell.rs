use std::cmp::PartialEq;
use yew::{html, Component, Context, Html, MouseEvent};

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
    is_selecting: bool,
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
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_selecting: false,
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
            Msg::SetLock => {false},
            Msg::UpdateValue => {
                self.is_selecting = true;
                if self.value == CellValue::S {
                    self.value = CellValue::O;
                } else if  self.value == CellValue::O {
                    self.is_selecting = false;
                    self.value = CellValue::Empty;
                } else {
                    self.value = CellValue::S;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let on_select = ctx.link().callback(move |event: MouseEvent| {
            event.prevent_default();
            Msg::UpdateValue
        });
        let text_class = if self.is_selecting {"selecting"} else {""};
        html!{
            <div class="cell unselectable" oncontextmenu={on_select}>
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