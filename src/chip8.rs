
use std::fs::File;
use std::io::Read;
use std::io::Result;

type Addr = u16;
type RegNum = u8;
type ByteVal = u8;

#[derive(Debug)]
struct OpVal(u8, u8, u8, u8);

#[derive(Debug)]
enum OpCode {
    ///// CHIP-8 opcodes /////

    UND,  // Undefined
    NOP,  // No-op
    
    // Syscall opcodes
    SYS(Addr), // Call RCA 1802 program at give address

    // Display opcodes
    CLS, // Clear the display

    // Flow opcodes
    RET,         // Return from subroutine
    JP(Addr),    // Jump to addr
    JPREL(Addr), // Jump to V0 + addr
    CALL(Addr),  // Call subroutine at addr

    // Conditional opcodes
    SEC(RegNum, ByteVal),  // Skip next instruction if reg == val
    SNEC(RegNum, ByteVal), // Skip next instruction if reg != val
    SE(RegNum, RegNum),    // Skip next instruction if reg1 == reg2
    SNE(RegNum, RegNum),   // Skip next instruction if reg1 != reg2

    // Assign opcodes
    LDC(RegNum, ByteVal), // reg <- val
    LD(RegNum, RegNum),   // reg1 <- reg2
    LDI(Addr),            // I <- addr

    LDDT(RegNum), // reg <- delay timer
    STDT(RegNum), // delay timer <- reg
    STST(RegNum), // sound timer <- reg

    LDTC(RegNum, ByteVal), // Wait for key and place key in reg

    LDSPRT(RegNum), // I <- sprite_addr[reg]

    STBCD(RegNum), // [I] <- bcd(reg)

    LDALL(ByteVal), // v0-vx <- [I]
    StALL(ByteVal), // [I] <- v0-vx

    // BitOp opcodes
    OR(RegNum, RegNum),  // reg1 <- reg1 | reg2
    AND(RegNum, RegNum), // reg1 <- reg1 & reg2
    XOR(RegNum, RegNum), // reg1 <- reg1 ^ reg2
    SHR(RegNum, RegNum), // reg1 <- reg2 >> 1
    SHL(RegNum, RegNum), // reg1 <- reg2 <- reg2 << 1

    // Math opcodes
    ADDC(RegNum, ByteVal), // reg1 <- reg1 + val
    ADD(RegNum, RegNum),   // reg1 <- reg1 + reg2
    ADDI(RegNum),          // I <- I + reg
    SUB(RegNum, RegNum),   // reg1 <- reg1 - reg2
    SUBN(RegNum, RegNum),  // reg1 <- reg2 - reg1

    // RNG opcodes
    RND(RegNum, ByteVal), // reg <- rnd_val & val

    // Display opcodes
    DRW(RegNum, RegNum, ByteVal),

    // Key opcodes
    SKP(RegNum), // Skip next instruction if key specified in reg is pressed
    SKNP(RegNum), // Skip next instruction if key specified in reg is not pressed

    

    ///// Chip-48 opcodes
}

const PROG_START_ADDR: usize = 0x200;

struct CPU {
    pc:   u16,
    vreg: [u8; 16],
    ireg: u16,
    dt:   u8,
    st:   u8,
}

impl CPU {
    fn new() -> Self {
        CPU {
            pc: PROG_START_ADDR as u16,
            vreg: [0; 16],
            ireg: 0,
            dt: 0,
            st: 0
        }
    }

    fn incr_pc(&mut self) {
        self.pc += 2;
    }

    fn fetch_op(&mut self, ba: &ByteAddressable) -> OpVal {
        let b0 = ba.read_byte(self.pc);
        let b1 = ba.read_byte(self.pc + 1);

        self.incr_pc();
        
        OpVal(b0 >> 4, b0 & 0xf, b1 >> 4, b1 & 0xf)
    }
    
    fn decode_op(opval: OpVal) -> OpCode {
        let OpVal(n0, n1, n2, n3) = opval;

        match n0 {
            0x0 => match n1 {
                0x0 if n2 == 0xe && n3 == 0x0 => OpCode::CLS,
                0x0 if n2 == 0xe && n3 == 0xe => OpCode::RET,
                _                             => OpCode::SYS(CPU::make_3nibble_addr(n1, n2, n3))
            }
            0x1 => OpCode::JP(CPU::make_3nibble_addr(n1, n2, n3)),
            0x2 => OpCode::CALL(CPU::make_3nibble_addr(n1, n2, n3)),
            _   => OpCode::UND
                
        }
    }

    fn execute_op(&self, op: OpCode, ba: &mut ByteAddressable) {
        match op {
            UND => eprintln!("Instruction could not be decoded"),
        };
    }

    fn make_3nibble_addr(n0: u8, n1: u8, n2: u8) -> u16 {
        ((n0 as u16) << 8) | ((n1 as u16) << 4) | (n2 as u16)
    }

    fn read_reg(&self, regnum: RegNum) -> ByteVal {
        self.vreg[regnum as usize]
    }

    fn write_reg(&mut self, regnum: RegNum, val: ByteVal) {
        self.vreg[regnum as usize] = val;
    }
}

trait ByteAddressable {
    fn read_byte(&self, Addr) -> ByteVal;
    fn write_byte(&mut self, Addr, ByteVal);
}
                 
struct Memory {
    mem: Vec<u8>
}

impl Memory {
    fn new(size: usize) -> Self {
        Memory {
            mem: vec![0; size]
        }
    }
}

impl ByteAddressable for Memory {
    fn read_byte(&self, addr: Addr) -> ByteVal {
        self.mem[addr as usize]
    }
    
    fn write_byte(&mut self, addr: Addr, val: ByteVal) {
        self.mem[addr as usize] = val;
    }
}
    
const MEM_SIZE: usize = 4096;


pub struct Chip8 {
    cpu: CPU,
    mem: Memory
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: CPU::new(),
            mem: Memory::new(MEM_SIZE)
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
        
        let opval = self.cpu.fetch_op(&self.mem);

        println!("OpVal: {:x?}", opval);

        let opcode = CPU::decode_op(opval);
        println!("OpCode: {:x?}", opcode);

        self.cpu.execute_op(opcode, &mut self.mem);

        println!("Cycle end\n");
    }

}
