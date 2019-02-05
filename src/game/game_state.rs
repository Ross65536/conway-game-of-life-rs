use game::cell::Cell;
use std::collections::HashSet;

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
        cell.neighbours_iter()
            .filter(|cell| self.cells.contains(cell))
            .count()
    }

    pub fn iterate(&self) -> GameState {
        let mut suspect_cells = self.cells.clone();
        self.cells.iter().for_each(|cell| {
            (*cell).neighbours_iter().for_each(|cell| {
                suspect_cells.insert(cell);
            })
        });

        let mut new_cells = HashSet::new();
        for cell in suspect_cells {
            let num_neighbours = self.count_neighbours(&cell);
            if num_neighbours >= 2 && num_neighbours <= 3 {
                if num_neighbours == 3 || self.cells.contains(&cell) {
                    new_cells.insert(cell);
                }
            }
        }

        GameState::new(new_cells)
    }
}

#[cfg(test)]
mod tests {

    use game::cell::Cell;
    use game::game_state::GameState;
    use std::collections::HashSet;

    fn build_cells(coordinates: &[(i64, i64)]) -> HashSet<Cell> {
        return coordinates.to_vec()
            .into_iter()
            .map(|(x,y)| Cell::new(x, y))
            .collect();
    }

    #[test]
    fn game_state_iterate_loop() {

        let initial: HashSet<Cell> = build_cells(&[(1, 0), (1, 1), (1, 2)]);
        let state = GameState::new(initial.clone());

        let next_state = state.iterate();
        let actual = next_state.cells();
        let next: HashSet<Cell> = build_cells(&[(0, 1), (1, 1), (2, 1)]);

        assert_eq!(next, actual);

        let actual = next_state.iterate().cells();

        assert_eq!(initial, actual);

    }

}
