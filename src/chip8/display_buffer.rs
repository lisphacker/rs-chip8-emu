
use chip8::core::DisplayInterface;
use chip8::types::ByteVal;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct DisplayBuffer {
    mem: [ByteVal; WIDTH * HEIGHT]
}

impl DisplayBuffer {
    pub fn new() -> Self {
        DisplayBuffer {
            mem: [0; WIDTH * HEIGHT]
        }
    }
}

impl DisplayInterface for DisplayBuffer {
    fn dimensions(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }
    
    fn clear(&mut self) {
        self.mem = [0; WIDTH * HEIGHT];
    }

    fn read_pixel(&self, x: ByteVal, y: ByteVal) -> ByteVal {
        let off = y as usize * WIDTH + x as usize;
        return self.mem[off];
    }
    
    fn write_pixel(&mut self, x: ByteVal, y: ByteVal, val: ByteVal) {
        let off = y as usize * WIDTH + x as usize;
        self.mem[off] = if val == 0 { 0 } else { 1 };
    }

    fn write_pixel_xor(&mut self, x: ByteVal, y: ByteVal, val: ByteVal) -> bool {
        let off = y as usize * WIDTH + x as usize;
        let val = if val == 0 { 0 } else { 1 };
        let cleared = self.mem[off] == 1 && val == 1;
        self.mem[off] ^= val;
        cleared
    }

    fn write_pixel_row(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal) {
        for i in 0..8 {
            self.write_pixel(x + i, y, rowval >> (7 - i));
        }
    }
    
    fn write_pixel_row_xor(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal) -> bool {
        let mut cleared = false;
        for i in 0..8 {
            let pixel_cleared = self.write_pixel_xor(x + i, y, (rowval >> (7 - i)) & 1);
            cleared = cleared || pixel_cleared;
        }
        cleared
    }
}
