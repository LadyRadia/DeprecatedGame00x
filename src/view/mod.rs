use ::model::Model;
use std::collections::HashMap;

use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache, TextureSettings };
use glutin_window::GlutinWindow as Window;
use piston::input::RenderArgs;
use graphics::{Graphics, Context};
use std::path::Path;

pub mod console_view;
pub mod map_view;

extern crate rusttype;
//place holder error for now for when uninitialized stuff is called
pub struct ViewError {

}

pub trait Viewer {
    fn notify (&mut self, m: &Model, glyphs: &mut GlyphCache, ctx: &mut Context, gfx: &mut GlGraphics, res: [u32; 2]);
}
/*
    wrapper around collection of views, initialized by the Controller
*/
pub struct Views<'a> {
    views: Vec<Box<Viewer>>,
    glyph_cache: GlyphCache<'a>,
}

impl<'a> Views<'a> {
    pub fn new(font_path: &Path,  settings: TextureSettings) -> Views<'a> {

        let mut views = Views {
            views: vec!{},
            glyph_cache: GlyphCache::new(font_path, (), settings).expect("Unable to load font from path"),
        };
        views.glyph_cache.preload_printable_ascii(12).expect("Failed to load fonts for ASCII");
        views
    }

    pub fn notify(&mut self, m: &Model, window: &mut Window, gfx: &mut GlGraphics, args: RenderArgs) -> Result<bool, ViewError> {
        //delegate actual draw logic to subviews.
        gfx.draw(args.viewport(), |mut ctx, mut sub_gfx| {
            for viewer in &mut self.views {
                viewer.notify(m, &mut self.glyph_cache, &mut ctx, &mut sub_gfx, [args.width, args.height]);
            }
        });
        Ok(true)
    }

    pub fn addView(&mut self, v: Box<Viewer>) {
        self.views.push(v)
    }
}