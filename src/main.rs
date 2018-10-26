mod configuration;
mod game;
mod command_line;

use configuration::Configuration;
use command_line::*;

use std::time::Duration;
use std::thread;

fn main() {

    let args = parse_command_line(); 
    let config = Configuration::new(&args);
    
    let state = config.build_grid();
    for i in 0..4 {
        print_grid_state(&state);
        println!("{}", i);
        thread::sleep(Duration::from_millis(1000));
    }
}

