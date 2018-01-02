use ::view::Viewer;
use ::model::Model;
use controller::opengl_graphics::{ GlGraphics, OpenGL };
use controller::glutin_window as Window;
use controller::graphics::*;
use controller::graphics::rectangle::*;

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
    fn notify(&mut self, m: &Model, ctx: &mut Context, gfx: &mut GlGraphics, res: [u32; 2]) {
        debug!("mapview change notified");
        let border = Border {
            color: color::WHITE,
            radius: 1.,
        }; 

        let surrounding_grid : Rectangle = Rectangle {
            border: Some(border),
            color: color::BLACK,
            shape: Shape::Square,
        };
        let (width, height) = (res[0], res[1]);
        let dimensions = rectangle_by_corners((width/4) as f64, -1 as f64, (3 * (width/4)) as f64, (3*(height/4)) as f64);
        surrounding_grid.draw(dimensions, &ctx.draw_state, ctx.transform, gfx);
    }
}