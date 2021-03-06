
use std::sync::{Arc, Mutex};
use chip8::types::{Addr, ByteVal, RegNum, TimerVal};

#[derive(Debug)]
pub struct OpVal(ByteVal, ByteVal, ByteVal, ByteVal);

pub trait MemoryInterface {
    fn read_byte(&self, Addr) -> ByteVal;
    fn write_byte(&mut self, Addr, ByteVal);
}

pub trait DisplayInterface {
    fn dimensions(&self) -> (usize, usize);
    fn clear(&mut self);
    
    fn read_pixel(&self, x: ByteVal, y: ByteVal) -> ByteVal;
    
    fn write_pixel(&mut self, x: ByteVal, y: ByteVal, val: ByteVal);
    fn write_pixel_xor(&mut self, x: ByteVal, y: ByteVal, val: ByteVal) -> bool;

    fn write_pixel_row(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal);
    fn write_pixel_row_xor(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal) -> bool;
}

pub type RcRefDisplayInterface = Arc<Mutex<DisplayInterface + Send>>;

pub trait KeyboardInterface {
    fn key_pressed(&self, key: ByteVal) -> bool;
    fn wait_for_key(&self) -> ByteVal;
}

pub type RcRefKeyboardInterface = Arc<Mutex<KeyboardInterface + Send>>;
                 
pub const PROG_START_ADDR: Addr = 0x200;

pub struct CPU<'a> {
    pc:    Addr,
    vreg:  [ByteVal; 16],
    ireg:  Addr,
    dt:    TimerVal,
    st:    TimerVal,
    stack: Vec<Addr>,
        
    mem:      &'a mut MemoryInterface,
    display:  &'a RcRefDisplayInterface,
    keyboard: &'a RcRefKeyboardInterface
}

impl<'a> CPU<'a> {
    pub fn new(mem: &'a mut MemoryInterface,
               display: &'a RcRefDisplayInterface,
               keyboard: &'a RcRefKeyboardInterface) -> Self {
        CPU {
            pc:    PROG_START_ADDR,
            vreg:  [0; 16],
            ireg:  0,
            dt:    0,
            st:    0,
            stack: Vec::new(),

            mem:      mem,
            display:  display,
            keyboard: keyboard
        }
    }

    pub fn dump_reg(&self) {
        print!("V:[");
        for i in 0..16 {
            print!("{}:{:x?} ", i, self.vreg[i as usize]);
        }
        print!("] ");
        
        print!("I:{:x?},{} ", self.ireg, self.ireg);

        println!("");
        
    }

    fn set_pc(&mut self, addr: Addr) {
        self.pc = addr;
    }
    
    fn incr_pc(&mut self) {
        self.pc += 2;
    }

    pub fn fetch_op(&mut self) -> (OpVal, Addr) {
        let b0 = self.mem.read_byte(self.pc);
        let b1 = self.mem.read_byte(self.pc + 1);

        let pc = self.pc;
        self.incr_pc();
        
        (OpVal(b0 >> 4, b0 & 0xf, b1 >> 4, b1 & 0xf), pc)
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

            (0x9,   _,   _, 0x0) => self.op_sne(x, y),

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
        let mut display = self.display.lock().unwrap();
        (*display).clear();
    }

    // Return from subroutine
    fn op_ret(&mut self) {
        match self.stack.pop() {
            Some(addr) => self.set_pc(addr),
            _          => eprintln!("Call stack underflow")
        };
    }

    // Jump to addr
    fn op_jp(&mut self, addr: Addr) {
        self.pc = addr;
    }

    // Jump to V0 + addr
    fn op_jp_rel(&mut self, addr: Addr) {
        self.pc = self.vreg[0] as Addr + addr;
    }

    // Call subroutine at addr
    fn op_call(&mut self, addr: Addr) {
        self.stack.push(self.pc);
        self.pc = addr;
    }

    // Skip next instruction if reg == val
    fn op_sec(&mut self, vx: RegNum, val: ByteVal) {
        if self.vreg[vx] == val {
            self.incr_pc();
        }
    }

    // Skip next instruction if reg != val
    fn op_snec(&mut self, vx: RegNum, val: ByteVal) {
        if self.vreg[vx] != val {
            self.incr_pc();
        }
    }

    // Skip next instruction if reg1 == reg2
    fn op_se(&mut self, vx: RegNum, vy: RegNum) {
        if self.vreg[vx] == self.vreg[vy] {
            self.incr_pc();
        }
    }

    // Skip next instruction if reg1 != reg2
    fn op_sne(&mut self, vx: RegNum, vy: RegNum) {
        if self.vreg[vx] != self.vreg[vy] {
            self.incr_pc();
        }
    }

    // Load register with val
    fn op_ldc(&mut self, vx: RegNum, val: ByteVal) {
        self.vreg[vx] = val;
    }

