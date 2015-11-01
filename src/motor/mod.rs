extern crate sdl2;
extern crate sdl2_image;

pub mod grid;
pub mod gfx;

use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::{Renderer};
use sdl2_image::{INIT_PNG};

use sdl2::keyboard::{KeyboardState, Keycode};
use std::collections::HashSet;

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>,
    pub event_pump : EventPump,
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

pub trait MotorApp {
    fn init(&mut self, motor_context : &mut MotorContext);
    fn update(&mut self, motor_context : &mut MotorContext, delta_time : f64) -> bool;
}

struct MotorTimer {
    timer_subsystem : TimerSubsystem,
    interval : u32,
    last_tick : u32,
    last_second : u32,
    fps : u16
}

pub struct MotorKeyboard {
    prev_keys : HashSet<Keycode>
}

impl MotorKeyboard {
    fn new() -> MotorKeyboard {
        MotorKeyboard {
            prev_keys : HashSet::new()
        }
    }

    fn update(&mut self, keyboard_state : KeyboardState) {
        let keys = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &self.prev_keys;
        let old_keys = &self.prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("{:?} -> {:?}", new_keys, old_keys);
        }

        self.prev_keys = keys;

    }

}

impl MotorTimer {
    pub fn new(target_fps : u32, mut timer_subsystem : TimerSubsystem) -> MotorTimer {

        let now = timer_subsystem.ticks();

        MotorTimer {
            timer_subsystem : timer_subsystem,
            interval : 1000 / target_fps,
            last_tick : now,
            last_second : now,
            fps : 0
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
        self.fps += 1;

        if now - self.last_second > 1_000 {
            println!("FPS: {}", self.fps);
            self.last_second = now;
            self.fps = 0;
        }
        return (true, elapsed);
    }
}


pub fn motor_start(window_title : &'static str, width: u32, height : u32, app : &mut MotorApp) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut motor_timer = MotorTimer::new(60, sdl_context.timer().unwrap());

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
