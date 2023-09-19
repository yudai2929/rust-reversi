use crate::domains::cell_state::CellState;

pub struct Board {
    cells: Vec<Vec<CellState>>,
    size: usize,
}

impl Board {
    pub fn new(cells: Vec<Vec<CellState>>, size: usize) -> Result<Board, String> {
        if size % 2 != 0 {
            return Err(String::from("Board size must be even"));
        }
        Ok(Board { cells, size })
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &CellState {
        &self.cells[x][y]
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn set_cell(&mut self, x: usize, y: usize, state: CellState) {
        self.cells[x][y] = state;
    }

    pub fn count_black_stones(&self) -> usize {
        let count = self.count_cells(CellState::Black);
        count
    }

    pub fn count_white_stones(&self) -> usize {
        let count = self.count_cells(CellState::White);
        count
    }

    fn count_cells(&self, state: CellState) -> usize {
        let count = self
            .cells
            .iter()
            .flat_map(|row| row.iter()) // フラットなイテレータを生成
            .filter(|&&cell| cell == state) // 条件に合致する要素をフィルタリング
            .count(); // 要素数をカウント
        count
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellState::Empty => print!(" "),
                    CellState::Black => print!("⚫️"),
                    CellState::White => print!("⚪️"),
                }
            }
            println!();
        }
    }
}
