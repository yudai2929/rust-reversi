mod domains;
use domains::{
    board::Board, factories::reversi_board_factory::BoardFactory, game::Game, player::Player,
    player_color::PlayerColor,
};

mod constants;
use constants::board_default_size::BOARD_DEFAULT_SIZE;

fn main() {
    let first_player = Player::new(String::from("Player 1"), PlayerColor::Black, false);
    let second_player = Player::new(String::from("Player 2"), PlayerColor::White, false);
    let board = match BoardFactory::new().create_reversi_board() {
        Ok(board) => board,
        Err(err) => return println!("{}", err),
    };
    let game = Game::new(first_player, second_player, board, 0);
}
