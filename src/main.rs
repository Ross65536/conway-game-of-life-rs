mod configuration;
mod game;
mod display;

use configuration::Configuration;
use game::engine::Engine;

fn main() {

    let args = display::get_configuration();
    let config = Configuration::new(&args);
    
    let mut engine  = Engine::new(display::get_display_updater(), config);
    engine.game_loop();
}

