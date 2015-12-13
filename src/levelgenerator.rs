use generator;
use world::*;
use world::grid::Grid;

pub struct Level {
    pub grid : Grid<Cell>,
    pub start_tile : (u32, u32)
}

pub fn make_level(width : u32, height : u32) -> Level {
    println!("make level");

    let level = generator::make_level(width, height);
    let template = level.grid;

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

    //println!("{:?} {:?} {:?} {:?}", min_x, max_x, min_y, max_y);
    //println!("generating tiles");

    let w = max_x - min_x + 3;
    let h = max_y - min_y + 3;

    println!("{:?} {:?} {:?} {:?} ", w, h, min_x, min_y);

    let cell_size = 3;

    let mut grid = Grid::<Cell>::new(w * cell_size, h * cell_size);

    let mut start = (0, 0);

    let mut x = 0;
    let mut y = 0;
    for ty in (min_y - 1)..(max_y + 2) {
        for tx in (min_x - 1)..(max_x + 2) {


            if tx == level.start.0 && ty == level.start.1 {
                println!("whoot");
                start = (x, y);
            }

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

    Level {
        grid : grid,
        start_tile : start
    }
}
