use crate::engine::cell::CellValue;

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
    row: u16,
    col: u16,
    total: u16,
    cells: Vec<CellValue>,
    sos: Vec<(u16, u16, u16)>
}



impl Game {
    pub fn new(row:u16, col: u16, players: u8) -> Self {
        let total = row * col;

        Self {
            num_of_players: players,
            turn: 0,
            scores: vec![0;players as usize],
            row,
            col,
            total,
            cells: vec![CellValue::Empty;total as usize],
            sos: vec![]
        }

    }

    pub fn update(&mut self, player:u8, pos:u16, value: CellValue) -> Result<(), UpdateError> {
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
        println!("{:?}", value);
        if value == CellValue::Empty {
            return Err(UpdateError::InvalidMove);
        }

        Ok(())
    }

    // pub fn check_sos(&self, pos:u16, value:CellValue) -> u16 {
    //     let mov = vec![];
    // }
}