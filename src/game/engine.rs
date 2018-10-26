use configuration::Configuration;
use game::grid_state::GridState;

use std::time::Duration;
use std::thread;

pub type ScreenUpdater = Fn(&GridState);

pub struct Engine<'a> {
    grid: GridState,
    screen_updater: &'a ScreenUpdater 
}

impl<'a> Engine<'a> {
    pub fn new(update_screen: &'a ScreenUpdater, config: &Configuration) -> Engine<'a> {
        let (x_size, y_size) = config.get_size();
        let grid = GridState::new_empty(x_size, y_size);
        Engine { screen_updater: update_screen, grid: grid }
    }

    fn iteration(&mut self) {
        
    }

    pub fn game_loop(&mut self) {
        loop {
            self.iteration();
            (self.screen_updater)(&self.grid);
            thread::sleep(Duration::from_millis(1000));
        }
    }
}