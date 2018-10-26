#[derive(Clone, Copy, Debug)]
enum CellState {
    Empty,
    Filled
}

#[derive(Debug)]
pub struct GridState {
    cells: Vec<Vec<CellState>>
}

impl GridState {
    pub fn new_empty(x_size: u32, y_size: u32) -> GridState {
        let mut empty_cells = Vec::new();
        (0..x_size).for_each(|_| empty_cells.push(vec![CellState::Empty; y_size as usize]));

        GridState {cells: empty_cells}
    }
}