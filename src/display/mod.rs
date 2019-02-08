use std::collections::HashMap;
use game::game::BoardIter;
use std::io::Write;

const LIVE_CELL: &'static str = "X";
const DEAD_CELL: &'static str = ".";

pub struct ScreenUpdater<'a> {
    out: &'a mut Write,
    updater: &'a mut FnMut(&mut Write, BoardIter),
}

impl<'a> ScreenUpdater<'a> {
    pub fn new(out: &'a mut Write, updater: &'a mut FnMut(&mut Write, BoardIter)) -> ScreenUpdater<'a> {
        ScreenUpdater{ out: out, updater: updater }
    }

    pub fn print(&mut self, iter: BoardIter) {
        (self.updater)(&mut self.out, iter);
    }

}

pub fn parse_command_line(args: impl Iterator<Item = String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut key = String::new();
    for arg in args {
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

pub fn print_grid_state<'a>(out: &'a mut Write, grid_state: BoardIter) {
    clear_screen();
    print_grid(out, grid_state);
}

fn print_grid<'a>(out: &'a mut Write, producer: BoardIter) {
    let mut y_counter: usize = 1;
    for (_, y, has_cell) in producer {
        if y_counter == y {
            println!();
            y_counter += 1;
        }
        write!(out, "{}", if has_cell {LIVE_CELL} else {DEAD_CELL}).unwrap();
    }
    writeln!(out).unwrap();
}

fn clear_screen() { 
    print!("{}[2J", 27 as char);
} 

