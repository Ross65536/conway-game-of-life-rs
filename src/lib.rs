pub mod configuration;
pub mod game;
pub mod display;

use game::game::ScreenUpdater;
use configuration::Configuration;
use game::game::Game;
use std::io::stdout;
use display::parse_command_line;
use display::print_grid_state;
use std::env;

pub fn start_animation<'a, T>(args: T, update_screen: &'a ScreenUpdater) 
    where T: Iterator<Item = String> {
    let args = parse_command_line(args);
    let config = Configuration::new(&args);
    
    let mut game = Game::new(update_screen, config);
    game.game_loop();
}

pub fn start_console_animation() {
    let update = |grid| print_grid_state(&mut stdout(), grid);
    start_animation(env::args(), &update);
}
