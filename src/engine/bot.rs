use crate::engine::cell::CellValue;
use crate::engine::game::Game;
use rand::Rng;

pub struct Bot {

}

impl Bot {
    pub fn make_move(game: &mut Game) -> (u16, CellValue) {
        let mut candidates: Vec<(u16, CellValue)> = vec![];
        let mut highest = 0;
        for i in 0..game.total {
            let i_usize = i as usize;
            if game.cells[i_usize] != CellValue::Empty {
                continue;
            }
            let s = game.add_s(i as i16);
            let o = game.add_o(i as i16);
            let slen = s.len() as u16;
            let olen = o.len() as u16;
            for (len, value) in [(slen, CellValue::S), (olen, CellValue::O)] {
                if len > highest {
                    highest = len;
                    candidates = vec![(i, value)];
                } else if len == highest {
                    candidates.push((i,value));
                }
            }
        }

        if highest == 0 {
            candidates =  Bot::strategic_moves(&candidates, game);

        }

        let mut rng = rand::thread_rng();
        candidates[rng.gen_range(0..candidates.len())]
    }

    fn strategic_moves(moves: &Vec<(u16, CellValue)>, game: &Game) -> Vec<(u16, CellValue)> {
        let mut candidates: Vec<(u16, CellValue)> = vec![];
        for i in 0..moves.len() {
            let (pos, val) = moves[i];
            if val == CellValue::O && Bot::is_defensive_move(game, i as i16) {
                candidates.push((pos, CellValue::O));
            }
        }
        if candidates.is_empty() {
            return moves.clone();
        }
        candidates
    }

    fn is_defensive_move(game: &Game, pos: i16) -> bool {
        let patterns = [
            (pos - (game.col as i16) - 1, pos + (game.col as i16) + 1), (pos - (game.col as i16) + 1, pos + (game.col as i16) - 1),
            (pos - (game.col as i16) , pos + (game.col as i16)), (pos - 1, pos + 1),
        ];

        for (x,z) in patterns {
            if x < 0 || z < 0 || x >= game.total as i16 || z >= game.total as i16 {
                continue;
            }

            let irow = x as u16/ game.col;
            let krow = z as u16/ game.col;

            if !(krow - irow == 0 || krow - irow == 2) {
                continue;
            }

            if game.cells[x as usize] == CellValue::S && game.cells[z as usize] == CellValue::S {
                return true;
            }
        }
        false
    }
}
