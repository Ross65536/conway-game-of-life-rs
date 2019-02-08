use display::ScreenUpdater;
use std::vec::IntoIter;
use game::game_view::GameView;
use configuration::Configuration;
use game::game_state::GameState;

use std::time::Duration;
use std::thread;

pub type BoardIter = IntoIter<(usize, usize, bool)>;

pub struct Game<'a> {
    frametime_ms: u64,
    game_state: GameState,
    game_view: GameView,
    screen_updater: ScreenUpdater<'a>,
    num_iterations: usize,
}

impl<'a> Game<'a> {
    pub fn new(update_screen: ScreenUpdater<'a>, config: Configuration) -> Game<'a> {
        let game_state = GameState::new(config.cells());

        let (x_size, y_size) = config.size();
        let game_view = GameView::new(x_size, y_size);

        Game { 
            screen_updater: update_screen, 
            game_state: game_state, 
            game_view: game_view, 
            frametime_ms: config.frametime_ms(),
            num_iterations: config.num_iterations(),
        }
    }

    fn update_display(&mut self) {
        self.screen_updater.print(self.game_view
                .build_sequential_iterator(self.game_state.cells()));
    }

    pub fn game_loop(&mut self) {

        self.update_display();
        for _ in 0..self.num_iterations {
            self.game_state = self.game_state.iterate();
            self.update_display();
            if self.frametime_ms > 0 {
                thread::sleep(Duration::from_millis(self.frametime_ms));
            }
        }
    }
}