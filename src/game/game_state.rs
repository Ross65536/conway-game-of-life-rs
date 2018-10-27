use std::collections::HashSet;
use game::cell::Cell;


#[derive(Debug)]
pub struct GameState {
    cells: HashSet<Cell>,
}

impl GameState {
    pub fn new(cells: HashSet<Cell>) -> GameState {
        GameState { cells: cells }
    }

    pub fn cells(&self) -> HashSet<Cell> {
        self.cells.clone()
    }

    fn count_neighbours(&self, cell: &Cell) -> usize {
        cell.neighbours_iter().
            filter(|cell| self.cells.contains(cell))
            .count()
    }

    pub fn iterate(&self) -> GameState {
        let mut suspect_cells = self.cells.clone();
        self.cells.iter()
            .for_each(|cell| (*cell).neighbours_iter()
                    .for_each(|cell| { suspect_cells.insert(cell); })
            );

        let mut new_cells = HashSet::new();
        for cell in suspect_cells {
            let num_neighbours = self.count_neighbours(&cell);
            if  num_neighbours >= 2 && num_neighbours <= 3 {
                if num_neighbours == 3 || self.cells.contains(&cell) {
                    new_cells.insert(cell);
                }
            }
        }

        GameState::new(new_cells)
    }

    
}