extern crate sdl2;
extern crate sdl2_image;
extern crate rand;
extern crate nalgebra as na;

use std::path::Path;
use std::string::ToString;

use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode};

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



use na::{Vec2};
use std::ops::{Add, Mul};

struct Entity {
    position : Vec2<f64>,
    velocity : Vec2<f64>,
    collision_data : CollisionData
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            position : na::zero(),
            velocity : na::zero(),
            collision_data : CollisionData::new()
        }
    }
}


#[derive (Clone, Copy, Debug)]
struct Rectangle {
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h : f64) -> Rectangle {
        Rectangle {
            x : x,
            y : y,
            w : w,
            h : h
        }
    }

    pub fn init(&mut self, x: f64, y: f64, w: f64, h : f64) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }

    pub fn overlaps(&self, r : &Rectangle) -> bool {
        self.x < r.x + r.w && self.x + self.w > r.x && self.y < r.y + r.h && self.y + self.h > r.y
    }
}

struct CollisionData {
    rects : [Rectangle; 5],
    count : usize
}

impl CollisionData {
    pub fn new() -> CollisionData {
        CollisionData {
            rects : [Rectangle::new(0f64, 0f64, 0f64, 0f64) ; 5],
            count : 0
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn add(&mut self, x: f64, y: f64, w: f64, h : f64) {
        self.rects[self.count].init(x, y, w , h);
        self.count += 1;
    }
}

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
    camera : Camera,
    player : Entity
}

impl App {
    pub fn new(display_size : (u32, u32)) -> App {
        App {
            state_time : 0f64,
            assets : None,
            sprites : Vec::new(),
            controller_id : None,
            camera : Camera::new(display_size),
            player : Entity::new()
        }
    }


}


fn get_collision_tiles(start_x : u32, end_x : u32, start_y : u32, end_y : u32, grid : &Grid<Cell>, collision_data : &mut CollisionData) {
    let size = 8f64;

    collision_data.reset();

    for y in start_y..(end_y + 1) {
        for x in start_x..(end_x + 1) {
            let t = grid.get_if(x, y, |cell| {
                match cell.tile {
                    Tile::Solid | Tile::Wall => {
                        true
                    }
                    _ => { false }
                }
            });
            if t.is_some() {
                collision_data.add(x as f64 * size, y as f64 * size, size, size);
            }
        }
    }
}


fn make_grid(width : u32, height : u32) -> Grid<Cell> {
    println!("make grid");

    let level = generator::make_level(width, height);
    let template = level.grid;

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
            self.player.position = Vec2::new(100f64, 100f64);
        }

        render::render_grid(context, &assets.grid, &assets.tile_set, &mut self.sprites, &self.camera);

        let font = &assets.font;
        let mut y = 0;
        let x = 80;
        font.draw_string(format!("x:{:.*}", 5,  self.player.position.x), x, y, &mut context.renderer);
        y += font.line_height;
        font.draw_string(format!("y:{:.*}", 5,  self.player.position.y), x, y, &mut context.renderer);

        if self.controller_id.is_none() {
            self.controller_id = context.joystick.get_controller_id();
        }

        {
            let acceleration = 0.5f64;
            let friction = 0.7f64;

            if context.keyboard.is_key_pressed(Keycode::Left) {
                self.player.velocity.x -= acceleration;
            }
            if context.keyboard.is_key_pressed(Keycode::Right) {
                self.player.velocity.x += acceleration;
            }
            if context.keyboard.is_key_pressed(Keycode::Up) {
                self.player.velocity.y -= acceleration;
            }
            if context.keyboard.is_key_pressed(Keycode::Down) {
                self.player.velocity.y += acceleration;
            }
            self.player.velocity = self.player.velocity.mul(friction);


            let size = 8f64;
            let mut player_rect = Rectangle::new(self.player.position.x, self.player.position.y , size, size);

            let mut start_x;
            let mut end_x;
            let mut start_y;
            let mut end_y;
            if self.player.velocity.x > 0f64 {
                start_x = ((player_rect.x + size + self.player.velocity.x)/size) as u32;
                end_x = start_x;
            } else {
                start_x = ((player_rect.x + self.player.velocity.x)/size) as u32;
                end_x = start_x;
            }
            start_y = (player_rect.y /size) as u32;
            end_y = ((player_rect.y + size) / size) as u32;
            get_collision_tiles(start_x, end_x, start_y, end_y, &assets.grid, &mut self.player.collision_data);
            player_rect.x += self.player.velocity.x;
            'x_loop: for i in 0..self.player.collision_data.count {
                if player_rect.overlaps(&self.player.collision_data.rects[i]) {
                    self.player.velocity.x = 0f64;
                    break 'x_loop;
                }
            }
            player_rect.x = self.player.position.x;

            if self.player.velocity.y > 0f64 {
                start_y = ((player_rect.y + size + self.player.velocity.y)/size) as u32;
                end_y = start_y;
            } else {
                start_y = ((player_rect.y + self.player.velocity.y)/size) as u32;
            }
            start_x = (player_rect.x /size) as u32;
            end_x = ((player_rect.x + size) / size) as u32;
            get_collision_tiles(start_x, end_x, start_y, end_y, &assets.grid, &mut self.player.collision_data);
            player_rect.y += self.player.velocity.y;
            'y_loop: for i in 0..self.player.collision_data.count {
                if player_rect.overlaps(&self.player.collision_data.rects[i]) {
                    self.player.velocity.y = 0f64;
                    break 'y_loop;
                }
            }

            let p = self.player.position.add(self.player.velocity);
            self.player.position = p;

            self.sprites[0].position = (self.player.position.x, self.player.position.y);
        }

        self.camera.position = self.sprites[0].position;

        for s in self.sprites.iter_mut() {
            s.update(delta_time);
        }

        context.render_nine_patch(&assets.nine_patch, 1, 0, 47, 20);
        font.draw_str("Ninepatch", 5, 6, &mut context.renderer);

        return done;
    }
}

pub fn main() {
    let display_size = (200, 150);
    let mut app = App::new(display_size);
    motor::motor_start("rust-sdl2-game", (800, 600), Some(display_size), &mut app)
}
