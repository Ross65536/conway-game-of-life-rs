extern crate game_of_life;

use std::env;
use game_of_life::display::parse_command_line;
use game_of_life::configuration::Configuration;
use game_of_life::start_animation;
use game_of_life::display::print_grid_state;

fn main() {
    let args = parse_command_line(env::args());

    let config = Configuration::new(&args);
    start_animation(config, &print_grid_state);
}

