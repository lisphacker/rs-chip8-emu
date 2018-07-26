
use chip8::types::Addr;
use chip8::types::RegNum;
use chip8::types::ByteVal;

#[derive(Debug)]
pub struct OpVal(u8, u8, u8, u8);

/*
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
 */

pub trait MemoryInterface {
    fn read_byte(&self, Addr) -> ByteVal;
    fn write_byte(&mut self, Addr, ByteVal);
}

pub trait DisplayInterface {
    fn clear(&self);
}
                 
pub const PROG_START_ADDR: usize = 0x200;

pub struct CPU<'a> {
    pc:   u16,
    vreg: [u8; 16],
    ireg: u16,
    dt:   u8,
    st:   u8,
    stack: Vec<Addr>,
        
    mem:     &'a mut MemoryInterface,
    display: &'a mut DisplayInterface
}

impl<'a> CPU<'a> {
    pub fn new(mem: &'a mut MemoryInterface, display: &'a mut DisplayInterface) -> Self {
        CPU {
            pc: PROG_START_ADDR as u16,
            vreg: [0; 16],
            ireg: 0,
            dt: 0,
            st: 0,
            stack: Vec::new(),

            mem: mem,
            display: display
        }
    }

    fn incr_pc(&mut self) {
        self.pc += 2;
    }

    pub fn fetch_op(&mut self) -> OpVal {
        let b0 = self.mem.read_byte(self.pc);
        let b1 = self.mem.read_byte(self.pc + 1);

        self.incr_pc();
        
        OpVal(b0 >> 4, b0 & 0xf, b1 >> 4, b1 & 0xf)
    }
    
    pub fn decode_and_execute_op(&mut self, opval: OpVal) {
        let OpVal(n0, n1, n2, n3) = opval;
        let addr = ((n1 as Addr) << 8) | ((n2 as Addr) << 4) | (n3 as Addr);
        let x = n1 as RegNum;
        let y = n2 as RegNum;
        let imm8 = ((n2 as ByteVal) << 4) | (n3 as ByteVal);
        let imm4 = n3 as ByteVal;
        

        match (n0, n1, n2, n3) {
            (0x0, 0x0, 0x0, 0x0) => self.op_undef(),
            (0x0, 0x0, 0xe, 0x0) => self.op_cls(),
            (0x0, 0x0, 0xe, 0xe) => self.op_ret(),
            (0x0,   _,   _,   _) => self.op_sys(addr),
            
            (0x1,   _,   _,   _) => self.op_jp(addr),
            (0x2,   _,   _,   _) => self.op_call(addr),
            (0x3,   _,   _,   _) => self.op_sec(x, imm8),
            (0x4,   _,   _,   _) => self.op_snec(x, imm8),
            (0x5,   _,   _, 0x0) => self.op_se(x, y),
            (0x6,   _,   _,   _) => self.op_ldc(x, imm8),
            (0x7,   _,   _,   _) => self.op_addc(x, imm8),


            (0x8,   _,   _, 0x0) => self.op_ld(x, y),
            (0x8,   _,   _, 0x1) => self.op_or(x, y),
            (0x8,   _,   _, 0x2) => self.op_and(x, y),
            (0x8,   _,   _, 0x3) => self.op_xor(x, y),
            (0x8,   _,   _, 0x4) => self.op_add(x, y),
            (0x8,   _,   _, 0x5) => self.op_sub(x, y),
            (0x8,   _,   _, 0x6) => self.op_shr(x),
            (0x8,   _,   _, 0x7) => self.op_subn(x, y),
            (0x8,   _,   _, 0xe) => self.op_shl(x),

            (0x9,   _,   _, 0x0) => self.op_se(x, y),

            (0xa,   _,   _,   _) => self.op_ldi(addr),
            (0xb,   _,   _,   _) => self.op_jp_rel(addr),
            (0xc,   _,   _,   _) => self.op_rnd(x, imm8),
            (0xd,   _,   _,   _) => self.op_drw(x, y, imm4),
            
            (0xe,   _, 0x9, 0xe) => self.op_skp(x),
            (0xe,   _, 0xa, 0x1) => self.op_sknp(x),

            (0xf,   _, 0x0, 0x7) => self.op_lddt(x),
            (0xf,   _, 0x0, 0xa) => self.op_ldtc(x),
            (0xf,   _, 0x1, 0x5) => self.op_stdt(x),
            (0xf,   _, 0x1, 0x8) => self.op_stst(x),
            (0xf,   _, 0x1, 0xe) => self.op_addi(x),
            (0xf,   _, 0x2, 0x9) => self.op_ldsprt(x),
            (0xf,   _, 0x3, 0x3) => self.op_stbcd(x),
            (0xf,   _, 0x5, 0x5) => self.op_stall(x),
            (0xf,   _, 0x6, 0x5) => self.op_ldall(x),

            (  _,   _,   _,   _) => self.op_undef()
        };
    }

