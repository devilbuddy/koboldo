use world::grid::Grid;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub enum Tile {
    Wall,
    Floor
}

struct FloorMaker {
    x : u32,
    y : u32,
    direction : Direction
}

// http://www.vlambeer.com/2013/04/02/random-level-generation-in-wasteland-kings/

impl FloorMaker {
    pub fn step(&mut self) {

    }
}

pub fn make_level(width : u32, height : u32) -> Grid<Tile> {

    let mut floor_makers = Vec::<FloorMaker>::new();

    let mut grid = Grid::<Tile>::new(width, height);

    // fill with walls
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.set(x, y, Tile::Wall);
        }
    }

    let mut done = false;
    while !done {

    }


    grid
}
