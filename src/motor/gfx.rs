use sdl2::rect::Rect;

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
