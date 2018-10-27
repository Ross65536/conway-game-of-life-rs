use configuration::Configuration;
use game::grid_state::GridState;

use std::time::Duration;
use std::thread;

pub type ScreenUpdater = Fn(&GridState);

pub struct Engine<'a> {
    grid: GridState,
    config: Configuration,
    screen_updater: &'a ScreenUpdater 
}

impl<'a> Engine<'a> {
    pub fn new(update_screen: &'a ScreenUpdater, config: Configuration) -> Engine<'a> {
        let (x_size, y_size) = config.get_size();
        let grid = GridState::new(x_size, y_size, config.get_cells());
        Engine { screen_updater: update_screen, grid: grid, config: config }
    }

   

    pub fn game_loop(&mut self) {
        let frametime_ms = self.config.get_frametime_ms();

        (self.screen_updater)(&self.grid);
        loop {
            self.grid = self.grid.iterate();
            (self.screen_updater)(&self.grid);
            thread::sleep(Duration::from_millis(frametime_ms));
        }
    }
}