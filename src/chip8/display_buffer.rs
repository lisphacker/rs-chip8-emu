
use chip8::types::Addr;
use chip8::types::ByteVal;

use chip8::core::DisplayInterface;

pub struct DisplayBuffer {
    mem: Vec<u8>
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer {
            mem: vec![0; 1]
        }
    }
}

impl DisplayInterface for DisplayBuffer {
    fn clear(&self) {}
}
