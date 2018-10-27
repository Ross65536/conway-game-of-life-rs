use game::cell::Cell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::collections::HashMap;

const X_SIZE: &'static str = "20";
const Y_SIZE: &'static str = "20";
const FRAME_TIME_MILLIS: &'static str = "1000";

#[derive(Debug)]
pub struct Configuration {
    x_size: usize,
    y_size: usize,
    frametime_ms: u64,
} 

impl Configuration {
    pub fn new(user_config: &HashMap<String, String>) -> Configuration {
        let config = Configuration::build_configs(user_config);

        let x_size: usize = Configuration::parse_arg(&config, "xSize");
        let y_size: usize = Configuration::parse_arg(&config, "ySize");
        let frametime: u64 = Configuration::parse_arg(&config, "frameTime");

        Configuration { x_size: x_size, y_size: y_size, frametime_ms: frametime } 
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    pub fn get_frametime_ms(&self) -> u64 {
        self.frametime_ms
    }

    pub fn get_cells(&self) -> HashSet<Cell> {
        HashSet::new()
    }

    fn init_deafult_args() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("xSize", X_SIZE);
        map.insert("ySize", Y_SIZE);
        map.insert("frameTime", FRAME_TIME_MILLIS);

        map.iter().map(|p| ((*p.0).into(), (*p.1).into())).collect()
    } 

    fn build_configs(configuration: &HashMap<String, String>) -> HashMap<String, String> {
        let mut config = Configuration::init_deafult_args();
        configuration.into_iter().for_each(|(k, v)| { 
            config.insert((*k).clone(), (*v).clone()); 
        });

        config
    }

    fn parse_arg<T>(config: &HashMap<String, String>, arg: &str) -> T where
         T: FromStr, <T as FromStr>::Err: Debug {
         config.get(arg.into())
            .unwrap()
            .parse()
            .unwrap()
    }
}


