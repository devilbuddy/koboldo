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
