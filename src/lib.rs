pub mod configuration;
pub mod game;
pub mod display;

use game::game::ScreenUpdater;
use configuration::Configuration;
use game::game::Game;


pub fn start_animation<'a>(config: Configuration, update_screen: &'a ScreenUpdater) {
    let mut game = Game::new(update_screen, config);
    game.game_loop();
}