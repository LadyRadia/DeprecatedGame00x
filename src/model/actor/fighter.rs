//TODO there has to be a better way to do this, right?
use model::actor::*;

pub struct Fighter {
    pub name: String,
    pub pos: [u32; 2],
}

impl Fighter {
    pub fn new(pos: [u32; 2], name: &str) -> Fighter {
        Fighter {
            pos: pos,
            name: name.to_string(),
        }
    } 
}

impl Actor for Fighter {
    fn update(&mut self) {
        debug!("update called on fighter {}", self.name);
    }

    fn get_pos(&self) -> [u32; 2] {
        self.pos
    }

    fn get_actor_type(&self) -> ActorType {
        ActorType::Player
    }
}