use std::vec::IntoIter;
use game::game_view::GameView;
use configuration::Configuration;
use game::game_state::GameState;

use std::time::Duration;
use std::thread;

pub type BoardIter = IntoIter<(usize, usize, bool)>;
pub type ScreenUpdater = Fn(BoardIter);

pub struct Game<'a> {
    config: Configuration,
    game_state: GameState,
    game_view: GameView,
    screen_updater: &'a ScreenUpdater 
}

impl<'a> Game<'a> {
    pub fn new(update_screen: &'a ScreenUpdater, config: Configuration) -> Game<'a> {
        let game_state = GameState::new(config.get_cells());

        let (x_size, y_size) = config.get_size();
        let game_view = GameView::new(x_size, y_size);

        Game { 
            screen_updater: update_screen, 
            game_state: game_state, 
            game_view: game_view, 
            config: config 
        }
    }

    fn update_display(&self) {
        (self.screen_updater)(self.game_view
                .build_sequential_iterator(self.game_state.cells()));
    }

    pub fn game_loop(&mut self) {
        let frametime_ms = self.config.get_frametime_ms();

        self.update_display();
        loop {
            self.game_state = self.game_state.iterate();
            self.update_display();
            thread::sleep(Duration::from_millis(frametime_ms));
        }
    }
}