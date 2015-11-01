use sdl2::rect::Rect;

use motor::MotorContext;
use motor::grid::Grid;
use tiles::TileSet;
use world::Cell;

pub fn render_grid(motor_context : &mut MotorContext, grid : &Grid<Cell>, tile_set : &TileSet) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some(cell) => {

                    let t = &cell.tile;
                    let texture_region = tile_set.get_texture_region(&t).expect("No texture region for tile");
                    let dst_rect = Rect::new_unwrap(x as i32 * 8, y as i32 * 8, 8, 8);
                    motor_context.renderer.copy(&tile_set.texture, Some(texture_region.bounds), Some(dst_rect));

                    let e = &cell.entity;

                },
                _ => {}
            };
        }
    }
}
