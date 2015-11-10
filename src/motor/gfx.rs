use std::rc::Rc;
use std::cell::RefCell;

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

pub struct SpriteBuilder {
    texture : Rc<RefCell<Texture>>,
    texture_region : Option<TextureRegion>,
    animation : Option<Animation>,
    position : (i32, i32)
}

impl SpriteBuilder {
    pub fn new(texture : Rc<RefCell<Texture>>) -> SpriteBuilder {
        SpriteBuilder {
            texture : texture,
            texture_region : None,
            animation : None,
            position : (0, 0)
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
    pub fn position(mut self, position : (i32, i32)) -> SpriteBuilder {
        self.position = position;
        self
    }
    pub fn build(self) -> Sprite {
        if self.texture_region.is_some() && self.animation.is_some() {
            panic!("both texture_region and animation can't be set");
        }
        Sprite {
            texture : self.texture,
            texture_region : self.texture_region,
            animation : self.animation,
            state_time : 0f64,
            color : (255, 255, 255),
            position : self.position
        }
    }
}

pub struct Sprite {
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

    pub fn render(&self, renderer : &mut Renderer) {
        let mut t = self.texture.borrow_mut();
        t.set_color_mod(self.color.0, self.color.1, self.color.2);
        if self.animation.is_some() {
            let texture_region = self.animation.as_ref().unwrap().get_texture_region(self.state_time);
            render_region(renderer, &t, texture_region, self.position);
        } else {
            render_region(renderer, &t, self.texture_region.as_ref().unwrap(), self.position);
        }
    }

}

pub fn render_region(renderer : &mut Renderer, texture: &Texture, texture_region : &TextureRegion, position : (i32, i32)) {
    renderer.copy(texture,
        Some(texture_region.bounds),
        Some(Rect::new_unwrap(position.0, position.1, texture_region.bounds.width(), texture_region.bounds.height()))
    );
}
