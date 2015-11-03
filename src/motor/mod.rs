extern crate sdl2;
extern crate sdl2_image;

pub mod grid;
pub mod gfx;

use sdl2::{TimerSubsystem, EventPump};
use sdl2::event::Event;
use sdl2::render::{Renderer};
use sdl2_image::{INIT_PNG};

use sdl2::keyboard::{KeyboardState, Keycode};
use std::collections::HashSet;

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>,
    event_pump : EventPump,
    pub motor_keyboard : MotorKeyboard
}

impl<'window> MotorContext<'window> {
    pub fn new(renderer : Renderer<'window>, event_pump : EventPump) -> MotorContext<'window> {
        sdl2_image::init(INIT_PNG);

        MotorContext {
            renderer : renderer,
            event_pump : event_pump,
            motor_keyboard : MotorKeyboard::new()
        }
    }

    pub fn update(&mut self) {
        self.motor_keyboard.update(self.event_pump.keyboard_state());
    }


}

impl<'window> Drop for MotorContext<'window> {
    fn drop(&mut self) {
        sdl2_image::quit();
    }
}

pub trait MotorGraphics {
    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32));
}

impl<'window> MotorGraphics for MotorContext<'window> {
    fn render(&mut self, texture: &sdl2::render::Texture, texture_region : &gfx::TextureRegion, position : (i32, i32)) {
        self.renderer.copy(texture,
            Some(texture_region.bounds),
            Some(sdl2::rect::Rect::new_unwrap(position.0, position.1, texture_region.bounds.width(), texture_region.bounds.height()))
        );
    }
}

pub trait MotorApp {
    fn init(&mut self, motor_context : &mut MotorContext);
    fn update(&mut self, motor_context : &mut MotorContext, delta_time : f64) -> bool;
}

pub struct MotorKeyboard {
    new_keys : HashSet<Keycode>,
    prev_keys : HashSet<Keycode>
}

impl MotorKeyboard {
    fn new() -> MotorKeyboard {
        MotorKeyboard {
            new_keys : HashSet::new(),
            prev_keys : HashSet::new()
        }
    }

    fn update(&mut self, keyboard_state : KeyboardState) {
        let keys = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        self.new_keys = &keys - &self.prev_keys;
        self.prev_keys = keys;
    }

    pub fn is_key_pressed(&self, key_code : Keycode) -> bool {
        return self.prev_keys.contains(&key_code);
    }

    pub fn is_key_just_pressed(&self, key_code : Keycode) -> bool {
        return self.new_keys.contains(&key_code);
    }

}

struct MotorTimer {
    timer_subsystem : TimerSubsystem,
    interval : u32,
    last_tick : u32,
    last_second : u32,
    fps : u16,
    log_enabled : bool
}

impl MotorTimer {
    pub fn new(target_fps : u32, mut timer_subsystem : TimerSubsystem) -> MotorTimer {

        let now = timer_subsystem.ticks();

        MotorTimer {
            timer_subsystem : timer_subsystem,
            interval : 1000 / target_fps,
            last_tick : now,
            last_second : now,
            fps : 0,
            log_enabled : false
        }
    }

    pub fn tick(&mut self) -> (bool, f64) {
        let now = self.timer_subsystem.ticks();
        let dt = now - self.last_tick;
        let elapsed = dt as f64 / 1000.0;
        if dt < self.interval {
            self.timer_subsystem.delay(self.interval - dt);
            return (false, 0f64);
        }
        self.last_tick = now;

        if self.log_enabled {
            self.fps += 1;
            if now - self.last_second > 1000 {
                println!("FPS: {}", self.fps);
                self.last_second = now;
                self.fps = 0;
            }
        }
        return (true, elapsed);
    }

    pub fn set_enable_fps_log(&mut self, enabled : bool) {
        self.log_enabled = enabled;
    }
}

pub fn motor_start(window_title : &'static str, width: u32, height : u32, app : &mut MotorApp) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut motor_timer = MotorTimer::new(60, sdl_context.timer().unwrap());
    motor_timer.set_enable_fps_log(true);

    let window = video.window(window_title, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut motor_context = MotorContext::new(
        window.renderer().build().unwrap(),
        sdl_context.event_pump().unwrap()
    );

    app.init(&mut motor_context);

    'running: loop {

        motor_context.update();

        for event in motor_context.event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>  {
                    break 'running;
                },
                _ => {}
            }
        }

        let t = motor_timer.tick();
        if t.0 {
            motor_context.renderer.clear();
            if app.update(&mut motor_context, t.1) {
                break 'running;
            }
            motor_context.renderer.present();
        }
    }
}
