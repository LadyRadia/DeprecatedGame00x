pub mod input;
mod map;

use self::map::Map;
use self::input::Input;

pub struct Model {
    map: Map
}

impl Model {
    pub fn new() -> Model {
        Model{
            map: Map::new(1, 1)
        }
    }

    pub fn player_input(&mut self, input: Input) {
        debug!("Input pressed: {:?}", input);
    } 
}