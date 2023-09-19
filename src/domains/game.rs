use crate::domains::board::Board;
use crate::domains::player::Player;

pub struct Game {
    board: Board,
    first_player: Player,
    second_player: Player,
    turnCount: u32,
}

impl Game {
    pub fn new(first_player: Player, second_player: Player, board: Board, turnCount: u32) -> Game {
        Game {
            board,
            first_player,
            second_player,
            turnCount,
        }
    }

    pub fn get_first_player(&self) -> &Player {
        &self.first_player
    }

    pub fn get_second_player(&self) -> &Player {
        &self.second_player
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}
