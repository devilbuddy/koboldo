extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2::render::Texture;
use sdl2_image::LoadTexture;

use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode};

mod motor;
use motor::MotorGraphics;
use motor::grid::*;
use motor::gfx::{Animation, TextureRegion};
use motor::font::*;

mod tiles;
mod render;
mod world;

use tiles::*;
use world::*;

use std::rc::Rc;
use std::cell::RefCell;

struct SpriteBuilder {
    texture : Rc<RefCell<Texture>>,
    texture_region : Option<TextureRegion>,
    animation : Option<Animation>,
}

impl SpriteBuilder {
    pub fn new(texture : Rc<RefCell<Texture>>) -> SpriteBuilder {
        SpriteBuilder {
            texture : texture,
            texture_region : None,
            animation : None
        }
    }
    pub fn texture_region(mut self, texture_region : TextureRegion) -> SpriteBuilder {
        self.texture_region = Some(texture_region);
        self
    }
    pub fn animation(mut self, animation : Animation) -> SpriteBuilder {
        self.animation = Some(animation);
        self
    }
    pub fn build(self) -> Sprite {
        Sprite {
            texture : self.texture,
            texture_region : self.texture_region,
            animation : self.animation,
            state_time : 0f64,
            color : (255, 255, 255),
            position : (0, 0)
        }
    }
}

struct Sprite {
    texture : Rc<RefCell<Texture>>,
    texture_region : Option<TextureRegion>,
    animation : Option<Animation>,
    state_time : f64,
    pub color : (u8, u8, u8),
    pub position : (i32, i32)
}

impl Sprite {
    pub fn update(&mut self, delta_time : f64) {
        self.state_time += delta_time;
    }

    pub fn draw(&self, context : &mut motor::MotorContext) {
        let mut t = self.texture.borrow_mut();
        t.set_color_mod(self.color.0, self.color.1, self.color.2);
        if self.animation.is_some() {
            let texture_region = self.animation.as_ref().unwrap().get_texture_region(self.state_time);
            context.render(&t, texture_region, self.position);
        } else {
            context.render(&t, self.texture_region.as_ref().unwrap(), self.position);
        }
    }
}

struct Assets {
    tile_set : TileSet,
    grid : Grid<Cell>,
    font : BitmapFont,
    monster_texture : Rc<RefCell<Texture>>
}

struct App {
    state_time : f64,
    assets : Option<Assets>,
    sprite : Option<Sprite>
}

impl App {
    pub fn new() -> App {
        App {
            state_time : 0f64,
            assets : None,
            sprite : None
        }
    }
}

fn make_grid(width : u32, height : u32) -> Grid<Cell> {
    let mut grid = Grid::<Cell>::new(width, height);

    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.set(x, y, Cell::new());
        }
    }

    let entity = Entity { position : Point { x: 0, y: 0}};
    grid.get_mut(0, 0).unwrap().set_entity(entity);

    grid
}

impl motor::MotorApp for App {
    fn init(&mut self, context : &mut motor::MotorContext) {

        let mut tile_set = TileSet::new(context.load_texture(&Path::new("assets/level_assets.png")));
        tile_set.add_tile(Tile::Grass, TextureRegion::new(0, 0, 8, 8));
        tile_set.add_tile(Tile::Water, TextureRegion::new(0, 8, 8, 8));

        context.renderer.set_draw_color(Color::RGB(0, 0, 0));

        self.assets = Some(
            Assets {
                tile_set : tile_set,
                grid : make_grid(10, 10),
                font : context.load_font(&Path::new("assets/04b_03.fnt")),
                monster_texture : Rc::new(RefCell::new(context.load_texture(&Path::new("assets/monster_assets.png"))))
            });

        let t = self.assets.as_ref().unwrap().monster_texture.clone();
        let s = SpriteBuilder::new(t)
                    //.texture_region(TextureRegion::new(0, 8, 8, 8))
                    .animation(Animation::new(0.5f64, vec![TextureRegion::new(0, 0, 8, 8), TextureRegion::new(0, 8, 8, 8)]))
                    .build();
        self.sprite = Some(s);
    }

    fn update(&mut self, context : &mut motor::MotorContext, delta_time : f64) -> bool {
        let mut done = false;
        if context.keyboard.is_key_pressed(Keycode::Escape) {
            done = true;
        }
        self.state_time += delta_time;

        let assets = self.assets.as_mut().unwrap();

        render::render_grid(context, &assets.grid, &assets.tile_set);

        let font = &assets.font;
        let mut y = 20;
        font.draw_str("ABCDEFGHIJGKMNOPQRSTUVWXYZ", 20, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("abcdefghijklmnopqrstuvwxyz", 20, y, &mut context.renderer);
        y += font.line_height;
        font.draw_str("0123456789 !:\"#¤%&/()=", 20, y, &mut context.renderer);

        let mut s = self.sprite.as_mut().unwrap();
        s.update(delta_time);
        s.draw(context);

        return done;
    }
}

pub fn main() {
    let mut app = App::new();
    motor::motor_start("rust-sdl2-game", (800, 600), Some((200, 150)), &mut app)
}
