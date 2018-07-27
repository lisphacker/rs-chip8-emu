
extern crate cursive;

mod chip8;
mod backends;

use chip8::Chip8;
use chip8::memory::Memory;
use chip8::display_buffer::DisplayBuffer;

use backends::curses::Curses;

fn main() {
    let mut mem = Memory::new();
    let mut display = DisplayBuffer::new();
    
    if mem.load_file("programs/games/TICTAC").is_err() {
        println!("Unable to load ROM");
        return;
    }

    let mut backend = Curses::new();
    
    let mut chip8 = Chip8::new(&mut mem, &mut display, &mut backend);
    
    loop {
        chip8.cycle();
        backend.step();
    }
}
