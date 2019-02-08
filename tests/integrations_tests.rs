extern crate game_of_life;

use game_of_life::start_animation;
use game_of_life::display::print_grid_state;
use game_of_life::display::ScreenUpdater;
use std::str;

fn split_command_arguments(line: &'static str) -> impl Iterator<Item = String> {
    line.split_whitespace()
        .map(|s| s.to_string())    
}

#[test]
fn test_add<'a>() {
    let args = split_command_arguments("-pattern1 2,2;2,1;2,3 -xSize 5 -ySize 5 -numIterations 3");
    
    let mut buf: Vec<u8> = Vec::new();
    {
        let grid = &mut print_grid_state;
        let updater = ScreenUpdater::new(&mut buf, grid);
        start_animation(args, updater);
    }
    
    let output = str::from_utf8(&mut buf).unwrap();
    print!("{}", output);
    assert_eq!(1, 1);
}