use std::vec::IntoIter;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Cell {
    x: i64,
    y: i64,
}

impl<T> From<(T, T)> for Cell 
    where T: Into<i64> {
    fn from(tup: (T, T)) -> Self {
        Cell { x: tup.0.into(), y: tup.1.into() }
    }
}

impl Cell {
    pub fn new(x: i64, y: i64) -> Cell {
        Cell { x:x, y:y }
    }

    pub fn neighbours_iter(&self) -> IntoIter<Cell> {
        let neighbours = [
            (self.x + 1, self.y),
            (self.x + 1, self.y + 1),
            (self.x, self.y + 1),
            (self.x - 1, self.y + 1),
            (self.x - 1, self.y),
            (self.x - 1, self.y - 1),
            (self.x, self.y - 1),
            (self.x + 1, self.y - 1),
        ];

        neighbours.into_iter()
            .map(|p| Cell::from(*p))
            .collect::<Vec<Cell>>()
            .into_iter()
    }
}

