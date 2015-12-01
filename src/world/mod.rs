extern crate nalgebra as na;

pub mod grid;
use self::grid::*;

use rand::{Rng, Rand};
use self::na::*;
use std::ops::{Add, Mul};

use motor::MotorContext;
use motor::gfx::Sprite;

#[derive(PartialEq, Eq, Hash)]
pub enum Tile {
    Grass,
    Water,

    Solid,
    Wall,
    Floor
}

impl Rand for Tile {
     fn rand<R: Rng>(rng: &mut R) -> Tile {
         if rng.gen::<bool>() {
             Tile::Grass
         } else {
             Tile::Water
         }
     }
}

pub struct Cell {
    pub tile : Tile,
}

impl Cell {
    pub fn new(tile : Tile) -> Cell {
        Cell {
            tile : tile
        }
    }
}



pub struct Entity {
    pub position : Vec2<f64>,
    pub velocity : Vec2<f64>,
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

    pub fn set_position(&mut self, x : f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
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


pub fn do_collision_check(entity : &mut Entity, grid : &Grid<Cell>) {
    let size = 8f64;
    let mut player_rect = Rectangle::new(entity.position.x, entity.position.y , size, size);

    let mut start_x;
    let mut end_x;
    let mut start_y;
    let mut end_y;
    if entity.velocity.x > 0f64 {
        start_x = ((player_rect.x + size + entity.velocity.x)/size) as u32;
        end_x = start_x;
    } else {
        start_x = ((player_rect.x + entity.velocity.x)/size) as u32;
        end_x = start_x;
    }
    start_y = (player_rect.y /size) as u32;
    end_y = ((player_rect.y + size) / size) as u32;
    get_collision_tiles(start_x, end_x, start_y, end_y, grid, &mut entity.collision_data);
    player_rect.x += entity.velocity.x;
    'x_loop: for i in 0..entity.collision_data.count {
        if player_rect.overlaps(&entity.collision_data.rects[i]) {
            entity.velocity.x = 0f64;
            break 'x_loop;
        }
    }
    player_rect.x = entity.position.x;

    if entity.velocity.y > 0f64 {
        start_y = ((player_rect.y + size + entity.velocity.y)/size) as u32;
        end_y = start_y;
    } else {
        start_y = ((player_rect.y + entity.velocity.y)/size) as u32;
    }
    start_x = (player_rect.x /size) as u32;
    end_x = ((player_rect.x + size) / size) as u32;
    get_collision_tiles(start_x, end_x, start_y, end_y, grid, &mut entity.collision_data);
    player_rect.y += entity.velocity.y;
    'y_loop: for i in 0..entity.collision_data.count {
        if player_rect.overlaps(&entity.collision_data.rects[i]) {
            entity.velocity.y = 0f64;
            break 'y_loop;
        }
    }
    entity.position = entity.position.add(entity.velocity);

    let friction = 0.7f64;
    entity.velocity = entity.velocity.mul(friction);
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

pub trait Actor {
    fn update(&mut self, context : &mut MotorContext, delta_time : f64,  grid : &Grid<Cell>) -> bool;
    fn get_entity(&self) -> &Entity;
    fn get_entity_mut(&mut self) -> &mut Entity;
    fn get_sprite(&self) -> &Sprite;
}

pub struct World {
    pub grid : Grid<Cell>,
    pub actors : Vec<Box<Actor>>,
}

impl World {
    pub fn new(grid : Grid<Cell>) -> World {
        World {
            grid : grid,
            actors : Vec::new()
        }
    }

    pub fn init(&mut self, grid : Grid<Cell>) {
        self.grid = grid;
        self.actors[0].get_entity_mut().set_position(100f64, 100f64);
    }

    pub fn update(&mut self, context : &mut MotorContext, delta_time : f64) {
        for actor in self.actors.iter_mut() {
            actor.update(context, delta_time, &self.grid);
        }
    }
}
