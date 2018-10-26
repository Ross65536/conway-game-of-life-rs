mod configuration;
mod grid;
mod command_line;

use configuration::Configuration;

fn main() {

    let args = command_line::parse_command_line(); 
    let config = Configuration::new(&args);
    
    let state = config.build_grid();
    println!("{:?}", config);
    println!("{:?}", state);
}

