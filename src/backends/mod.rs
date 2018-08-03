
pub mod sdl;
pub mod textui;

use chip8::core::{RcRefKeyboardInterface, RcRefDisplayInterface};

#[derive(Clone)]
#[derive(Copy)]
pub enum BackendType {
    TUI,
    SDL
}

pub trait Backend {
    fn get_keyboard_interface(&self) -> RcRefKeyboardInterface;
    fn get_display_interface(&self) -> RcRefDisplayInterface;

    fn run(&mut self);
}

pub fn get_backend(backend_type: BackendType) -> Box<Backend> {
    match backend_type {
        BackendType::TUI => Box::new(textui::TextUI::new()),
        BackendType::SDL => Box::new(sdl::SDL::new())
    }
}
