pub mod input;
mod map;
pub mod actor;

use self::map::Map;
use self::input::Input;
use self::actor::Actor;
use self::actor::fighter::Fighter;

pub struct Model {
    map: Map,
    player: Box<Actor>,
}

impl Model {
    pub fn new() -> Model {
        Model{
            map: Map::new(50, 50),
            player: Box::new(Fighter::new([25, 25], "Beamed" )),
        }
    }

    pub fn player_input(&mut self, input: Input) {
        debug!("Input pressed: {:?}", input);
    } 

    pub fn get_player_position(&self) -> [u32; 2] {
        self.player.get_pos()
    }
    
    pub fn get_map_width(&self) -> u32 {
        self.map.get_width()
    }

    pub fn get_map_height(&self) -> u32 {
        self.map.get_height()
    }

    //should we even wrap this? is it OK to expose map to views? probably..
    pub fn get_actor_at_location(&self, x: u32, y: u32) -> &Option<Box<Actor>> {
        debug!("fetching actor at {},{}", x, y);
        self.map.get_actor_at_location(x, y)
    }
}