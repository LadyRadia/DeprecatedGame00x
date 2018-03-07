use model::Model;
use graphics::{Context};
use opengl_graphics::{ GlGraphics, GlyphCache };
use graphics::rectangle::*;
use graphics::Transformed;
use graphics::color;
use graphics::Image;
use graphics::character::CharacterCache;
use std::cmp;
use model::actor::*;

pub struct MapTile {
    pub width: f64,
    pub height:f64,
    pub start_corner_x: f64,
    pub start_corner_y: f64,
    pub coord: (u32, u32),
}

fn fetch_player_symbol(actor: ActorType) -> char {
    match actor {
        Player => '@'
    }
}

impl MapTile {
    pub fn draw(&self, model: &Model, glyph_cache: &mut GlyphCache, ctx: &mut Context, gfx: &mut GlGraphics) {
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
        if let &Some(ref actor) = model.get_actor_at_location(self.coord.0, self.coord.1) {
            debug!("Rendering actor at location: ({}, {})", self.coord.0, self.coord.1);
            let symbol = fetch_player_symbol(actor.get_actor_type());
            let font_size : u32 = cmp::min((2 * self.width as u32 / 3), (2 * self.height as u32 / 3)) + 1;
            if let Ok(actor_symbol) = glyph_cache.character(font_size, symbol) {
                debug!("Rendering actor");
                let img = Image::new_color(color::WHITE);
                img.draw(
                    actor_symbol.texture,
                    &ctx.draw_state,
                    ctx.transform.trans(self.start_corner_x + font_size as f64 / 3., self.start_corner_y + font_size as f64 / 3.),
                    gfx
                );
            } else {
                warn!("Unable to find cache for actor: {:?}", actor.get_actor_type());
            }
        }

    }
}