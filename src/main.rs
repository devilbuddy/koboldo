extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::path::Path;

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode};
use sdl2::controller::{Button};

mod motor;
use motor::{MotorGraphics, TextureReference};
use motor::gfx::{Animation, TextureRegion, SpriteBuilder, Sprite, NinePatch};
use motor::font::BitmapFont;

mod world;
use world::grid::Grid;

mod tiles;
mod render;
mod generator;

use tiles::*;
use world::*;

struct Assets {
    tile_set : TileSet,
    grid : Grid<Cell>,
    font : BitmapFont,
    monster_texture : TextureReference,
    nine_patch : NinePatch
}

struct App {
    state_time : f64,
    assets : Option<Assets>,
    sprites : Vec<Sprite>,
    controller_id : Option<i32>
}

impl App {
    pub fn new() -> App {
        App {
            state_time : 0f64,
            assets : None,
            sprites : Vec::new(),
            controller_id : None
        }
    }
}

fn make_grid(width : u32, height : u32) -> Grid<Cell> {
    let mut grid = Grid::<Cell>::new(width, height);

    let mut rng = rand::thread_rng();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let tile = rng.gen::<Tile>();
            grid.set(x, y, Cell::new(tile));
        }
    }

    let entity = Entity { position : Point { x: 0, y: 0}};
    grid.get_mut(0, 0).unwrap().set_entity(entity);

    grid
}


impl motor::MotorApp for App {
    fn init(&mut self, context : &mut motor::MotorContext) {

        let mut tile_set = TileSet::new(context.load_texture(&Path::new("assets/level_assets.png")));
        tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
        tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));

        context.renderer.set_draw_color(Color::RGB(0, 0, 0));

        let nine_patch = NinePatch::new(context.load_texture_as_ref(&Path::new("assets/level_assets.png")),
                                        TextureRegion::new(0, 8, 8, 8),
                                        3, 3, 3, 3);

        let assets = Assets {
            tile_set : tile_set,
            grid : make_grid(10, 10),
            font : context.load_font(&Path::new("assets/04b_03.fnt")),
            monster_texture : context.load_texture_as_ref(&Path::new("assets/monster_assets.png")),
            nine_patch : nine_patch
        };

        self.sprites.push(SpriteBuilder::new(assets.monster_texture.clone())
                    .animation(Animation::new(0.5f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]))
                    .position((40f64, 65f64))
                    .build());
        self.sprites.push(SpriteBuilder::new(assets.monster_texture.clone())
                    .texture_region(TextureRegion::new(16, 8, 8, 8))
                    .build());

        self.assets = Some(assets);
    }

    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64) -> bool {
        let mut done = false;
        if context.keyboard.is_key_pressed(Keycode::Escape) {
            done = true;
        }
        self.state_time += delta_time;

        let assets = self.assets.as_mut().unwrap();

        render::render_grid(context, &assets.grid, &assets.tile_set);

        let font = &assets.font;
        let mut y = 0;
        let x = 80;
        font.draw_str("ABCDEFGHIJGKMNOPQRSTUVWXYZ", x, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("abcdefghijklmnopqrstuvwxyz", x, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("0123456789 !:\"#¤%&/()=", x, y, &mut context.renderer);

        if self.controller_id.is_none() {
            self.controller_id = context.joystick.get_controller_id();
        }

        {
            let mut s = &mut self.sprites[0];
            let mut x = s.position.0;
            let mut y = s.position.1;
            let d =  delta_time * 50f64;

            if self.controller_id.is_some() {
                let c = context.joystick.get_controller(self.controller_id.unwrap());
                if c.game_controller.button(Button::DPadLeft) {
                    x -= d;
                }
                if c.game_controller.button(Button::DPadRight) {
                    x += d;
                }
                if c.game_controller.button(Button::DPadUp) {
                    y -= d;
                }
                if c.game_controller.button(Button::DPadDown) {
                    y += d;
                }
            }

            if context.keyboard.is_key_pressed(Keycode::Left) {
                x -= d;
            }
            if context.keyboard.is_key_pressed(Keycode::Right) {
                x += d;
            }
            if context.keyboard.is_key_pressed(Keycode::Up) {
                y -= d;
            }
            if context.keyboard.is_key_pressed(Keycode::Down) {
                y += d;
            }
            s.position = (x,y);
        }

        for s in self.sprites.iter_mut() {
            s.update(delta_time);
            context.render_sprite(s);
        }

        context.render_nine_patch(&assets.nine_patch, 10, 80, 47, 20);
        font.draw_str("Ninepatch", 14, 86, &mut context.renderer);

        return done;
    }
}

pub fn main() {
    let mut app = App::new();
    motor::motor_start("rust-sdl2-game", (800, 600), Some((200, 150)), &mut app)
}
