use world::Tile;
use motor::gfx::TextureRegion;
use sdl2::render::{Texture};
use std::collections::HashMap;

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
