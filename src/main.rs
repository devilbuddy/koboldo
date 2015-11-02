extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::render::Texture;
use sdl2_image::LoadTexture;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode};

mod motor;
use motor::grid::*;
use motor::gfx::{Animation, TextureRegion};

mod tiles;
mod render;
mod world;

use tiles::*;
use world::*;


struct App {
    tile_set : Option<TileSet>,
    grid : Option<Grid<Cell>>,
    monster_texture : Option<Texture>,
    animation : Option<Animation>,
    state_time : f64
}

impl App {
    pub fn new() -> App {
        App {
            tile_set : None,
            grid : None,
            monster_texture : None,
            animation : None,
            state_time : 0f64
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
        self.monster_texture = Some(
                motor_context.renderer.load_texture(&Path::new("assets/monster_assets.png")).unwrap()
            );

        self.animation = Some(Animation::new(1f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]));


        motor_context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        motor_context.renderer.set_logical_size(200, 150).unwrap();
    }

    fn update(&mut self, motor_context : &mut motor::MotorContext, delta_time : f64) -> bool {


        let mut done = false;
        if motor_context.motor_keyboard.is_key_pressed(Keycode::Escape) {
            done = true;
        }

        for event in motor_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>  {
                    done = true;
                },
                _ => {}
            }
        }

        let grid = self.grid.as_ref().unwrap();
        let tile_set = self.tile_set.as_ref().unwrap();
        render::render_grid(motor_context, grid, tile_set);

        self.state_time += delta_time;


        let a = match self.animation {
            Some(ref animation) => animation,
            _ => panic!("ok")
        };
        let texture_region = a.get_texture_region(self.state_time);
        let dst_rect = Rect::new_unwrap(50, 50, 8, 8);
        motor_context.renderer.copy(self.monster_texture.as_ref().unwrap(), Some(texture_region.bounds), Some(dst_rect));

        /*
        let texture_region = self.animation.unwrap().get_texture_region(self.state_time);
        let dst_rect = Rect::new_unwrap(50, 50, 8, 8);
        motor_context.renderer.copy(self.monster_texture.as_ref().unwrap(), Some(texture_region.bounds), Some(dst_rect));
        */

        return done;
    }
}

pub fn main() {
    let mut app = App::new();
    motor::motor_start("rust-sdl2-game", 800, 600, &mut app)
}
