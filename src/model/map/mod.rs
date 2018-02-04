mod tile;
use self::tile::Tile;
use ::model::Actor;

pub struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32
}

#[derive()]
pub enum MapError {
    InvalidCoordinates(String),

}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let tiles : Vec<Tile> = Vec::with_capacity((width * height) as usize);
        //tile initialization here
        Map {
            width: width,
            height: height,
            tiles: tiles,
        }
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Result<&Tile, MapError> {
        let index = self.get_index_from_coordinates(x, y)?;
        Ok(&self.tiles[index])
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
        }  else {
            &None
        }
    }
}
