
extern crate sdl2;
extern crate tui;
extern crate clap;

use std::thread;
use std::sync::mpsc::{channel, Receiver};
use clap::{Arg, App};

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
        chip8.cycle();
        match rx.try_recv() {
            Ok(Msg::Exit) => break,
            _             => {}
        }
    }
}

struct Opts {
    backend: backends::BackendType,
    rom_file: String
}

fn parse_args() -> Opts {
    let matches = App::new("rs-chip8-emu")
        .version("0.1.0")
        .author("Gautham Ganapathy <gauthamg@gmail.com>")
        .about("CHIP-8 simulator")
        .arg(Arg::with_name("backend")
             .short("b")
             .long("backend")
             .value_name("BACKEND")
             .help("Select a backend")
             .takes_value(true)
             .possible_value("sdl")
             .possible_value("text")
             .default_value("sdl"))
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .required(true)
             .index(1)
             .default_value("programs/games/TICTAC"))
        .get_matches();
    Opts {
        backend: if matches.value_of("backend").expect("Unknown backend") == "sdl" {
            backends::BackendType::SDL
        } else {
            backends::BackendType::TUI
        },
        rom_file: matches.value_of("INPUT").unwrap().into()
    }
}

fn main() {
    let opts = parse_args();
    let mut ref_backend = backends::get_backend(opts.backend);
    let mut backend = ref_backend.as_mut();
    
    let keyboard: RcRefKeyboardInterface = backend.get_keyboard_interface();
    let display: RcRefDisplayInterface = backend.get_display_interface();
    
    let (tx, rx) = channel();

    let sim_thread = thread::spawn(move || { simulation_thread(display, keyboard, rx); });

    backend.run();

    tx.send(Msg::Exit).expect("Unable to send exit message");
    sim_thread.join().expect("Failed to wait for simulation thread");
}
