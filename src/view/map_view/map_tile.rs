use ::model::Model;
use graphics::{Context};
use opengl_graphics::{ GlGraphics, GlyphCache };
use graphics::rectangle::*;
use graphics::text::*;
use graphics::color;

pub struct MapTile {
    pub width: f64,
    pub height:f64,
    pub start_corner_x: f64,
    pub start_corner_y: f64,
    pub player_present: bool,
}

impl MapTile {
    pub fn draw(&self, model: &Model, glyph_cache: &GlyphCache, ctx: &mut Context, gfx: &mut GlGraphics) {
        let rect = Rectangle {
            border: Some(Border {
                color: color::hex("2099B4"),
                radius: 0.5
            }),
            color: color::hex("4A0F78"),
            shape: Shape::Square,
        };
        let dimensions = rectangle_by_corners(self.start_corner_x, self.start_corner_y, self.start_corner_x + self.width, self.start_corner_y + self.height);
        rect.draw(dimensions, &ctx.draw_state, ctx.transform, gfx);
        if self.player_present {
            let player_char = Text {
                color: color::WHITE,
                font_size: 2 * self.width as u32 / 3,
                round: true,
            };
            
        }
    }
}