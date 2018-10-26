mod cli;

use game::engine::ScreenUpdater;

use std::collections::HashMap;


pub fn get_configuration() -> HashMap<String, String> { 
    cli::parse_command_line()
}

pub fn get_display_updater() -> &'static ScreenUpdater { 
    &cli::print_grid_state
}