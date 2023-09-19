use crate::{
    constants::board_default_size::BOARD_DEFAULT_SIZE,
    domains::{board::Board, cell_state::CellState},
};

pub struct BoardFactory {}

impl BoardFactory {
    pub fn new() -> BoardFactory {
        BoardFactory {}
    }

    pub fn create_reversi_board(&self) -> Result<Board, String> {
        let cells = self.init_cells(BOARD_DEFAULT_SIZE);
        let mut board = match Board::new(cells, BOARD_DEFAULT_SIZE) {
            Ok(board) => board,
            Err(err) => return Err(err),
        };
        Ok(board)
    }

    fn init_cells(&self, size: usize) -> Vec<Vec<CellState>> {
        let mut cells = self.init_cells_with_size(size);

        let half_size = size / 2;
        cells[half_size - 1][half_size - 1] = CellState::White;
        cells[half_size - 1][half_size] = CellState::Black;
        cells[half_size][half_size - 1] = CellState::Black;
        cells[half_size][half_size] = CellState::White;
        cells
    }

    fn init_cells_with_size(&self, size: usize) -> Vec<Vec<CellState>> {
        let mut cells = Vec::new();
        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                row.push(CellState::Empty);
            }
            cells.push(row);
        }
        cells
    }
}
