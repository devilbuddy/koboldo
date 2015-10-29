pub mod grid;
pub mod gfx;

use sdl2::render::{Renderer};

pub struct MotorContext<'window> {
    pub renderer : Renderer<'window>
}

impl<'window> MotorContext<'window> {
    pub fn new(renderer : Renderer<'window>) -> MotorContext<'window> {
        MotorContext {
            renderer : renderer
        }
    }
}

impl<'window> Drop for MotorContext<'window> {
    fn drop(&mut self) {
        
    }
}

pub enum ViewState {
    None,
    Quit
}

pub trait View {
    fn render(&mut self) -> ViewState;
}
