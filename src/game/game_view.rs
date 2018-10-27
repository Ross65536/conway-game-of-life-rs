use game::game::BoardIter;
use std::collections::HashSet;
use game::cell::Cell;

pub struct GameView {
    x_size: usize,
    y_size: usize,
}

impl GameView {
    pub fn new(x_size: usize, y_size: usize) -> GameView {
        GameView { x_size: x_size, y_size: y_size }
    }

    pub fn build_sequential_iterator(&self, live_cells: HashSet<Cell>) -> BoardIter {
        let mut all_cells = Vec::new();

        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let cell = Cell::new(x as i64, y as i64);
                let has_cell = live_cells.contains(&cell);
                all_cells.push((x, y, has_cell));
            }
        }

        all_cells.into_iter()
    }
}