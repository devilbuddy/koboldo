extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2_image::LoadTexture;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod motor;
use motor::grid::*;

mod tiles;
mod render;
mod world;

use tiles::*;
use world::*;


struct App {
    tile_set : Option<TileSet>,
    grid : Option<Grid<Cell>>
}

impl App {
    pub fn new() -> App {
        App {
            tile_set : None,
            grid : None
        }
    }
}

impl motor::MotorApp for App {
    fn init(&mut self, motor_context : &mut motor::MotorContext) {
        let texture = motor_context.renderer.load_texture(&Path::new("assets/level_assets.png")).unwrap();

        let mut tile_set = TileSet::new(texture);
        tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
        tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));


        let mut grid = Grid::<Cell>::new(10, 10);

        grid.set(0, 0, Cell::new());
        grid.set(1, 1, Cell::new());
        grid.set(2, 1, Cell::new());

        let entity = Entity { position : Point { x: 0, y: 0}};
        grid.get_mut(0, 0).unwrap().set_entity(entity);


        self.tile_set = Some(tile_set);
        self.grid = Some(grid);

        motor_context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        motor_context.renderer.set_logical_size(200, 150).unwrap();
    }

    fn update(&mut self, motor_context : &mut motor::MotorContext) -> bool{

        let mut done = false;

        for event in motor_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>  {
                    done = true;
                },
                Event::KeyDown {keycode, ..} => {
                    match keycode  {
                        Some(Keycode::Escape) => { done = true; },
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

        let grid = self.grid.as_ref().unwrap();
        let tile_set = self.tile_set.as_ref().unwrap();
        render::render_grid(motor_context, grid, tile_set);

        return done;
    }
}

pub fn main() {
    let mut app = App::new();
    motor::motor_start(800, 600, &mut app)
}
