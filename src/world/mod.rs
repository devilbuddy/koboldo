pub mod grid;

use rand::{Rng, Rand};

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

pub struct Point {
    pub x : i32,
    pub y : i32
}

pub struct Entity {
    pub position : Point
}


pub struct Cell {
    pub tile : Tile,
    pub entity : Option<Entity>
}

impl Cell {
    pub fn new(tile : Tile) -> Cell {
        Cell {
            tile : tile,
            entity : None
        }
    }

    pub fn set_entity(&mut self, entity : Entity) {
        self.entity = Some(entity);
    }
}
