use std::rc::Rc;
use std::cell::RefCell;

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

pub struct TextureRegion {
    x : u32,
    y : u32,
    w : u32,
    h : u32,
    pub bounds : Rect
}

impl TextureRegion {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> TextureRegion {
        let rect = Rect::new_unwrap(x as i32, y as i32, width, height);
        TextureRegion {
            x : x as u32,
            y : y as u32,
            w : width,
            h : height,
            bounds : rect
        }
    }

    pub fn xywh(&self) -> (u32, u32, u32, u32) {
        (self.x, self.y, self.w, self.h)
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

pub struct NinePatch {
    texture : Rc<RefCell<Texture>>,
    top_left : TextureRegion,
    top_center : TextureRegion,
    top_right : TextureRegion,
    middle_left : TextureRegion,
    middle_center : TextureRegion,
    middle_right : TextureRegion,
    bottom_left : TextureRegion,
    bottom_center : TextureRegion,
    bottom_right : TextureRegion
}

impl NinePatch {
    pub fn new(texture : Rc<RefCell<Texture>>, texture_region : TextureRegion, left : u32, right : u32, top : u32, bottom : u32) -> NinePatch {
        let (x, y, w, h) = texture_region.xywh();
        let left_width = x + left;
        let center_width = w - left - right;
        let middle_height = h - top - bottom;

        let top_left = TextureRegion::new(x, y, left_width, top);
        let top_center = TextureRegion::new(left_width, y, center_width, top);
        let top_right = TextureRegion::new(x + w - right, y, right, top);
        let middle_left = TextureRegion::new(x, y + top, left, middle_height);
        let middle_center = TextureRegion::new(x + left, y + top, left, middle_height);
        let middle_right = TextureRegion::new(x, y + top, right, middle_height);
        let bottom_left = TextureRegion::new(x, y + h - bottom, left_width, bottom);
        let bottom_center = TextureRegion::new(x + left, y + h - bottom, center_width, bottom);
        let bottom_right = TextureRegion::new(x + w - right, y + h - bottom, right, bottom);

        NinePatch {
            texture : texture,
            top_left : top_left,
            top_center : top_center,
            top_right : top_right,
            middle_left : middle_left,
            middle_center : middle_center,
            middle_right : middle_right,
            bottom_left : bottom_left,
            bottom_center : bottom_center,
            bottom_right : bottom_right
        }
    }

    pub fn render(&self, (x, y, w, h) : (i32, i32, u32, u32), renderer : &mut Renderer) {
        let t = self.texture.borrow();
        let left_width = self.top_left.w;
        let right_width = self.top_right.w;
        let top_height = self.top_left.h;
        let bottom_height = self.bottom_left.h;
        let center_width = w - left_width - right_width;
        let center_height = h - top_height - bottom_height;

        render_region_dst(renderer, &t, &self.top_left, (x, y), (left_width, top_height));
        render_region_dst(renderer, &t, &self.top_center, (x + left_width as i32, y), (center_width, top_height));
        render_region_dst(renderer, &t, &self.top_right, (x + w as i32 - right_width as i32, y), (right_width, top_height));
        render_region_dst(renderer, &t, &self.middle_left, (x, y + top_height as i32), (left_width, center_height));
        render_region_dst(renderer, &t, &self.middle_center, (x + left_width as i32, y + top_height as i32), (center_width, center_height));
        render_region_dst(renderer, &t, &self.middle_right, (x + w as i32 - right_width as i32, y + top_height as i32), (right_width, center_height));
        render_region_dst(renderer, &t, &self.bottom_left, (x, y + h as i32 - bottom_height as i32), (left_width, bottom_height));
        render_region_dst(renderer, &t, &self.bottom_center, (x + left_width as i32, y + h as i32 - bottom_height as i32), (center_width, bottom_height));
        render_region_dst(renderer, &t, &self.bottom_right, (x + w as i32 - right_width as i32, y + h as i32 - bottom_height as i32), (right_width, bottom_height));
    }
}

fn render_region_dst(renderer : &mut Renderer, texture: &Texture, texture_region : &TextureRegion, position : (i32, i32), size : (u32, u32)) {
    renderer.copy(texture,
        Some(texture_region.bounds),
        Some(Rect::new_unwrap(position.0, position.1, size.0, size.1))
    );
}

pub fn render_region(renderer : &mut Renderer, texture: &Texture, texture_region : &TextureRegion, position : (i32, i32)) {
    renderer.copy(texture,
        Some(texture_region.bounds),
        Some(Rect::new_unwrap(position.0, position.1, texture_region.bounds.width(), texture_region.bounds.height()))
    );
}
