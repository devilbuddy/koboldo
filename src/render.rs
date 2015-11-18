use motor::{MotorContext, MotorGraphics};

use std::cmp::*;

use world::{Cell};
use world::grid::Grid;

use tiles::TileSet;
use camera::Camera;

pub fn render_grid(context : &mut MotorContext, grid : &Grid<Cell>, tile_set : &TileSet, camera : &Camera) {

    let tile_size = 8i32;

    let num_tiles_across = (camera.size.0 / tile_size as u32) + 1;
    let num_tiles_down = (camera.size.1 / tile_size as u32) + 2;


    //println!("num_tiles_across {:?} num_tiles_down {:?}", num_tiles_across, num_tiles_down);

    let cam_pos = (camera.position.0 as i32, camera.position.1 as i32);
    let size = (camera.size.0 as i32, camera.size.1 as i32);

    println!("cam_pos ({:?}-{:?}) size:({:?}-{:?})", cam_pos.0, cam_pos.1, size.0, size.1);

    let foo_x = camera.position.0 - (size.0 as f64 / 2f64);
    let foo_y = camera.position.1 - (size.1 as f64 / 2f64);

    println!("foo {:?} {:?}", foo_x, foo_y);
    let xxx = foo_x as i32 / tile_size;
    let yyy = foo_y as i32 / tile_size;;
    println!("___ {:?} {:?}", xxx, yyy);

    let start_x = max(foo_x as i32 / tile_size, 0i32) as u32;
    let start_y = max(foo_y as i32 / tile_size, 0i32) as u32;

    println!("start_x {:?} start_y {:?}", start_x, start_y);

    let mut count = 0;
    for y in start_y..(start_y + num_tiles_down) {
        for x in start_x..(start_x + num_tiles_across) {
            match grid.get(x, y) {
                Some(cell) => {

                    let t = &cell.tile;
                    let texture_region = tile_set.get_texture_region(&t).expect("No texture region for tile");

                    let x_pos = x as i32 * tile_size as i32 - (foo_x as i32);
                    let y_pos = y as i32 * tile_size as i32 - (foo_y as i32);

                    //println!("x_pos {:?} y_pos {:?}", x_pos, y_pos);
                    context.render(&tile_set.texture, texture_region, (x_pos, y_pos));

                    count += 1;
                    //let e = &cell.entity;

                },
                _ => {}
            };
        }
    }

    println!("{:?}", count);
}
