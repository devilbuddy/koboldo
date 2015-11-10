use motor::{MotorContext, MotorGraphics};

use world::{Cell};
use world::grid::Grid;

use tiles::TileSet;

pub fn render_grid(motor_context : &mut MotorContext, grid : &Grid<Cell>, tile_set : &TileSet) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some(cell) => {

                    let t = &cell.tile;
                    let texture_region = tile_set.get_texture_region(&t).expect("No texture region for tile");

                    motor_context.render(&tile_set.texture, texture_region, (x as i32 * 8, y as i32 * 8));


                    //let e = &cell.entity;

                },
                _ => {}
            };
        }
    }
}
