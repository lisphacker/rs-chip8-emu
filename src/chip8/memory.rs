
use std::fs::File;
use std::io::Read;
use std::io::Result;

use chip8::types::Addr;
use chip8::types::ByteVal;

use chip8::core::MemoryInterface;
use chip8::core::PROG_START_ADDR;

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
            self.mem[PROG_START_ADDR + i] = byte_vec[i];
        }
        
        Ok(())
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
