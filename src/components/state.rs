use std::collections::HashMap;
use yew::Callback;
use crate::engine::cell::CellValue;

#[derive(Clone, PartialEq)]
pub enum BoardEvents {
    Idle,
    Lock,
    Unlock,
    Update(HashMap<u16, (u8, Option<CellValue>)>),
}
#[derive(Clone, PartialEq)]
pub struct LockingState {
    pub event: Callback<(u16, CellValue)>,
    pub last_locked_id: Option<u16>,
    pub last_locked_value: Option<CellValue>,
}

#[derive(Clone, PartialEq)]
pub struct BoardState {
    // pub callback: Callback<(BoardEvents)>,
    pub events: BoardEvents,
}
