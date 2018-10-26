use std::collections::HashMap;
use std::env;

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