    // Call RCA 1802 program at give address
    fn op_sys(&mut self, addr: Addr) {
        eprintln!("RCA1802 calls are not supported!");
    }

    // Clear the display
    fn op_cls(&mut self) {
        self.display.clear();
    }

    // Return from subroutine
    fn op_ret(&mut self) {
    }

    // Jump to addr
    fn op_jp(&mut self, addr: Addr) {
    }

    // Jump to V0 + addr
    fn op_jp_rel(&mut self, addr: Addr) {
    }

    // Call subroutine at addr
    fn op_call(&mut self, addr: Addr) {
    }

    // Skip next instruction if reg == val
    fn op_sec(&mut self, vx: RegNum, val: ByteVal) {
    }

    // Skip next instruction if reg != val
    fn op_snec(&mut self, vx: RegNum, val: ByteVal) {
    }

    // Skip next instruction if reg1 == reg2
    fn op_se(&mut self, vx: RegNum, vy: RegNum) {
    }

    // Skip next instruction if reg1 != reg2
    fn op_sne(&mut self, vx: RegNum, vy: RegNum) {
    }

    // Load register with val
    fn op_ldc(&mut self, vx: RegNum, val: ByteVal) {
    }

    // Load register from another register
    fn op_ld(&mut self, vx: RegNum, vy: RegNum) {
    }

    // Load IREG with address
    fn op_ldi(&mut self, addr: Addr) {
    }

    // Load register from delay timer
    fn op_lddt(&mut self, vx: RegNum) {
    }

    // Store register into delay timer
    fn op_stdt(&mut self, vx: RegNum) {
    }

    // Store register into sound timer
    fn op_stst(&mut self, vx: RegNum) {
    }

    // Wait for key and place key in reg
    fn op_ldtc(&mut self, vx: RegNum) {
    }

    // Load IREG with sprite address of character in vx
    fn op_ldsprt(&mut self, vx: RegNum) {
    }

    // Store BCD representation of value in vx to [IREG], [IREG+1] and [IREG+2]
    fn op_stbcd(&mut self, vx: RegNum) {
    }

    // Load registers v0-vx from [i]
    fn op_ldall(&mut self, vx: RegNum) {
    }

    //Store registers v0-vx to [i]
    fn op_stall(&mut self, vx: RegNum) {
    }

    // vx <- vx | vy
    fn op_or(&mut self, vx: RegNum, vy: RegNum) {
    }

    // vx <- vx & vy
    fn op_and(&mut self, vx: RegNum, vy: RegNum) {
    }

    // vx <- vx & vy
    fn op_xor(&mut self, vx: RegNum, vy: RegNum) {
    }

    // vx <- vx >> 1
    fn op_shr(&mut self, vx: RegNum) {
    }

    // vx <- vx << 1
    fn op_shl(&mut self, vx: RegNum) {
    }

    // vx <- vx + val
    fn op_addc(&mut self, vx: RegNum, val: ByteVal) {
    }

    // vx <- vx + vy
    fn op_add(&mut self, vx: RegNum, vy: RegNum) {
    }
    // IREG <- IREG + vx
    fn op_addi(&mut self, vx: RegNum) {
    }

    // vx <- vx - vy
    fn op_sub(&mut self, vx: RegNum, vy: RegNum) {
    }

    // vx <- vy - vx
    fn op_subn(&mut self, vx: RegNum, vy: RegNum) {
    }

    // vx <- rnd_val & val
    fn op_rnd(&mut self, vx: RegNum, val: ByteVal) {
    }

    // Draw
    fn op_drw(&mut self, vx: RegNum, vy: RegNum, val: ByteVal) {
    }

    // Skip next instruction if key specified in reg is pressed
    fn op_skp(&mut self, vx: RegNum) {
    }

    // Skip next instruction if key specified in reg is not pressed
    fn op_sknp(&mut self, vx: RegNum) {
    }

    fn op_undef(&mut self) {
    }

    /*
    
    fn execute_op(&self, op: OpCode, ba: &mut MemoryInterface) {
        match op {
            UND       => eprintln!("Instruction could not be decoded"),
            SYS(addr) => 
        };
    }

    fn make_3nibble_addr(n0: u8, n1: u8, n2: u8) -> u16 {
        ((n0 as u16) << 8) | ((n1 as u16) << 4) | (n2 as u16)
    }
    */

    fn read_reg(&self, regnum: RegNum) -> ByteVal {
        self.vreg[regnum as usize]
    }

    fn write_reg(&mut self, regnum: RegNum, val: ByteVal) {
        self.vreg[regnum as usize] = val;
    }

    fn push_addr(&mut self, addr: Addr) {
        self.stack.push(addr);
    }

    fn pop_addr(&mut self) -> Addr {
        self.stack.pop().unwrap_or(0)
    }
}

