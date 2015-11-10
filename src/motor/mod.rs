extern crate sdl2;
extern crate sdl2_image;

pub mod keyboard;
pub mod mouse;
pub mod grid;
pub mod gfx;
pub mod font;
mod timer;

use sdl2::{EventPump};
use sdl2::event::Event;
use sdl2::render::{Renderer, Texture};
use sdl2_image::{INIT_PNG, LoadTexture};

use std::path::Path;

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>,
    event_pump : EventPump,
    pub keyboard : keyboard::MotorKeyboard,
    pub mouse : mouse::MotorMouse
}

impl<'window> MotorContext<'window> {
    pub fn new(renderer : Renderer<'window>, event_pump : EventPump) -> MotorContext<'window> {
        sdl2_image::init(INIT_PNG);

        MotorContext {
            renderer : renderer,
            event_pump : event_pump,
            keyboard : keyboard::MotorKeyboard::new(),
            mouse : mouse::MotorMouse::new()
        }
    }

    pub fn update(&mut self) {
        self.keyboard.update(self.event_pump.keyboard_state());
    }
}

impl<'window> Drop for MotorContext<'window> {
    fn drop(&mut self) {
        sdl2_image::quit();
    }
}

pub trait MotorGraphics {
    fn load_texture(&mut self, path : &Path) -> Texture;
    fn load_font(&mut self, path : &Path) -> font::BitmapFont;
    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32));
    fn render_sprite(&mut self, sprite : &gfx::Sprite);
}

impl<'window> MotorGraphics for MotorContext<'window> {
    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32)) {
        gfx::render_region(&mut self.renderer, texture, texture_region, position);
    }

    fn render_sprite(&mut self, sprite : &gfx::Sprite) {
        sprite.render(&mut self.renderer);
    }

    fn load_texture(&mut self, path : &Path) -> Texture {
        self.renderer.load_texture(path).unwrap()
    }

    fn load_font(&mut self, path: &Path) -> font::BitmapFont {
        font::BitmapFont::load(path, &self.renderer).unwrap()
    }
}

pub trait MotorApp {
    fn init(&mut self, context : &mut MotorContext);
    fn update(&mut self, context : &mut MotorContext, delta_time : f64) -> bool;
}

pub fn motor_start(window_title : &'static str, window_size : (u32, u32), logical_size : Option<(u32, u32)>, app : &mut MotorApp) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut timer = timer::MotorTimer::new(60, sdl_context.timer().unwrap());
    timer.set_enable_fps_log(true);

    let window = video.window(window_title, window_size.0, window_size.1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut context = MotorContext::new(
        window.renderer().build().unwrap(),
        sdl_context.event_pump().unwrap()
    );

    match logical_size {
        Some((w, h)) => {
            context.renderer.set_logical_size(w, h).unwrap();
        }
        _ => {}
    }

    app.init(&mut context);

    'running: loop {
        context.update();

        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>  {
                    break 'running;
                },
                Event::MouseMotion {..} | Event::MouseButtonDown {..} | Event::MouseButtonUp {..} | Event::MouseWheel {..} => {
                    context.mouse.handle_event(event);
                }
                _ => {}
            }
        }

        let t = timer.tick();
        if t.0 {
            context.renderer.clear();
            if app.update(&mut context, t.1) {
                break 'running;
            }
            context.renderer.present();
        }
    }
}
