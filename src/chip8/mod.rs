
pub mod types;
pub mod core;
pub mod memory;
pub mod display_buffer;
pub mod keyboard;

use chip8::core::{CPU, MemoryInterface, DisplayInterface, KeyboardInterface};

pub struct Chip8<'a> {
    cpu:     CPU<'a>
}

impl<'a> Chip8<'a> {
    pub fn new(mem: &'a mut MemoryInterface, display: &'a mut DisplayInterface) -> Chip8<'a> {
        Chip8 {
            cpu:     CPU::new(mem, display),
        }
    }
    

    pub fn cycle(&mut self) {
        println!("Cycle start");
        
        let opval = self.cpu.fetch_op();

        println!("OpVal: {:x?}", opval);

        self.cpu.decode_and_execute_op(opval);

        println!("Cycle end\n");
    }

}
