pub mod grid;

#[derive(PartialEq, Eq, Hash)]
pub enum Tile {
    Grass,
    Water
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
    pub fn new() -> Cell {
        Cell {
            tile : Tile::Grass,
            entity : None
        }
    }

    pub fn set_entity(&mut self, entity : Entity) {
        self.entity = Some(entity);
    }
}
