
pub mod types;
pub mod core;
pub mod memory;
pub mod display_buffer;

use std::fs::File;
use std::io::Read;
use std::io::Result;

use chip8::core::CPU;
use chip8::core::MemoryInterface;
use chip8::core::DisplayInterface;
use chip8::core::PROG_START_ADDR;
use chip8::memory::MEM_SIZE;

pub struct Chip8<'a> {
    cpu:     CPU,
    mem:     &'a mut MemoryInterface,
    display: &'a mut DisplayInterface
}

impl<'a> Chip8<'a> {
    pub fn new(mem: &'a mut MemoryInterface, display: &'a mut DisplayInterface) -> Chip8<'a> {
        Chip8 {
            cpu:     CPU::new(),
            mem:     mem,
            display: display
        }
    }
    
    pub fn load_file(&mut self, path: &str) -> Result<()> {
        let mut f = File::open(path)?;
        let mut byte_vec = Vec::new();

        let read_bytes = f.read_to_end(&mut byte_vec)?;

        let memlen = if read_bytes < MEM_SIZE - PROG_START_ADDR {
            read_bytes
        } else {
            MEM_SIZE - PROG_START_ADDR
        };

        for i in 0..memlen {
            self.mem.write_byte((PROG_START_ADDR + i) as u16, byte_vec[i]);
        }
        
        Ok(())
    }

    pub fn cycle(&mut self) {
        println!("Cycle start");
        
        let opval = self.cpu.fetch_op(self.mem);

        println!("OpVal: {:x?}", opval);

        self.cpu.decode_and_execute_op(opval, self.mem, self.display);

        println!("Cycle end\n");
    }

}
