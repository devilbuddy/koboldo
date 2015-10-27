use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use std::collections::HashMap;

use world::Tile;

pub struct TileSet {
    pub texture : Texture,
    tiles : HashMap<Tile, TextureRegion>
}

impl TileSet {
    pub fn new(texture : Texture) -> TileSet {
        TileSet {
            texture : texture,
            tiles : HashMap::new()
        }
    }

    pub fn add_tile(&mut self, tile: Tile, texture_region : TextureRegion) {
        self.tiles.insert(tile, texture_region);
    }

    pub fn get_texture_region(&self, tile : &Tile) -> Option<&TextureRegion> {
        if self.tiles.contains_key(tile) {
            return self.tiles.get(tile);
        }
        return None;
    }
}

pub struct TextureRegion {
    pub bounds : Rect
}

impl TextureRegion {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> TextureRegion {
        let rect = Rect::new_unwrap(x, y, width, height);
        TextureRegion {
            bounds : rect
        }
    }
}
