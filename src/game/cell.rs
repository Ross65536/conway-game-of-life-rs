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

impl<T> From<(T, T)> for Cell 
    where T: Into<i64> {
    fn from(tup: (T, T)) -> Self {
        Cell { x: tup.0.into(), y: tup.1.into() }
    }
}