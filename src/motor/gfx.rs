use sdl2::rect::Rect;
use sdl2::render::Texture;

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


use sdl2::render::{Renderer};

pub fn render<'a>(renderer : &mut Renderer<'a>, texture: &Texture, texture_region : &TextureRegion, position : (i32, i32)) {
    renderer.copy(texture,
        Some(texture_region.bounds),
        Some(Rect::new_unwrap(position.0, position.1, texture_region.bounds.width(), texture_region.bounds.height()))
    );
}


pub struct Animation {
    frame_duration : f64,
    frames : Vec<TextureRegion>
}

impl Animation {

    pub fn new(frame_duration : f64, frames : Vec<TextureRegion>) -> Animation {
        Animation {
            frame_duration : frame_duration,
            frames : frames
        }
    }

    pub fn get_texture_region(&self, state_time : f64) -> &TextureRegion {
        let frame_number =  state_time / self.frame_duration;
        let frame_index = frame_number % (self.frames.len() as f64);
        return &self.frames[frame_index as usize];
    }
}
