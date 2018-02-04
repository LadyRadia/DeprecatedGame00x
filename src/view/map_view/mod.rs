mod map_tile;

use ::view::Viewer;
use ::model::Model;
use opengl_graphics::{ GlGraphics, OpenGL, GlyphCache };
use glutin_window as Window;
use graphics::*;
use graphics::rectangle::*;


//hard-coded view of map is 11x11 FOR NOW
const VIEW_WIDTH : f64 = 11.;
const VIEW_HEIGHT : f64 = 11.;

pub struct MapView {
    
}


#[derive(Copy, Clone, Debug)]
struct MapDimensions {
    pub mapw_start : f64,
    pub mapw_end: f64,
    pub maph_start : f64,
    pub maph_end : f64,
}
impl MapDimensions {
    pub fn new(ws: f64, we: f64, hs: f64, he: f64) -> MapDimensions {
        MapDimensions {
            mapw_start : ws,
            mapw_end : we,
            maph_start: hs,
            maph_end :he,
        }
    }
}

fn map_dimensions_from_stats(width: u32, height: u32) -> MapDimensions {
    debug!("generating map dimensions from width: {}, height: {}", width, height);
    MapDimensions::new((width/4) as f64, (3 * width / 4) as f64 , -1., (3 * height / 4) as f64)
}

fn get_smallest_in_limit(pos: u32, border: u32) -> u32 {
    if pos < border {
        0
    } else {
        pos - border
    }
}

fn get_largest_in_limit(pos: u32, dist: u32, limit: u32 ) -> u32 {
    if pos + dist > limit {
        limit
    } else {
        pos + dist
    }
}

impl MapView {
    pub fn new() -> MapView {
        MapView{

        }
    }

    fn render_tiles(&mut self, map_dim: MapDimensions, model: &Model, glyphs: &mut GlyphCache, ctx: &mut Context, gfx: &mut GlGraphics) {
        //hardcode to 11x11 for now (lets you sit with 10 surrounding each dir)
        //center on player
        let (center_x, center_y) = { let pos = model.get_player_position(); (pos[0], pos[1])};
        //get coordinates of each corner of square we're rendering
        debug!("player at: {},{}", center_x, center_y);
        let left_bound = get_smallest_in_limit(center_x, VIEW_WIDTH as u32 / 2);
        let top_bound = get_smallest_in_limit(center_y, VIEW_HEIGHT as u32 / 2);
        let right_bound = get_largest_in_limit(center_x, (VIEW_WIDTH as u32 + 1) / 2 , model.get_map_width());
        let bottom_bound = get_largest_in_limit(center_y, (VIEW_HEIGHT as u32) + 1 / 2, model.get_map_height());
        let mut cur_x = map_dim.mapw_start;
        let mut cur_y = map_dim.maph_start;
        //yeah we're type-casting here, who cares, what are the odds something terrible will happen OH OGD
        let tile_width  = ((map_dim.mapw_end - map_dim.mapw_start)/ VIEW_WIDTH);
        let tile_height  = ((map_dim.maph_end - map_dim.maph_start)  / VIEW_HEIGHT);
        //now, render squares!
        for row in top_bound..bottom_bound {
            for col in left_bound..right_bound {
                let player_on_tile = row == center_y && col == center_x;
                let tile = map_tile::MapTile {
                    width: tile_width,
                    height: tile_height,
                    start_corner_x: cur_x,
                    start_corner_y: cur_y,
                    coord: (col, row),
                };
                tile.draw(model, glyphs, ctx, gfx);
                cur_x += tile_width;
            }
            //shift down a row
            cur_y += tile_height;
            cur_x = map_dim.mapw_start;
        }
    }

}

impl Viewer for MapView {
    fn notify(&mut self, m: &Model, glyphs: &mut GlyphCache, ctx: &mut Context, gfx: &mut GlGraphics, res: [u32; 2]) {
        debug!("mapview change notified");
        let border = Border {
            color: color::WHITE,
            radius: 1.,
        }; 

        let surrounding_grid : Rectangle = Rectangle {
            border: Some(border),
            color: color::TRANSPARENT,
            shape: Shape::Square,
        };
        let (width, height) = (res[0], res[1]);
        let map_dim : MapDimensions = map_dimensions_from_stats(width, height);
        debug!("Output data. Mapwstart: {} Mapwend: {} MapHStart: {} MapHEnd: {}", map_dim.mapw_start, map_dim.mapw_end, map_dim.maph_start, map_dim.maph_end);
        let dimensions = rectangle_by_corners(map_dim.mapw_start, map_dim.maph_start, map_dim.mapw_end, map_dim.maph_end);
        self.render_tiles(map_dim, m, glyphs, ctx, gfx);
        surrounding_grid.draw(dimensions, &ctx.draw_state, ctx.transform, gfx);
    }
}