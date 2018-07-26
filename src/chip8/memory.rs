
use chip8::types::Addr;
use chip8::types::ByteVal;

use chip8::core::MemoryInterface;

pub const MEM_SIZE: usize = 4096;

pub struct Memory {
    mem: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: vec![0; MEM_SIZE]
        }
    }
}

impl MemoryInterface for Memory {
    fn read_byte(&self, addr: Addr) -> ByteVal {
        self.mem[addr as usize]
    }
    
    fn write_byte(&mut self, addr: Addr, val: ByteVal) {
        self.mem[addr as usize] = val;
    }
}
