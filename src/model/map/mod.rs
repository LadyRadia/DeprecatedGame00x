mod tile;
use self::tile::Tile;
use ::model::Actor;
use std::fmt::Debug;

pub struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32
}

#[derive(Debug)]
pub enum MapError {
    InvalidCoordinates(String),
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut tiles : Vec<Tile> = Vec::with_capacity((width * height) as usize);
        //tile initialization here
        let mut new_self = Map {
            width: width,
            height: height,
            tiles: tiles,
        };
        new_self.initialize_tiles();
        return new_self;
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Result<& Tile, MapError> {
        let index = self.get_index_from_coordinates(x, y)?;
        Ok(&self.tiles[index])
    }

    //let's try to avoid publicly exposing tile mutability
    fn get_tile_mut(&mut self, x: u32, y: u32) -> Result<&mut Tile, MapError> {
        let index = self.get_index_from_coordinates(x, y)?;
        Ok(&mut self.tiles[index])
    }

    pub fn set_tile(&mut self, x: u32, y: u32, new_tile: Tile) -> Result<&Tile, MapError> {
        let index = self.get_index_from_coordinates(x, y)?;
        self.tiles[index] = new_tile;
        Ok(&self.tiles[index])
    }

    fn get_index_from_coordinates(&self, x: u32, y: u32) -> Result<usize, MapError> {
        self.validate_parameters(x, y)?;
        Ok(((x * self.width) + y ) as usize)
    }

    fn validate_parameters(&self, x: u32, y: u32) -> Result<(), MapError> {
        if x >= self.width || y >= self.height {
            Err(MapError::InvalidCoordinates(format!("x or y were invalid with width {width} and height {height}", width=self.width, height=self.height)))
        } else {
            Ok(())
        }
    }

    fn initialize_tiles(&mut self) {
        for y in 0..self.height  {
            for x in 0..self.width  {
                //unwrapping just bc we know, from the above range, it's valid
                //let index = self.get_index_from_coordinates(x, y).expect("Error initializing tiles, indexes out of range. Aborting");
                self.tiles.push(Tile {
                    actor: None
                });
            }
        }
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_actor_at_location(&self, x: u32, y: u32) -> &Option<Box<Actor>> {
        if let Ok(tile) = self.get_tile(x, y) {
            &tile.actor
        } else {
            &None
        }
    }

    pub fn place_actor_at_location(&mut self, x: u32, y: u32, actor: Box<Actor>) {
        let index = self.get_index_from_coordinates(x, y).expect("Unable to find index of tile");
        //TODO refactor tile array to be its own structure, have all these methods on it
        if let Ok(tile) = self.get_tile_mut(x, y) {
            tile.set_actor(actor);
        } else {
            warn!("Received invalid coordinates for player location: {}, {}", x, y);
            //don't fail fast for now.
        }
    }
}
