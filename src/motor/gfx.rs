use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};


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
