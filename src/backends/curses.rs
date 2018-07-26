
use cursive::views::TextView;
use cursive::Cursive;

use chip8::core::KeyboardInterface;

pub struct Curses {
    siv: Cursive
}

impl Curses {
    pub fn new() -> Self {
        let mut siv = Cursive::default();
        siv.add_global_callback('q', Cursive::quit);
        Curses {
            siv: siv
        }
    }
}

impl KeyboardInterface for Curses {
    fn key_pressed(&self, key: u8) -> bool{
        false
    }
    
    fn wait_for_key(&self, key: u8) {
    }
}
