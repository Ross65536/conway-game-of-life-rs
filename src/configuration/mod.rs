use game::grid_state::GridState;
use std::collections::HashMap;

const X_SIZE: &'static str = "20";
const Y_SIZE: &'static str = "20";

#[derive(Debug)]
pub struct Configuration {
    arguments: HashMap<String, String>
} 


impl Configuration {
    fn init_deafult_args() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("xSize", X_SIZE);
        map.insert("ySize", Y_SIZE);

        map.iter().map(|p| ((*p.0).into(), (*p.1).into())).collect()
    } 

    pub fn new(configuration: &HashMap<String, String>) -> Configuration {
        let mut config = Configuration::init_deafult_args();
        configuration.into_iter().for_each(|(k, v)| { 
            config.insert((*k).clone(), (*v).clone()); 
        });
        Configuration { arguments: config } 
    }

    pub fn build_grid(&self) -> GridState {
        let x_size: usize = self.arguments.get("xSize".into())
            .unwrap()
            .parse()
            .unwrap();

        let y_size: usize = self.arguments.get("xSize".into())
            .unwrap()
            .parse()
            .unwrap();

        GridState::new_empty(x_size, y_size)
    }

}


