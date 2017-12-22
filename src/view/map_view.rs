use ::view::Viewer;
use ::model::Model;
use controller::opengl_graphics::{ GlGraphics, OpenGL };

pub struct MapView {
    
}

struct MapTile {
    
}

impl MapView {
    pub fn new() -> MapView {
        MapView{

        }
    }
}

impl Viewer for MapView {
    fn notify(&mut self, m: &Model, gfx: &GlGraphics) {

    }
}