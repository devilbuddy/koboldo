extern crate sdl2;
extern crate sdl2_image;
extern crate rand;
extern crate nalgebra as na;

use std::ops::{Add, Mul};

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode};

mod motor;
use motor::{MotorGraphics, TextureReference, MotorContext};
use motor::gfx::{Animation, TextureRegion, SpriteBuilder, Sprite, NinePatch};
use motor::font::BitmapFont;

mod world;
use world::grid::Grid;

mod render;
mod generator;
mod camera;
mod levelgenerator;

use world::*;
use camera::*;
use render::TileSet;

struct Bullet {
    entity : Entity,
    sprite : Sprite,
    alive : bool
}

impl Bullet {
    pub fn new(sprite : Sprite) -> Bullet {
        Bullet {
            entity : Entity::new(),
            sprite : sprite,
            alive : true
        }
    }
}

impl Actor for Bullet {
    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64, grid : &Grid<Cell>) -> Action {
        self.sprite.update(delta_time);
        let collision = world::move_entity(&mut self.entity, grid);
        if collision {
            self.alive = false;
        }
        Action::None
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }
}

struct Player {
    entity : Entity,
    sprite : Sprite,
    alive : bool
}

impl Player {
    pub fn new(sprite : Sprite) -> Player {
        Player {
            entity : Entity::new(),
            sprite : sprite,
            alive : true
        }
    }
}

impl Actor for Player {
    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64, grid : &Grid<Cell>) -> Action {

        let mut action = Action::None;

        self.sprite.update(delta_time);

        let acceleration = 0.5f64;
        if context.keyboard.is_key_pressed(Keycode::Left) {
            self.entity.velocity.x -= acceleration;
        }
        if context.keyboard.is_key_pressed(Keycode::Right) {
            self.entity.velocity.x += acceleration;
        }
        if context.keyboard.is_key_pressed(Keycode::Up) {
            self.entity.velocity.y -= acceleration;
        }
        if context.keyboard.is_key_pressed(Keycode::Down) {
            self.entity.velocity.y += acceleration;
        }
        if context.keyboard.is_key_pressed(Keycode::K) {
            self.alive = false;
        }
        world::move_entity(&mut self.entity, grid);


        let friction = 0.7f64;
        self.entity.velocity = self.entity.velocity.mul(friction);


        if context.keyboard.is_key_pressed(Keycode::Space) {
            action = Action::Fire {
                        x: self.entity.position.x,
                        y: self.entity.position.y,
                        velocity_x: self.entity.velocity.x * 2f64,
                        velocity_y: self.entity.velocity.y * 2f64
                    };
        }

        return action;
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
    fn get_entity(&self) -> &Entity {
        &self.entity
    }
    fn get_entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
    fn get_sprite(&self) -> &Sprite {
        &self.sprite
    }
}

struct Assets {
    tile_set : TileSet,
    font : BitmapFont,
    monster_texture : TextureReference,
    nine_patch : NinePatch
}

struct App {
    state_time : f64,
    assets : Option<Assets>,
    controller_id : Option<i32>,
    camera : Camera,
    world : Option<World>
}

impl App {
    pub fn new(display_size : (u32, u32)) -> App {
        App {
            state_time : 0f64,
            assets : None,
            controller_id : None,
            camera : Camera::new(display_size),
            world : None
        }
    }
}

impl motor::MotorApp for App {
    fn init(&mut self, context : &mut motor::MotorContext) {
        context.renderer.set_draw_color(Color::RGB(0, 0, 0));

        let mut tile_set = TileSet::new(context.load_texture(&Path::new("assets/level_assets.png")));
        tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
        tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));
        tile_set.add_tile(Tile::Solid, TextureRegion::new(0,16,8,8));
        tile_set.add_tile(Tile::Wall, TextureRegion::new(8,16,8,8));
        tile_set.add_tile(Tile::Floor, TextureRegion::new(64,0,8,8));

        let nine_patch = NinePatch::new(context.load_texture_as_ref(&Path::new("assets/level_assets.png")),
                                        TextureRegion::new(0, 8, 8, 8),
                                        3, 3, 3, 3);

        let assets = Assets {
            tile_set : tile_set,
            font : context.load_font(&Path::new("assets/04b_03.fnt")),
            monster_texture : context.load_texture_as_ref(&Path::new("assets/monster_assets.png")),
            nine_patch : nine_patch
        };

        let mut world = World::new();

        let player_sprite = SpriteBuilder::new(assets.monster_texture.clone())
                    .animation(Animation::new(0.5f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]))
                    .build();
        world.actors.push(Box::new(Player::new(player_sprite)));

        self.assets = Some(assets);
        self.world = Some(world);
    }

    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64) -> bool {
        let mut done = false;
        if context.keyboard.is_key_pressed(Keycode::Escape) {
            done = true;
        }

        self.state_time += delta_time;
        let assets = self.assets.as_mut().unwrap();
        let world = self.world.as_mut().unwrap();

        if context.keyboard.is_key_pressed(Keycode::R) {
            let level = levelgenerator::make_level(100, 100);
            {
                let start_position = (level.start_tile.0 as f64 * 8f64, level.start_tile.1 as f64 * 8f64);
                println!("start: {:?}", start_position);
                world.actors[0].get_entity_mut().set_position(start_position.0, start_position.1);
            }

            let grid = level.grid;
            self.camera.set_world_size(grid.width * 8, grid.height * 8);
            world.init(grid);
        }

        if world.grid.is_some() {
            render::render_grid(context, &world, &assets.tile_set, &self.camera);
        }

        if self.controller_id.is_none() {
            self.controller_id = context.joystick.get_controller_id();
        }

        let mut actions = Vec::new();
        world.update(context, delta_time, &mut actions);
        for action in actions.iter() {
            match *action {
                Action::Fire { x, y, velocity_x, velocity_y } => {
                    let bullet_sprite = SpriteBuilder::new(assets.monster_texture.clone())
                                .animation(Animation::new(0.5f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]))
                                .build();

                    let mut bullet = Bullet::new(bullet_sprite);
                    bullet.entity.position.x = x;
                    bullet.entity.position.y = y;
                    bullet.entity.velocity.x = velocity_x;
                    bullet.entity.velocity.y = velocity_y;

                    world.actors.push(Box::new(bullet));

                },
                _ => {}
            }
            /*
            match action {
                Action::Fire {x:x, y:y, velocity_x:velocity_x, velocity_y:velocity_y } => {

                },
                _ => {}
            }
            */
        }
        actions.clear();

        let p = &world.actors[0];
        let pos = p.get_entity().position;
        self.camera.set_position(pos.x, pos.y);

        let font = &assets.font;
        context.render_nine_patch(&assets.nine_patch, 1, 0, 47, 20);
        font.draw_str("Ninepatch", 5, 6, &mut context.renderer);

        let mut y = 0;
        let x = 80;
        font.draw_string(format!("x:{:.*}", 5,  pos.x), x, y, &mut context.renderer);
        y += font.line_height;
        font.draw_string(format!("y:{:.*}", 5,  pos.y), x, y, &mut context.renderer);

        return done;
    }
}

pub fn main() {
    let display_size = (200, 150);
    let mut app = App::new(display_size);
    motor::motor_start("rust-sdl2-game", (800, 600), Some(display_size), &mut app)
}
