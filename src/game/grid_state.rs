use std::collections::HashSet;
use game::cell::Cell;


#[derive(Debug)]
pub struct GridState {
    x_size: usize,
    y_size: usize,
    cells: HashSet<Cell>,
}

impl GridState {
    pub fn new(x_size: usize, y_size: usize, cells: HashSet<Cell>) -> GridState {
        GridState { x_size: x_size, y_size: y_size, cells: cells }
    }

    fn empty_grid(x_size: usize, y_size: usize) -> Vec<Vec<bool>> {
        let mut empty_cells = Vec::new();
        (0..x_size).for_each(|_| empty_cells.push(vec![false; y_size]));

        empty_cells
    }

    pub fn for_each_sequential<F>(&self, mut consumer: F) where 
        F: FnMut(usize, usize, bool) {

        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let cell = Cell::new(x as i64, y as i64);
                let has_cell = self.cells.contains(&cell);
                consumer(x, y, has_cell)
            }
        }
    }
}