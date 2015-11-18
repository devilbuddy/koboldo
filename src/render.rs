use motor::{MotorContext, MotorGraphics};

use world::{Cell};
use world::grid::Grid;

use tiles::TileSet;
use camera::Camera;

pub fn render_grid(context : &mut MotorContext, grid : &Grid<Cell>, tile_set : &TileSet, camera : &Camera) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some(cell) => {

                    let t = &cell.tile;
                    let texture_region = tile_set.get_texture_region(&t).expect("No texture region for tile");

                    let x_pos = x as i32 * 8 - (camera.position.0 as i32);
                    let y_pos = y as i32 * 8 - (camera.position.1 as i32);

                    context.render(&tile_set.texture, texture_region, (x_pos, y_pos));


                    //let e = &cell.entity;

                },
                _ => {}
            };
        }
    }
}
