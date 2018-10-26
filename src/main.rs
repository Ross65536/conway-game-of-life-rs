mod configuration;
mod game;
mod command_line;

use configuration::Configuration;
use command_line::*;
use game::engine::Engine;


fn main() {

    let args = parse_command_line(); 
    let config = Configuration::new(&args);
    
    let mut engine  = Engine::new(&print_grid_state, config);
    engine.game_loop();
}

