
use chip8::core::KeyboardInterface;

pub struct Keyboard {
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard
    }
}

impl KeyboardInterface for Keyboard {
    fn key_pressed(&self, key: u8) {
    }
    
    fn wait_for_key(&self, key: u8) {
    }
}
