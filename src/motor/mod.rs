extern crate sdl2;
extern crate sdl2_image;

pub mod grid;
pub mod gfx;

use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::{Renderer};
use sdl2_image::{INIT_PNG};

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>,
    pub event_pump : EventPump
}

impl<'window> MotorContext<'window> {
    pub fn new(renderer : Renderer<'window>, event_pump : EventPump) -> MotorContext<'window> {
        sdl2_image::init(INIT_PNG);

        MotorContext {
            renderer : renderer,
            event_pump : event_pump
        }
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
    before : u32,
    last_second : u32,
    fps : u16
}

/*
impl MotorTimer {
    pub fn new(target_fps : u32, timer_subsystem : TimerSubsystem) -> MotorTimer {

        let now = timer_subsystem.ticks();

        MotorTimer {
            timer_subsystem : timer_subsystem,
            interval : 1_000 / (target_fps as f64),
            before : now,
            last_second : now,
            fps : 0
        }
    }
}
*/

pub fn motor_start(window_title : &'static str, width: u32, height : u32, app : &mut MotorApp) {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

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

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    'running: loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;
        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }
        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            //println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        motor_context.renderer.clear();

        if app.update(&mut motor_context, elapsed) {
            break 'running;
        }

        motor_context.renderer.present();

    }

}
