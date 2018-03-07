use ::model::Actor;

pub struct Tile {
    pub actor: Option<Box<Actor>>,
}

impl Tile {
    pub fn set_actor(&mut self, actor: Box<Actor>) {
        self.actor = Some(actor)
    }

    pub fn remove_actor(&mut self) {
        self.actor = None
    }
}