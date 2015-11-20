extern crate rand;

use world::grid::Grid;

use rand::thread_rng;
use rand::{Rng, Rand};
use rand::distributions::Range;
use rand::distributions::IndependentSample;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Rand for Direction {
     fn rand<R: Rng>(rng: &mut R) -> Direction {
         let range = Range::new(0, 4);
         match range.ind_sample(rng) {
             0 => Direction::Up,
             1 => Direction::Down,
             2 => Direction::Left,
             3 => Direction::Right,
             _ => panic!("oops")
         }
     }
}

impl Direction {
    pub fn turn(&self, turn_type : TurnType) -> Direction {
        match *self {
            Direction::Up => {
                match turn_type {
                    TurnType::Left => Direction::Left,
                    TurnType::Right => Direction::Right,
                    TurnType::UTurn => Direction::Down,
                    _=> Direction::Up
                }
            },
            Direction::Down => {
                match turn_type {
                    TurnType::Left => Direction::Right,
                    TurnType::Right => Direction::Left,
                    TurnType::UTurn => Direction::Up,
                    _=> Direction::Down
                }
            },
            Direction::Left => {
                match turn_type {
                    TurnType::Left => Direction::Down,
                    TurnType::Right => Direction::Up,
                    TurnType::UTurn => Direction::Right,
                    _=> Direction::Left
                }
            },
            Direction::Right => {
                match turn_type {
                    TurnType::Left => Direction::Up,
                    TurnType::Right => Direction::Down,
                    TurnType::UTurn => Direction::Left,
                    _=> Direction::Right
                }
            }
        }
    }
}

pub enum Tile {
    Wall,
    Floor
}

#[derive (Clone, Copy, Debug)]
enum TurnType {
    None,
    Left,
    Right,
    UTurn
}

struct TurnChanceConfig {
    turns : [TurnType ; 100],
    range : Range<usize>
}

impl TurnChanceConfig {
    pub fn new(left : u32, right : u32, u : u32) -> TurnChanceConfig {
        if left + right + u > 100 {
            panic!("more than 100!");
        }

        let mut turns = [TurnType::None; 100];
        let mut index = 0;
        for _ in 0..left {
            turns[index] = TurnType::Left;
            index += 1;
        }
        for _ in 0..right {
            turns[index] = TurnType::Right;
            index += 1;
        }
        for _ in 0..left {
            turns[index] = TurnType::UTurn;
            index += 1;
        }
        TurnChanceConfig {
            turns: turns,
            range : Range::new(0, 100)
        }
    }
    pub fn random_turn(&self) -> TurnType {
        let mut rng = rand::thread_rng();
        let index = self.range.ind_sample(&mut rng);
        self.turns[index]
    }
}

#[derive (Clone, Copy, Debug)]
enum RoomType {
    None,
    TwoByTwo,
    ThreeByThree
}

struct MakeRoomConfig {
    rooms : [RoomType; 100],
    range : Range<usize>
}

impl MakeRoomConfig {
    pub fn new(two_by_two : u32, three_by_three : u32) -> MakeRoomConfig {
        if two_by_two > 100 {
            panic!("more than 100!");
        }

        let mut rooms = [RoomType::None; 100];
        let mut index = 0;
        for _ in 0..two_by_two {
            rooms[index] = RoomType::TwoByTwo;
            index += 1;
        }
        for _ in 0..three_by_three {
            rooms[index] = RoomType::ThreeByThree;
            index += 1;
        }
        MakeRoomConfig {
            rooms: rooms,
            range : Range::new(0, 100)
        }
    }
    pub fn random_room_type(&self) -> RoomType {
        let mut rng = rand::thread_rng();
        let index = self.range.ind_sample(&mut rng);
        self.rooms[index]
    }
}

struct FloorMaker {
    x : u32,
    y : u32,
    direction : Direction,
    turn_chance_config : TurnChanceConfig,
    step_count : u32
}

// http://www.vlambeer.com/2013/04/02/random-level-generation-in-wasteland-kings/

impl FloorMaker {

    pub fn new(x : u32, y : u32) -> FloorMaker {
        FloorMaker {
            x : x,
            y : y,
            direction : rand::thread_rng().gen::<Direction>(),
            turn_chance_config  : TurnChanceConfig::new(10, 10, 10),
            step_count : 0
        }
    }

    pub fn step(&mut self) {
        let turn_type = self.turn_chance_config.random_turn();
        self.direction = self.direction.turn(turn_type);
        match self.direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        self.step_count += 1;
    }
}

fn place_floor(x : u32, y : u32, grid : &mut Grid<Tile>) -> bool{
    if grid.get(x, y).is_some() {
        match *grid.get(x, y).unwrap() {
            Tile::Wall => {
                grid.set(x, y, Tile::Floor);
                return true;
            },
            _ => {}
        }
    }
    false
}

fn place_room(room_type : RoomType, x_start : u32, y_start : u32, grid :  &mut Grid<Tile>) -> u32 {
    let mut w = 0;
    let mut h = 0;
    let mut floor_count = 0;
    match room_type {
        RoomType::TwoByTwo => {
            w = 2;
            h = 2;
        },
        RoomType::ThreeByThree => {
            w = 3;
            h = 3;
        },
        _ => {}
    }
    
    for y in y_start..(y_start + h) {
        for x in x_start..(x_start + w) {
            if place_floor(x, y, grid) {
                floor_count += 1;
            }
        }
    }
    floor_count
}

pub struct Level {
    pub grid : Grid<Tile>,
    pub start : (u32, u32)
}

pub fn make_level(width : u32, height : u32) -> Level {
    let mut grid = Grid::<Tile>::new(width, height);
    // fill with walls
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.set(x, y, Tile::Wall);
        }
    }

    let make_room_config = MakeRoomConfig::new(10, 10);
    let mut floor_makers = Vec::<FloorMaker>::new();

    let start = (50, 50);

    floor_makers.push(FloorMaker::new(start.0, start.1));

    let mut done = false;
    let mut floor_count = 0;
    while !done {
        let mut new_floor_makers = Vec::new();

        let chance_to_spawn_new_floor_maker = (floor_makers.len() * 10) as u32;

        for floor_maker in floor_makers.iter_mut() {
            floor_maker.step();

            if place_floor(floor_maker.x, floor_maker.y, &mut grid) {
                floor_count += 1;
                floor_count += place_room(make_room_config.random_room_type(), floor_maker.x, floor_maker.y, &mut grid);
            }

            // spawn new
            if rand::thread_rng().gen_weighted_bool(chance_to_spawn_new_floor_maker) {
                new_floor_makers.push(FloorMaker::new(floor_maker.x, floor_maker.y));
            }
        }

        // add spawned floor_makers
        floor_makers.extend(new_floor_makers);

        // remove floor_makers
        let num_floor_makers = floor_makers.len();
        if num_floor_makers > 1 {
            let mut rng = rand::thread_rng();
            let mut num_left = num_floor_makers;
            let chance_to_destroy = (100 - num_floor_makers * 10) as u32;

            // save in vector - return true
            floor_makers.retain(|ref floor_maker| {
                let mut r = true;
                if num_left > 1 {
                    if floor_maker.step_count > 0 {
                        r = rng.gen_weighted_bool(chance_to_destroy);
                    }
                    if !r {
                        num_left -= 1;
                    }
                }
                r
            });
        }
        if floor_makers.len() == 0 {
            panic!("no floormakers left - shoudnt happen")
        }

        if floor_count > 110 {
            done = true;
        }
    }

    Level {
        grid : grid,
        start : start
    }
}
