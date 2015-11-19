extern crate sdl2;
extern crate sdl2_image;
extern crate rand;

use std::path::Path;

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
mod camera;

use tiles::*;
use world::*;
use camera::*;

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
    controller_id : Option<i32>,
    camera : Camera
}

impl App {
    pub fn new(display_size : (u32, u32)) -> App {
        App {
            state_time : 0f64,
            assets : None,
            sprites : Vec::new(),
            controller_id : None,
            camera : Camera::new(display_size)
        }
    }
}


fn make_grid(width : u32, height : u32) -> Grid<Cell> {
    println!("make grid");

    let template = generator::make_level(width, height);
    println!("generated level");

    let mut min_x = template.width;
    let mut min_y = template.height;
    let mut max_x = 0;
    let mut max_y = 0;

    for y in 0..height {
        for x in 0..width {
            match *template.get(x, y).unwrap() {
                generator::Tile::Floor => {
                    if y < min_y {
                        min_y = y;
                    }
                    if y >= max_y {
                        max_y = y;
                    }
                    if x < min_x {
                        min_x = x;
                    }
                    if x >= max_x {
                        max_x = x;
                    }
                },
                _ => {}
            }
        }
    }

    println!("{:?} {:?} {:?} {:?}", min_x, max_x, min_y, max_y);
    println!("generating tiles");

    let w = max_x - min_x + 3;
    let h = max_y - min_y + 3;

    let cell_size = 3;

    let mut grid = Grid::<Cell>::new(w * cell_size, h * cell_size);

    let mut x = 0;
    let mut y = 0;
    for ty in (min_y - 1)..(max_y + 2) {
        for tx in (min_x - 1)..(max_x + 2) {
            grid.fill(x, y, cell_size, cell_size, || {
                let tile;
                match *template.get(tx, ty).unwrap() {
                    generator::Tile::Floor => {
                        tile = Tile::Floor;
                    },
                    generator::Tile::Wall => {
                        tile = Tile::Solid;
                    }
                }
                Cell::new(tile)
            });
            x += cell_size;
        }
        x = 0;
        y += cell_size;
    }

    // "autotile"
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut below_is_floor = false;
            {
                let below = grid.get(x, y + 1);
                if below.is_some() && below.unwrap().tile == Tile::Floor {
                    below_is_floor = true;
                }
            }

            let mut cell = grid.get_mut(x, y).unwrap();
            if cell.tile == Tile::Solid && below_is_floor {
                cell.tile = Tile::Wall;
            }

        }
    }

    println!("generated tiles");
    grid
}


impl motor::MotorApp for App {
    fn init(&mut self, context : &mut motor::MotorContext) {


        let mut tile_set = TileSet::new(context.load_texture(&Path::new("assets/level_assets.png")));
        tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
        tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));

        tile_set.add_tile(Tile::Solid, TextureRegion::new(0,16,8,8));
        tile_set.add_tile(Tile::Wall, TextureRegion::new(8,16,8,8));
        tile_set.add_tile(Tile::Floor, TextureRegion::new(64,0,8,8));

        context.renderer.set_draw_color(Color::RGB(0, 0, 0));

        let nine_patch = NinePatch::new(context.load_texture_as_ref(&Path::new("assets/level_assets.png")),
                                        TextureRegion::new(0, 8, 8, 8),
                                        3, 3, 3, 3);

        let assets = Assets {
            tile_set : tile_set,
            grid : make_grid(100, 100),
            font : context.load_font(&Path::new("assets/04b_03.fnt")),
            monster_texture : context.load_texture_as_ref(&Path::new("assets/monster_assets.png")),
            nine_patch : nine_patch
        };

        self.sprites.push(SpriteBuilder::new(assets.monster_texture.clone())
                    .animation(Animation::new(0.5f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]))
                    .position((96f64, 71f64))
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

        if context.keyboard.is_key_pressed(Keycode::R) {
            assets.grid = make_grid(100, 100);
        }

        render::render_grid(context, &assets.grid, &assets.tile_set, &self.camera);

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
            let mut x = self.camera.position.0;
            let mut y = self.camera.position.1;
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
            self.camera.position = (x,y);
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
    let display_size = (200, 150);
    let mut app = App::new(display_size);
    motor::motor_start("rust-sdl2-game", (800, 600), Some(display_size), &mut app)
}
