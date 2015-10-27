extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2_image::{LoadTexture, INIT_PNG};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod motor;
use motor::grid::*;

mod tiles;
mod render;
mod world;

use tiles::*;
use world::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    sdl2_image::init(INIT_PNG);

    let window = video_subsystem.window("rust-sdl2-game", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.set_logical_size(200, 150).unwrap();

    let texture = renderer.load_texture(&Path::new("assets/level_assets.png")).unwrap();
    let monster_texture = renderer.load_texture(&Path::new("assets/monster_assets.png")).unwrap();

    let mut tile_set = TileSet::new(texture);
    tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
    tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));

    let mut grid = Grid::<Cell>::new(10, 10);

    grid.set(0, 0, Cell::new());
    grid.set(1, 1, Cell::new());
    grid.set(2, 1, Cell::new());

    let entity = Entity { position : Point { x: 0, y: 0}};
    grid.get_mut(0, 0).unwrap().set_entity(entity);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>  {
                    break 'running
                },
                Event::KeyDown {keycode, ..} => {
                    match keycode  {
                        Some(Keycode::Escape) => { break 'running },
                        Some(Keycode::Up) => {},
                        Some(Keycode::Down) => {},
                        Some(Keycode::Left) => {},
                        Some(Keycode::Right) => {},
                        _ => {}
                    };
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...
        renderer.clear();

        // render grid
        render::render_grid(&mut renderer, &grid, &tile_set);

        renderer.present();
    }
}
