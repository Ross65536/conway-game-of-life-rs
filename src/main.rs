mod configuration;
mod grid;
mod command_line;

use configuration::Configuration;
use command_line::*;

fn main() {

    let args = parse_command_line(); 
    let config = Configuration::new(&args);
    
    let state = config.build_grid();
    print_state(&state);
}

