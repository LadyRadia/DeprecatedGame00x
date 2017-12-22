use ::view::Viewer;
use ::model::Model;
use controller::opengl_graphics::{ GlGraphics, OpenGL };

pub struct ConsoleView {
}

impl ConsoleView {
    pub fn new() -> ConsoleView {
        ConsoleView{

        }
    }
}

impl Viewer for ConsoleView {
    fn notify(&mut self, m: &Model, gfx: &GlGraphics) {

    }
}