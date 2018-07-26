
use chip8::types::Addr;
use chip8::types::ByteVal;

use chip8::core::DisplayInterface;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct DisplayBuffer {
    mem: [u8; WIDTH * HEIGHT]
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer {
            mem: [0; WIDTH * HEIGHT]
        }
    }
}

impl DisplayInterface for DisplayBuffer {
    fn dimensions(&self) -> (u8, u8) {
        (WIDTH as u8, HEIGHT as u8)
    }
    
    fn clear(&mut self) {
        self.mem = [0; WIDTH * HEIGHT];
    }

    fn toggle_pixel(&mut self, x: u8, y: u8) {
        let off = y as usize * WIDTH + x as usize;
        self.mem[off] ^= 1;
    }
}
