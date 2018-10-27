mod configuration;
mod game;
mod display;

use configuration::Configuration;
use game::engine::Engine;

fn main() {

    let args = display::parse_command_line();
    let config = Configuration::new(&args);
    
    let mut engine  = Engine::new(&display::print_grid_state, config);
    engine.game_loop();
}

