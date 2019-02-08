pub mod configuration;
pub mod game;
pub mod display;

use configuration::Configuration;
use game::game::Game;
use std::io::stdout;
use display::parse_command_line;
use display::print_grid_state;
use display::ScreenUpdater;
use std::env;


pub fn start_animation<'a>(args: impl Iterator<Item = String>, update_screen: ScreenUpdater<'a>) {
    let args = parse_command_line(args);
    let config = Configuration::new(&args);
    
    let mut game = Game::new(update_screen, config);
    game.game_loop();
}

pub fn start_console_animation() {
    let out = &mut stdout();
    let grid = &mut print_grid_state;
    let updater = ScreenUpdater::new(out, grid);
    start_animation(env::args(), updater);
}
