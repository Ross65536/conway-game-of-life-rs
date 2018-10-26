#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Filled
}

#[derive(Debug)]
pub struct GridState {
    x_size: usize,
    y_size: usize,
    cells: Vec<Vec<CellState>>
}

impl GridState {
    pub fn new_empty(x_size: usize, y_size: usize) -> GridState {
        let mut empty_cells = Vec::new();
        (0..x_size).for_each(|_| empty_cells.push(vec![CellState::Empty; y_size]));

        GridState { 
            x_size: x_size,
            y_size: y_size,
            cells: empty_cells
        }
    }

    pub fn for_each_sequential<F>(&self, consumer: F) where 
        F: Fn(usize, usize, CellState) {
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                consumer(x, y, self.cells[x][y])
            }
        }
    }
}