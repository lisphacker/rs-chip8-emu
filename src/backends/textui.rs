
use std::sync::{Arc, Mutex};
use std::thread;

use std::time::Duration;

use sdl2;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use backends::Backend;
use chip8::core::{KeyboardInterface, DisplayInterface};
use chip8::core::{RcRefKeyboardInterface, RcRefDisplayInterface};

pub struct IOState {
    key_pressed: [bool; 16]
}

type RcRefIOState = Arc<Mutex<IOState>>;

impl KeyboardInterface for IOState {
    fn key_pressed(&self, key: u8) -> bool {
        self.key_pressed[key as usize]
    }
    
    fn wait_for_key(&self, key: u8) {
    }
}

impl DisplayInterface for IOState {
    fn dimensions(&self) -> (u8, u8) {
        (0, 0)
    }
    
    fn clear(&mut self) {
    }
    
    fn read_pixel(&self, x: u8, y: u8) -> u8 {
        0
    }
    
    fn write_pixel(&mut self, x: u8, y: u8, val: u8) {
    }
    
    fn write_pixel_xor(&mut self, x: u8, y: u8, val: u8) -> bool {
        false
    }

    fn write_pixel_row(&mut self, x: u8, y : u8, rowval: u8) {
    }
    
    fn write_pixel_row_xor(&mut self, x: u8, y : u8, rowval: u8) -> bool {
        false
    }
}

pub struct TextUI {
    pub iostate: RcRefIOState
}

impl TextUI {
    pub fn new() -> Self {
        TextUI {
            iostate: Arc::new(Mutex::new(IOState {
                key_pressed: [false; 16]
            }))
        }
    }

    pub fn run(&mut self) {
    }
}


impl Backend for TextUI {
    fn get_keyboard_interface(&self) -> RcRefKeyboardInterface {
        self.iostate.clone()
    }
    
    fn get_display_interface(&self) -> RcRefDisplayInterface {
        self.iostate.clone()
    }

    fn run(&mut self) {
    }
}
