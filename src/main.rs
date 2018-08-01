
extern crate sdl2;
extern crate tui;

use std::thread;
use std::sync::mpsc::{channel, Receiver};

mod chip8;
mod backends;

use chip8::Chip8;
use chip8::core::{RcRefKeyboardInterface, RcRefDisplayInterface};
use chip8::memory::Memory;
use chip8::display_buffer::DisplayBuffer;

enum Msg {
    Exit
}

fn simulation_thread(display: RcRefDisplayInterface, keyboard: RcRefKeyboardInterface, rx: Receiver<Msg>) {
    let mut mem = Memory::new();
    if mem.load_file("programs/games/TICTAC").is_err() {
        println!("Unable to load ROM");
        return;
    }
    
    let mut chip8 = Chip8::new(&mut mem, &display, &keyboard);

    loop {
        //chip8.cycle();
        match rx.try_recv() {
            Ok(Msg::Exit) => break,
            _             => {}
        }
    }
}

fn main() {
    let mut ref_backend = backends::get_backend(backends::BackendType::SDL);
    let mut backend = ref_backend.as_mut();
    
    let keyboard: RcRefKeyboardInterface = backend.get_keyboard_interface();
    let display: RcRefDisplayInterface = backend.get_display_interface();
    
    let (tx, rx) = channel();

    let sim_thread = thread::spawn(move || { simulation_thread(display, keyboard, rx); });

    backend.run();

    tx.send(Msg::Exit).expect("Unable to send exit message");
    sim_thread.join().expect("Failed to wait for simulation thread");
}
