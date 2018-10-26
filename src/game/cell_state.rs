#[derive(Clone, Copy, Debug)]
pub enum CellState {
    Empty,
    Filled
}

impl CellState {
    pub fn is_empty(&self) -> bool {
        match self {
            Empty => true,
            Filled => false
        }
    }
}