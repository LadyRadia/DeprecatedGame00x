pub mod fighter;

//trait as old as time,
//impl as old as rhyme,
//actor game design pattern
pub trait Actor {
    fn update(&mut self);

    fn get_pos(&self) -> [u32; 2];

    fn get_actor_type(&self) -> ActorType;
}

#[derive(Debug)]
pub enum ActorType {
    Player,
    Enemy,
}
