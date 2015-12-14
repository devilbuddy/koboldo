extern crate nalgebra as na;

pub mod grid;

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
    pub width : f64,
    pub height : f64,
    collision_data : CollisionData
}

impl Entity {
    pub fn new(width: f64, height: f64) -> Entity {
        Entity {
            position : na::zero(),
            velocity : na::zero(),
            width : width,
            height : height,
            collision_data : CollisionData::new()
        }
    }

    pub fn set_position(&mut self, x : f64, y: f64) {
        println!("set_position {:?} {:?}", x, y);
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


pub fn move_entity(entity : &mut Entity, grid : &grid::Grid<Cell>) -> bool {

    let mut collision = false;

    let tile_size = 8f64;
    let mut player_rect = Rectangle::new(entity.position.x, entity.position.y , entity.width, entity.height);

    let mut start_x;
    let mut end_x;
    let mut start_y;
    let mut end_y;
    if entity.velocity.x > 0f64 {
        start_x = ((player_rect.x + player_rect.w + entity.velocity.x)/tile_size) as u32;
        end_x = start_x;
    } else {
        start_x = ((player_rect.x + entity.velocity.x)/tile_size) as u32;
        end_x = start_x;
    }
    start_y = (player_rect.y / tile_size) as u32;
    end_y = ((player_rect.y + player_rect.w) / tile_size) as u32;
    get_collision_tiles(start_x, end_x, start_y, end_y, grid, &mut entity.collision_data);
    player_rect.x += entity.velocity.x;
    'x_loop: for i in 0..entity.collision_data.count {
        if player_rect.overlaps(&entity.collision_data.rects[i]) {
            entity.velocity.x = 0f64;
            collision = true;
            break 'x_loop;
        }
    }
    player_rect.x = entity.position.x;

    if entity.velocity.y > 0f64 {
        start_y = ((player_rect.y + player_rect.h + entity.velocity.y)/tile_size) as u32;
        end_y = start_y;
    } else {
        start_y = ((player_rect.y + entity.velocity.y)/tile_size) as u32;
    }
    start_x = (player_rect.x / tile_size) as u32;
    end_x = ((player_rect.x + player_rect.w) / tile_size) as u32;
    get_collision_tiles(start_x, end_x, start_y, end_y, grid, &mut entity.collision_data);
    player_rect.y += entity.velocity.y;
    'y_loop: for i in 0..entity.collision_data.count {
        if player_rect.overlaps(&entity.collision_data.rects[i]) {
            entity.velocity.y = 0f64;
            collision = true;
            break 'y_loop;
        }
    }
    entity.position = entity.position.add(entity.velocity);

    collision
}

fn get_collision_tiles(start_x : u32, end_x : u32, start_y : u32, end_y : u32, grid : &grid::Grid<Cell>, collision_data : &mut CollisionData) {
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

pub enum Action {
    None,
    Fire {x: f64, y : f64, velocity_x : f64, velocity_y : f64}
}

pub trait Actor {
    fn update(&mut self, context : &mut MotorContext, delta_time : f64,  grid : &grid::Grid<Cell>) -> Action;
    fn is_alive(&self) -> bool;
    fn get_entity(&self) -> &Entity;
    fn get_entity_mut(&mut self) -> &mut Entity;
    fn get_sprite(&self) -> &Sprite;
}

pub struct World {
    pub grid : Option<grid::Grid<Cell>>,
    pub actors : Vec<Box<Actor>>,
}

impl World {
    pub fn new() -> World {
        World {
            grid : None,
            actors : Vec::new()
        }
    }

    pub fn init(&mut self, grid : grid::Grid<Cell>) {
        self.grid = Some(grid);
    }

    pub fn update(&mut self, context : &mut MotorContext, delta_time : f64, actions : &mut Vec<Action>) {
        if self.grid.is_some() {
            for actor in self.actors.iter_mut() {
                let action = actor.update(context, delta_time, self.grid.as_ref().unwrap());
                match action {
                    Action::None => {},
                    _ => { actions.push(action); }
                }
            }
        }
        self.actors.retain(|ref a| a.is_alive());
    }
}
