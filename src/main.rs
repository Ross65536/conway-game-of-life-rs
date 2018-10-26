use std::collections::HashMap;
use std::env;

const X_SIZE: &'static str = "20";
const Y_SIZE: &'static str = "20";

fn init_deafult_args() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("xSize", X_SIZE);
    map.insert("ySize", Y_SIZE);

    map.iter().map(|p| (String::from(*p.0), String::from(*p.1))).collect()
} 

fn init_args() -> HashMap<String, String> {
    let mut map = init_deafult_args();

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

fn main() {
    let args = init_args();
    println!("{:?}", args);
}

