extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;
use std::collections::HashMap;

use sdl2_image::{LoadTexture, INIT_PNG};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

#[derive(PartialEq, Eq, Hash)]
enum Tile {
    Grass,
    Water
}

struct TileSet {
    pub texture : Texture,
    tiles : HashMap<Tile, TextureRegion>
}

impl TileSet {
    pub fn new(texture : Texture) -> TileSet {
        TileSet {
            texture : texture,
            tiles : HashMap::new()
        }
    }

    pub fn add_tile(&mut self, tile: Tile, texture_region : TextureRegion) {
        self.tiles.insert(tile, texture_region);
    }

    pub fn get_texture_region(&self, tile : &Tile) -> Option<&TextureRegion> {
        if self.tiles.contains_key(tile) {
            return self.tiles.get(tile);
        }
        return None;
    }
}

struct TextureRegion {
    pub bounds : Rect
}

impl TextureRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> TextureRegion {
        let rect = Rect::new_unwrap(x, y, width, height);
        TextureRegion {
            bounds : rect
        }
    }
}

struct Grid<T> {
    pub width : u32,
    pub height : u32,
    cells : Vec<Option<T>>
}

impl <T> Grid <T> {
    pub fn new(width: u32, height: u32) -> Grid<T> {
        let mut vec = Vec::new();
        let size = width * height;
        for _ in 0..size {
            vec.push(None);
        }
        Grid {
            width: width,
            height: height,
            cells : vec
        }
    }

    pub fn get(&self, x : u32, y : u32) -> Option<&T> {
        let index = self.width * y + x;
        return self.cells[index as usize].as_ref();
    }

    pub fn set(&mut self, x : u32, y: u32, element : T) {
        let index = self.width * y + x;
        self.cells[index as usize] = Some(element);
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    sdl2_image::init(INIT_PNG);

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let texture = renderer.load_texture(&Path::new("assets/level_assets.png")).unwrap();

    let mut tile_set = TileSet::new(texture);
    tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
    tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));

    match renderer.set_logical_size(200, 150) {
        Ok(_) => {},
        Err(err) => panic!("Failed to set logical size: {}", err)
    };


    renderer.set_draw_color(Color::RGB(255, 0, 0));

    let mut grid = Grid::<Tile>::new(10, 10);
    grid.set(0, 0, Tile::Grass);
    grid.set(1, 1, Tile::Grass);
    grid.set(2, 1, Tile::Water);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        renderer.clear();

        for y in 0..grid.height {
            for x in 0..grid.width {
                match grid.get(x, y) {
                    Some(t) => {
                        let texture_region = tile_set.get_texture_region(&t).expect("");
                        let dst_rect = Rect::new_unwrap(x as i32* 8, y as i32 * 8, 8, 8);
                        renderer.copy(&tile_set.texture, Some(texture_region.bounds), Some(dst_rect));
                    },
                    _ => {}
                };
            }
        }

        renderer.present();
    }
}
