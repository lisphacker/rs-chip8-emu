
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

    fn read_pixel(&self, x: u8, y: u8) -> u8 {
        let off = y as usize * WIDTH + x as usize;
        return self.mem[off];
    }
    
    fn write_pixel(&mut self, x: u8, y: u8, val: u8) {
        let off = y as usize * WIDTH + x as usize;
        self.mem[off] = if val == 0 { 0 } else { 1 };
    }

    fn write_pixel_xor(&mut self, x: u8, y: u8, val: u8) -> bool {
        let off = y as usize * WIDTH + x as usize;
        let val = if val == 0 { 0 } else { 1 };
        let cleared = self.mem[off] == 1 && val == 1;
        self.mem[off] &= val;
        cleared
    }

    fn write_pixel_row(&mut self, x: u8, y : u8, rowval: u8) {
        for i in 0..8 {
            self.write_pixel(x + i, y, rowval >> (7 - i));
        }
    }
    
    fn write_pixel_row_xor(&mut self, x: u8, y : u8, rowval: u8) -> bool {
        let mut cleared = false;
        for i in 0..8 {
            let pixel_cleared = self.write_pixel_xor(x + i, y, rowval >> (7 - i));
            cleared = cleared || pixel_cleared;
        }
        cleared
    }
}
