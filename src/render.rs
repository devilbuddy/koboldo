use motor::{MotorContext, MotorGraphics};
use motor::gfx::Sprite;

use std::cmp::*;

use world::{Cell};
use world::grid::Grid;

use tiles::TileSet;
use camera::Camera;

use world::{Actor, World};

pub fn render_grid(context : &mut MotorContext, world: &World, tile_set : &TileSet, camera : &Camera) {

    let grid = &world.grid;
    let actors = &world.actors;

    let tile_size = 8i32;

    let num_tiles_across = camera.size.0 / tile_size as u32;
    let num_tiles_down = camera.size.1 / tile_size as u32;

    let offset_x = camera.position.0 - (camera.size.0 as f64 / 2f64);
    let offset_y = camera.position.1 - (camera.size.1 as f64 / 2f64);

    let start_x = max(offset_x as i32 / tile_size, 0i32) as u32;
    let start_y = max(offset_y as i32 / tile_size, 0i32) as u32;
    let end_x = min(start_x + num_tiles_across + 1, grid.width);
    let end_y = min(start_y + num_tiles_down + 2, grid.height);

    for y in start_y..end_y {
        for x in start_x..end_x {
            match grid.get(x, y) {
                Some(cell) => {
                    let t = &cell.tile;
                    let texture_region = tile_set.get_texture_region(&t).expect("No texture region for tile");

                    let x_pos = x as i32 * tile_size - offset_x as i32;
                    let y_pos = y as i32 * tile_size - offset_y as i32;
                    context.render(&tile_set.texture, texture_region, (x_pos, y_pos));
                },
                _ => {}
            };
        }
    }

    for actor in actors {
        let position = actor.get_entity().position;
        let x = position.x - offset_x;
        let y = position.y - offset_y;
        context.render_sprite_at(actor.get_sprite(), x, y);
    }

}
