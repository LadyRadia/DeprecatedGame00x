use ::model::Model;

use controller::opengl_graphics::{ GlGraphics, OpenGL };
use controller::glutin_window::GlutinWindow as Window;
use controller::piston::event::RenderArgs;

mod console_view;

pub mod map_view;

pub trait Viewer {
    fn notify(&mut self, m: &Model, gfx: &GlGraphics);
}
/*
    wrapper around collection of views, initialized by the Controller
*/
pub struct Views {
    views: Vec<Box<Viewer>>
}

impl Views {
    pub fn new() -> Views {
        Views {
            views: vec!{}
        }
    }

    pub fn notify(&mut self, m: &Model, window: &mut Window, gfx: &mut GlGraphics, args: RenderArgs) {
        for viewer in &mut self.views {
            viewer.notify(m, gfx);
        }
    }

    pub fn addView(&mut self, v: Box<Viewer>) {
        self.views.push(v)
    }
}