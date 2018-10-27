#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Cell {
    x: i64,
    y: i64,
}

impl Cell {
    pub fn new(x: i64, y: i64) -> Cell {
        Cell { x:x, y:y }
    }
}