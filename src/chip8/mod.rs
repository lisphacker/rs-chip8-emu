
pub mod types;
pub mod core;
pub mod memory;
pub mod display_buffer;

use std::time::{Instant, Duration};

use chip8::core::{CPU, MemoryInterface, RcRefDisplayInterface, RcRefKeyboardInterface};

pub struct Chip8<'a> {
    cpu:            CPU<'a>,
    last_tick_time: Instant,
    period:         Duration
}

impl<'a> Chip8<'a> {
    pub fn new(mem: &'a mut MemoryInterface,
               display: &'a RcRefDisplayInterface,
               keyboard: &'a RcRefKeyboardInterface) -> Chip8<'a> {
        Chip8 {
            cpu:            CPU::new(mem, display, keyboard),
            last_tick_time: Instant::now(),
            period:         Duration::from_nanos(16666666)
        }
    }
    

    pub fn cycle(&mut self) {
        //println!("Cycle start");
        
        let opval = self.cpu.fetch_op();

        //println!("OpVal: {:x?}", opval);

        self.cpu.decode_and_execute_op(opval);

        let time = Instant::now();
        if time.duration_since(self.last_tick_time) > self.period {
            self.cpu.decrement_timers();
            self.last_tick_time = time;
        }

        //println!("Cycle end\n");
    }

}
