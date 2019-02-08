use game::cell::Cell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use std::collections::HashMap;

const X_SIZE: &'static str = "15";
const Y_SIZE: &'static str = "10";
const FRAME_TIME_MILLIS: &'static str = "1000";
const PATTERN: &'static str = "5,5;6,5;7,5";
const NUM_ITERATIONS: &'static str = "5";

#[derive(Debug)]
pub struct Configuration {
    x_size: usize,
    y_size: usize,
    frametime_ms: u64,
    num_iterations: usize,
    cells: HashSet<Cell>
} 

impl Configuration {
    pub fn new(user_config: &HashMap<String, String>) -> Configuration {
        let config = Configuration::build_configs(user_config);

        let x_size: usize = Configuration::parse_arg(&config, "xSize");
        let y_size: usize = Configuration::parse_arg(&config, "ySize");
        let frametime: u64 = Configuration::parse_arg(&config, "frameTime");
        let mut num_iterations: usize = Configuration::parse_arg(&config, "numIterations");
        if num_iterations == 0 {
            num_iterations = std::usize::MAX;
        }

        let cells = config.get("pattern".into())
            .unwrap()
            .split(";")
            .filter(|s| ! s.is_empty())
            .map( |point| {
                let pair: Vec<i64> = String::from(point)
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect();
                if pair.len() != 2 {
                    panic!("Invalid points parameter");
                }
                Cell::from((pair[0], pair[1]))
            }).collect();

        Configuration { x_size: x_size, y_size: y_size, frametime_ms: frametime, num_iterations: num_iterations, cells: cells } 
    }

    pub fn size(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    pub fn frametime_ms(&self) -> u64 {
        self.frametime_ms
    }

    pub fn cells(&self) -> HashSet<Cell> {
        self.cells.clone()
    }

    pub fn num_iterations(&self) -> usize {
        self.num_iterations
    }

    fn init_deafult_args() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("xSize", X_SIZE);
        map.insert("ySize", Y_SIZE);
        map.insert("frameTime", FRAME_TIME_MILLIS);
        map.insert("pattern", PATTERN);
        map.insert("numIterations", NUM_ITERATIONS);
        map.iter().map(|p| ((*p.0).into(), (*p.1).into())).collect()
    } 

    fn build_configs(configuration: &HashMap<String, String>) -> HashMap<String, String> {
        let mut config = Configuration::init_deafult_args();
        configuration.into_iter().for_each(|(k, v)| { 
            config.insert((*k).clone(), (*v).clone()); 
        });

        let pattern = configuration
            .keys()
            .filter(|k| (*k).starts_with("pattern"))
            .map(|k| configuration.get(k).unwrap())
            .fold(String::new(), |acum, arg| acum + ";" + arg);
        config.insert("pattern".into(), pattern);

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


