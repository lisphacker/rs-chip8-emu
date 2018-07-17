
use std::fs::File;
use std::io::Read;
use std::io::Result;

type Addr = u16;
type RegNum = u8;
type ByteVal = u8;

enum OpCode {
    ///// CHIP-8 opcodes /////
    
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

struct CPU {
    pc: u16,
    vreg: [u8; 16],
    ireg: u16,
    dt: u8,
    st: u8,
}

impl CPU {
    fn init(&mut self) {
        self.pc = 0x200;
    }

    //fn decode_op(&self) -> OpCode {}
}

const MEMSIZE: usize = 4096;

pub struct Chip8 {
    cpu: CPU,
    mem: [u8; MEMSIZE],
}

impl Chip8 {
    fn init(&mut self) {
        self.cpu.init();
        self.mem = [0; MEMSIZE];
    }

    fn load_file(&self, path: &str) -> Result<()> {
        let mut f = File::open(path)?;
        let mut byte_vec = Vec::new();

        let num_bytes = f.read_to_end(&mut byte_vec);

        println!("num_bytes = {:?}", num_bytes);

        Ok(())
    }
}
