pub mod grid;
pub mod gfx;

use sdl2::render::{Renderer};

pub enum ViewState {
    None,
    Quit
}

pub trait View {
    fn render(&mut self) -> ViewState;
}
