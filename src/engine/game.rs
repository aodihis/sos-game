use crate::engine::bot::Bot;
use crate::engine::cell::CellValue;

pub struct  UpdateResponse {
    pub new_sos: Vec<(u16, u16, u16)>,
    pub scores: Vec<u16>,
    pub next_turn: u8
}
#[derive(Debug)]
pub enum GameError {
    InvalidPlayer,
    InvalidPosition,
    PositionAlreadyOccupied,
    InvalidMove,
    GameFinished,
}

#[derive(Clone)]
pub struct Game {
    pub num_of_players: u8,
    turn : u8,
    pub scores: Vec<u16>,
    _row: u16,
    pub col: u16,
    pub total: u16,
    pub cells: Vec<CellValue>,
    total_occupied: u16,
    sos: Vec<(u16, u16, u16)>
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
            total_occupied: 0,
            sos: vec![]
        }

    }

    pub fn update(&mut self, player:u8, pos:u16, value: CellValue) -> Result<UpdateResponse, GameError> {
        if self.total_occupied >= self.total {
            return Err(GameError::GameFinished);
        }
        if pos >= self.total {
            return Err(GameError::InvalidPosition);
        }
        if self.cells[pos as usize] != CellValue::Empty {
            return Err(GameError::PositionAlreadyOccupied);
        }
        if self.turn != player {
            return Err(GameError::InvalidPlayer);
        }

        if  player >= self.num_of_players {
            return Err(GameError::InvalidPlayer);
        }

        if value == CellValue::Empty {
            return Err(GameError::InvalidMove);
        }


        let ret = if value == CellValue::S {
            self.add_s(pos as i16)
        }else {
            self.add_o(pos as i16)
        };
        self.cells[pos as usize] = value;
        self.scores[player as usize] += ret.len() as u16;
        self.turn = (self.turn + 1) % self.num_of_players;
        self.sos.extend_from_slice(&ret);
        self.total_occupied += 1;
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
        self.get_sos_candidates(&mut ret, groups, pos, CellValue::S);
        ret
    }

    pub fn add_o(&self, pos:i16) -> Vec<(u16, u16, u16)> {
        let col = self.col as i16;
        let  groups= vec![
            ( pos - (col) - 1 , pos, pos + col + 1), ( pos - (col)  , pos, pos + col),  ( pos - (col) + 1 , pos, pos + col - 1),
            ( pos - 1 , pos, pos + 1),
        ];

        let mut ret : Vec<(u16,u16,u16)> = vec![];
        self.get_sos_candidates(&mut ret, groups, pos, CellValue::O);
        ret
    }

    pub fn bot_move(&mut self) -> Result<(u16, CellValue, Vec<(u16, u16, u16)>), GameError> {
        if self.is_game_over() {
            return Err(GameError::GameFinished);
        }
        let (pos, val) = Bot::make_move(self);
        let res = self.update(self.turn, pos, val)?;
        Ok((pos, val, res.new_sos))
    }

    pub fn get_scores(&self) -> Vec<u16> {
        self.scores.clone()
    }

    pub fn get_current_turn(&self) -> u8 {
        self.turn
    }

    pub fn is_game_over(&self) -> bool {
        self.total_occupied >= self.total
    }

    fn get_sos_candidates(&self, candidates: &mut Vec<(u16, u16, u16)>, groups: Vec<(i16, i16, i16)>, pos: i16, value: CellValue) {
        for &(i,j, k) in groups.iter() {
            if i < 0 || j < 0 || k < 0 {
                continue;
            }
            if i >= self.total as i16 || j >= self.total as i16 || k >= self.total as i16 {
                continue;
            }

            let irow = i as u16/ self.col;
            let krow = k as u16/ self.col;

            if !(krow - irow == 0 || krow - irow == 2) {
                continue;
            }

            let x = if i  == pos {
                value
            } else {
                self.cells[i as usize]
            };
            let y = if j  == pos {
                value
            } else {
                self.cells[j as usize]
            };
            let z = if k  == pos {
                value
            } else {
                self.cells[k as usize]
            };
            if x == CellValue::S && y == CellValue::O  && z == CellValue::S {
                candidates.push((i as u16, j as u16, k as u16));
            }
        }
    }
}