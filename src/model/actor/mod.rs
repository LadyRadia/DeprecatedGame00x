pub mod fighter;

//trait as old as time,
//impl as old as rhyme,
//actor game design pattern
pub trait Actor {
    fn update(&mut self);

    fn get_pos(&self) -> [u32; 2];
}
