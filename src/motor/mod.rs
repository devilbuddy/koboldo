extern crate sdl2;
extern crate sdl2_image;

pub mod keyboard;
pub mod joystick;
pub mod mouse;
pub mod gfx;
pub mod font;
mod timer;

use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem};
use sdl2::event::Event;
use sdl2::render::{Renderer, Texture};
use sdl2_image::{INIT_PNG, LoadTexture};

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

pub type TextureReference = Rc<RefCell<Texture>>;

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>,
    event_pump : EventPump,
    pub keyboard : keyboard::MotorKeyboard,
    pub joystick : joystick::MotorJoystick,
    pub mouse : mouse::MotorMouse
}

impl<'window> MotorContext<'window> {
    pub fn new(renderer : Renderer<'window>, event_pump : EventPump, game_controller_subsystem : GameControllerSubsystem, joystick_subsystem : JoystickSubsystem) -> MotorContext<'window> {
        sdl2_image::init(INIT_PNG);

        MotorContext {
            renderer : renderer,
            event_pump : event_pump,
            keyboard : keyboard::MotorKeyboard::new(),
            joystick : joystick::MotorJoystick::new(game_controller_subsystem, joystick_subsystem),
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
    fn load_texture_as_ref(&mut self, path : &Path) -> TextureReference;
    fn load_font(&mut self, path : &Path) -> font::BitmapFont;
    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32));
    fn render_sprite(&mut self, sprite : &gfx::Sprite);
    fn render_nine_patch(&mut self, nine_patch : &gfx::NinePatch, x: i32, y : i32, w: u32, h : u32);
}

impl<'window> MotorGraphics for MotorContext<'window> {
    fn load_texture(&mut self, path : &Path) -> Texture {
        self.renderer.load_texture(path).unwrap()
    }

    fn load_texture_as_ref(&mut self, path : &Path) -> TextureReference {
        Rc::new(RefCell::new(self.load_texture(path)))
    }

    fn load_font(&mut self, path: &Path) -> font::BitmapFont {
        font::BitmapFont::load(path, &self.renderer).unwrap()
    }

    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32)) {
        gfx::render_region(&mut self.renderer, texture, texture_region, position);
    }

    fn render_sprite(&mut self, sprite : &gfx::Sprite) {
        sprite.render(&mut self.renderer);
    }

    fn render_nine_patch(&mut self, nine_patch : &gfx::NinePatch, x: i32, y : i32, w: u32, h : u32) {
        nine_patch.render((x, y, w, h), &mut self.renderer);
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
        sdl_context.event_pump().unwrap(),
        sdl_context.game_controller().unwrap(),
        sdl_context.joystick().unwrap()
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
                Event::MouseMotion {..} | Event::MouseButtonDown {..} |
                Event::MouseButtonUp {..} | Event::MouseWheel {..} => {
                    context.mouse.handle_event(event);
                },
                Event::ControllerAxisMotion {..} | Event::ControllerButtonDown {..} |
                Event::ControllerButtonUp {..} | Event::ControllerDeviceAdded {..} |
                Event::ControllerDeviceRemapped {..} | Event::ControllerDeviceRemoved {..} => {
                    context.joystick.handle_event(event);
                },
                _ => {
                    //println!("unhandled event {:?}", event);
                }
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