    // Load register from another register
    fn op_ld(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] = self.vreg[vy];
    }

    // Load IREG with address
    fn op_ldi(&mut self, addr: Addr) {
        self.ireg = addr;
    }

    // Load register from delay timer
    fn op_lddt(&mut self, vx: RegNum) {
        self.vreg[vx] = self.dt as ByteVal;
    }

    // Store register into delay timer
    fn op_stdt(&mut self, vx: RegNum) {
        self.dt = self.vreg[vx] as TimerVal;
    }

    // Store register into sound timer
    fn op_stst(&mut self, vx: RegNum) {
        self.st = self.vreg[vx] as TimerVal;
    }

    // Wait for key and place key in reg
    fn op_ldtc(&mut self, vx: RegNum) {
        /*
        let keyboard = self.keyboard.lock().unwrap();
        self.vreg[vx] = keyboard.wait_for_key();
         */
        'running: loop {
            let keyboard = self.keyboard.lock().unwrap();
            for i in 0..16 {
                if keyboard.key_pressed(i) {
                    self.vreg[vx] = i as ByteVal;
                    break 'running;
                }
            }
        }
    }

    // Load IREG with sprite address of character in vx
    fn op_ldsprt(&mut self, vx: RegNum) {
        self.ireg = self.vreg[vx] as Addr * 5;
    }

    // Store BCD representation of value in vx to [IREG], [IREG+1] and [IREG+2]
    fn op_stbcd(&mut self, vx: RegNum) {
        let v = self.vreg[vx];
        self.mem.write_byte(self.ireg, v / 100);
        self.mem.write_byte(self.ireg + 1, (v / 10) % 10);
        self.mem.write_byte(self.ireg + 2, v % 10);
    }

    // Load registers v0-vx from [i]
    fn op_ldall(&mut self, vx: RegNum) {
        for i in 0..=vx {
            self.vreg[i] = self.mem.read_byte(self.ireg + i);
        }
    }

    //Store registers v0-vx to [i]
    fn op_stall(&mut self, vx: RegNum) {
        for i in 0..=vx {
            self.mem.write_byte(self.ireg + i, self.vreg[i]);
        }
    }

    // vx <- vx | vy
    fn op_or(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] |= self.vreg[vy];
    }

    // vx <- vx & vy
    fn op_and(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] &= self.vreg[vy];
    }

    // vx <- vx & vy
    fn op_xor(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] ^= self.vreg[vy];
    }

    // vx <- vx >> 1
    fn op_shr(&mut self, vx: RegNum) {
        self.vreg[0xf] = self.vreg[vx] & 1;
        self.vreg[vx] >>= 1;
    }

    // vx <- vx << 1
    fn op_shl(&mut self, vx: RegNum) {
        self.vreg[0xf] = self.vreg[vx] >> 7;
        self.vreg[vx] <<= 1;
    }

    // vx <- vx + val
    fn op_addc(&mut self, vx: RegNum, val: ByteVal) {
        //self.vreg[vx] += val;
        // Avoid overflow
        self.vreg[vx] = (self.vreg[vx] as u16 + val as u16) as u8;
    }

    // vx <- vx + vy
    fn op_add(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] += self.vreg[vy];
    }
    
    // IREG <- IREG + vx
    fn op_addi(&mut self, vx: RegNum) {
        self.ireg += self.vreg[vx] as Addr;
    }

    // vx <- vx - vy
    fn op_sub(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] -= self.vreg[vy];
    }

    // vx <- vy - vx
    fn op_subn(&mut self, vx: RegNum, vy: RegNum) {
        self.vreg[vx] = self.vreg[vy] - self.vreg[vx];
    }

    // vx <- rnd_val & val
    fn op_rnd(&mut self, vx: RegNum, val: ByteVal) {
        eprintln!("rnd not implemented");
    }

    // Draw
    fn op_drw(&mut self, vx: RegNum, vy: RegNum, val: ByteVal) {
        let x = self.vreg[vx];
        let y = self.vreg[vy];

        self.vreg[0xf] = 0;

        let mut display = self.display.lock().unwrap();
        
        for i in 0..val {
            let rowval = self.mem.read_byte(self.ireg + i as Addr);
            let cleared = (*display).write_pixel_row_xor(x, y + i, rowval);
            if cleared { self.vreg[0xf] = 1; }
        }
    }

    // Skip next instruction if key specified in reg is pressed
    fn op_skp(&mut self, vx: RegNum) {
        let keyboard = self.keyboard.lock().unwrap();
        if (*keyboard).key_pressed(self.vreg[vx]) {
            self.incr_pc();
        }
    }

    // Skip next instruction if key specified in reg is not pressed
    fn op_sknp(&mut self, vx: RegNum) {
        let keyboard = self.keyboard.lock().unwrap();
        if !(*keyboard).key_pressed(self.vreg[vx]) {
            self.incr_pc();
        }
    }

    fn op_undef(&mut self) {
        eprintln!("Unknown instruction!");
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }
}

