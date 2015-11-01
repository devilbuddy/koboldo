extern crate sdl2;
extern crate sdl2_image;

pub mod grid;
pub mod gfx;

use sdl2::EventPump;
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
    fn update(&mut self, motor_context : &mut MotorContext) -> bool;
}

pub fn motor_start(width: u32, height : u32, app : &mut MotorApp) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2-game", width, height)
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
        motor_context.renderer.clear();

        if app.update(&mut motor_context) {
            break 'running;
        }
        
        motor_context.renderer.present();

    }

}
