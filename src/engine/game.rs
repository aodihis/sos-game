use gloo::console::info;
use wasm_bindgen::JsValue;
use crate::engine::cell::CellValue;

pub struct  UpdateResponse {
    pub new_sos: Vec<(u16, u16, u16)>,
    pub scores: Vec<u16>,
    pub next_turn: u8
}
#[derive(Debug)]
pub enum UpdateError {
    InvalidPlayer,
    InvalidPosition,
    PositionAlreadyOccupied,
    InvalidMove,
}

#[derive(Clone)]
pub struct Game {
    num_of_players: u8,
    turn : u8,
    scores: Vec<u16>,
    _row: u16,
    col: u16,
    total: u16,
    cells: Vec<CellValue>,
    _sos: Vec<(u16, u16, u16)>
}



impl Game {
    pub fn new(row:u16, col: u16, players: u8) -> Self {
        let total = row * col;

        Self {
            num_of_players: players,
            turn: 0,
            scores: vec![0;players as usize],
            _row : row,
            col,
            total,
            cells: vec![CellValue::Empty;total as usize],
            _sos: vec![]
        }

    }

    pub fn update(&mut self, player:u8, pos:u16, value: CellValue) -> Result<UpdateResponse, UpdateError> {
        if pos >= self.total {
            return Err(UpdateError::InvalidPosition);
        }
        if self.cells[pos as usize] != CellValue::Empty {
            return Err(UpdateError::PositionAlreadyOccupied);
        }
        if self.turn != player {
            return Err(UpdateError::InvalidPlayer);
        }

        if  player >= self.num_of_players {
            return Err(UpdateError::InvalidPlayer);
        }

        if value == CellValue::Empty {
            return Err(UpdateError::InvalidMove);
        }

        self.cells[pos as usize] = value;
        let ret = if value == CellValue::S {
            self.add_s(pos as i16)
        }else {
            self.add_o(pos as i16)
        };
        // let mut spk = vec![];
        // for val in &self.cells {
        //     match val {
        //         CellValue::Empty => {
        //             spk.push("".to_string());
        //         },
        //         CellValue::S =>  {
        //             spk.push("S".to_string());
        //         }
        //         CellValue::O => spk.push("O".to_string()),
        //     }
        // }
        // let vc = JsValue::from(spk);
        // info!(vc);
        self.scores[player as usize] += ret.len() as u16;
        self.turn = (self.turn + 1) % self.num_of_players;

        Ok(UpdateResponse {
            next_turn: self.turn,
            scores: self.scores.clone(),
            new_sos: ret,
        })
    }

    pub fn add_s(&self, pos:i16) -> Vec<(u16, u16, u16)> {
        let col = self.col as i16;
        let groups = vec![
            ( pos - (col*2) - 2 , pos-col - 1, pos), ( pos - (col*2)  , pos-col, pos),  ( pos - (col*2) + 2 , pos-col + 1, pos),
            ( pos - 2 , pos - 1, pos), (pos, pos+1, pos+2),
            (pos, pos + col -1, pos + (col*2) - 2),  (pos, pos + col, pos + (col*2) ),  (pos, pos + col + 1, pos + (col*2) + 2)
        ];

        let mut ret : Vec<(u16,u16,u16)> = vec![];
        self.get_sos_candidates(&mut ret, groups);
        ret
    }

    pub fn add_o(&self, pos:i16) -> Vec<(u16, u16, u16)> {
        let col = self.col as i16;
        let  groups= vec![
            ( pos - (col) - 1 , pos, pos + col + 1), ( pos - (col)  , pos, pos + col),  ( pos - (col) + 1 , pos, pos + col - 1),
            ( pos - 1 , pos, pos + 1),
        ];

        let mut ret : Vec<(u16,u16,u16)> = vec![];
        self.get_sos_candidates(&mut ret, groups);
        ret
    }

    fn get_sos_candidates(&self, candidates: &mut Vec<(u16, u16, u16)>, groups: Vec<(i16, i16, i16)>) {
        for &(i,j, k) in groups.iter() {
            // info!(i,j,k);
            if i < 0 || j < 0 || k < 0 {
                continue;
            }
            if i >= self.total as i16 || j >= self.total as i16 || k >= self.total as i16 {
                continue;
            }
            // info!("Closer", i, j, k);
            if self.cells[i as usize] == CellValue::S && self.cells[j as usize] == CellValue::O  && self.cells[k as usize] == CellValue::S {
                candidates.push((i as u16, j as u16, k as u16));
            }
        }
    }
}