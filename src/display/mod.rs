use std::collections::HashMap;
use std::env;

use game::grid_state::GridState;

const LIVE_CELL: &'static str = "X";
const DEAD_CELL: &'static str = ".";

pub fn parse_command_line() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut key = String::new();
    for arg in env::args() {
        if arg.starts_with("-") && arg.len() > 1 {
            key = arg.splitn(2, "-").collect();
        } else {
            if !key.is_empty() {
                map.insert(key.clone(), arg.clone());
            } else {
                println!("Invalid argument: {}", arg);
            }

            key.clear();
        } 
    }

    map
}

fn print_grid(grid_state: &GridState) {
    let mut y_counter: usize = 0;
    grid_state.for_each_sequential(|_, y, has_cell| {
        if y_counter == y {
            println!();
            y_counter = y;
        }
        print!("{}", if has_cell {LIVE_CELL} else {DEAD_CELL})
    });
    println!();
}

fn clear_screen() { 
    print!("{}[2J", 27 as char);
} 

pub fn print_grid_state(grid_state: &GridState) {
    clear_screen();
    print_grid(grid_state);
}