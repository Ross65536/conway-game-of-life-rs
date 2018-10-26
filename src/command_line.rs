use std::collections::HashMap;
use std::env;
use grid::GridState;
use grid::CellState;

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

pub fn print_state(gridState: &GridState) {
    let mut y_counter: usize = 0;
    gridState.for_each_sequential(|_, y, state| {
        if y_counter == y {
            println!();
            y_counter = y;
        }
        print!("{}", if state.is_empty() {"."} else {"X"})
    });
    println!();
}
