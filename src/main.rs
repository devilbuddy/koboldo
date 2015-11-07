extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::render::Texture;
use sdl2_image::LoadTexture;

use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode};

mod motor;
use motor::MotorGraphics;
use motor::grid::*;
use motor::gfx::{Animation, TextureRegion};
use motor::font::*;

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
    font : Option<BitmapFont>,
    state_time : f64
}

impl App {
    pub fn new() -> App {
        App {
            tile_set : None,
            grid : None,
            monster_texture : None,
            animation : None,
            font : None,
            state_time : 0f64
        }
    }
}

impl motor::MotorApp for App {
    fn init(&mut self, context : &mut motor::MotorContext) {
        let texture = context.load_texture(&Path::new("assets/level_assets.png"));

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
                context.load_texture(&Path::new("assets/monster_assets.png"))
            );

        self.animation = Some(Animation::new(1f64,
            vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]));


        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.set_logical_size(200, 150).unwrap();

        self.font = Some(context.load_font(&Path::new("assets/04b_03.fnt")));

    }

    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64) -> bool {
        let mut done = false;
        if context.keyboard.is_key_pressed(Keycode::Escape) {
            done = true;
        }

        let grid = self.grid.as_ref().unwrap();
        let tile_set = self.tile_set.as_ref().unwrap();
        render::render_grid(context, grid, tile_set);

        self.state_time += delta_time;


        let texture_region = self.animation.as_ref().unwrap().get_texture_region(self.state_time);
        context.render(self.monster_texture.as_ref().unwrap(), texture_region, (60, 60));

        let font = self.font.as_ref().unwrap();
        let mut y = 20;
        font.draw_str("ABCDEFGHIJGKMNOPQRSTUVWXYZ", 20, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("abcdefghijklmnopqrstuvwxyz", 20, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("0123456789 !:\"#¤%&/()=", 20, y, &mut context.renderer);

        return done;
    }
}

pub fn main() {
    let mut app = App::new();
    motor::motor_start("rust-sdl2-game", 800, 600, &mut app)
}
