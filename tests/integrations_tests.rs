extern crate game_of_life;
extern crate regex;

use game_of_life::start_animation;
use game_of_life::display::print_grid_state;
use game_of_life::display::ScreenUpdater;
use regex::Regex;
use std::str;

fn split_command_arguments(line: &'static str) -> impl Iterator<Item = String> {
    line.split_whitespace()
        .map(|s| s.to_string())    
}

#[test]
fn test_simple<'a>() {
    let args = split_command_arguments("-pattern1 1,1;1,0;1,2 -xSize 3 -ySize 3 -numIterations 2 -frameTime 0");
    
    let mut buf: Vec<u8> = Vec::new();
    {
        let grid = &mut print_grid_state;
        let updater = ScreenUpdater::new(&mut buf, grid);
        start_animation(args, updater);
    }
    
    let output = str::from_utf8(&mut buf).unwrap();

    // blinker in regex form, full cycle
    let expr = r"(\.X\.\s*){3}\s*(\.){3}\s*(X){3}\s*(\.){3}\s*(\.X\.\s*){3}";
    let regex = Regex::new(expr)
            .unwrap();
    assert!(regex.is_match(output));
}