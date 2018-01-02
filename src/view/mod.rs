use ::model::Model;

use controller::opengl_graphics::{ GlGraphics, OpenGL };
use controller::glutin_window::GlutinWindow as Window;
use controller::piston::input::RenderArgs;
use controller::graphics::{Graphics, Context};

pub mod console_view;

pub mod map_view;

pub trait Viewer {
    fn notify (&mut self, m: &Model, ctx: &mut Context, gfx: &mut GlGraphics, res: [u32; 2]);
}
/*
    wrapper around collection of views, initialized by the Controller
*/
pub struct Views {
    views: Vec<Box<Viewer>>,
}

impl Views {
    pub fn new() -> Views {
        Views {
            views: vec!{},
        }
    }

    pub fn notify(&mut self, m: &Model, window: &mut Window, gfx: &mut GlGraphics, args: RenderArgs) {
        //delegate actual draw logic to subviews.
        gfx.draw(args.viewport(), |mut ctx, mut subGfx| {
            for viewer in &mut self.views {
                viewer.notify(m, &mut ctx, &mut subGfx, [args.width, args.height]);
            }
        });

    }

    pub fn addView(&mut self, v: Box<Viewer>) {
        self.views.push(v)
    }
}