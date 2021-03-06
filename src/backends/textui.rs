
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use std::time::Duration;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Group, Size, Direction, Rect};

use backends::Backend;
use chip8::types::ByteVal;
use chip8::core::{KeyboardInterface, DisplayInterface};
use chip8::core::{RcRefKeyboardInterface, RcRefDisplayInterface};
use chip8::display_buffer::{WIDTH, HEIGHT};

pub struct IOState {
    key_pressed: [bool; 16]
}

type RcRefIOState = Arc<Mutex<IOState>>;

impl KeyboardInterface for IOState {
    fn key_pressed(&self, key: ByteVal) -> bool {
        self.key_pressed[key as usize]
    }
    
    fn wait_for_key(&self) -> ByteVal {
        0
    }
}

impl DisplayInterface for IOState {
    fn dimensions(&self) -> (usize, usize) {
        (0, 0)
    }
    
    fn clear(&mut self) {
    }
    
    fn read_pixel(&self, x: ByteVal, y: ByteVal) -> ByteVal {
        0
    }
    
    fn write_pixel(&mut self, x: ByteVal, y: ByteVal, val: ByteVal) {
    }
    
    fn write_pixel_xor(&mut self, x: ByteVal, y: ByteVal, val: ByteVal) -> bool {
        false
    }

    fn write_pixel_row(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal) {
    }
    
    fn write_pixel_row_xor(&mut self, x: ByteVal, y : ByteVal, rowval: ByteVal) -> bool {
        false
    }
}

pub struct TextUI {
    pub iostate: RcRefIOState
}

impl TextUI {
    pub fn new() -> Self {
        let backend = RawBackend::new().unwrap();
        let mut terminal = Terminal::new(backend).unwrap();

        TextUI::init(&mut terminal).expect("Initialization failed");
        
        TextUI {
            iostate: Arc::new(Mutex::new(IOState {
                key_pressed: [false; 16]
            }))
        }
    }

    pub fn init(terminal: &mut Terminal<RawBackend>) -> Result<(), io::Error> {
        let size = Rect::new(0, 0, WIDTH as u16 + 2, HEIGHT as u16 + 2);
        
        Block::default()
            .title("BLK")
            .borders(Borders::ALL)
            .render(terminal, &size);
        
        terminal.draw()
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
        loop{}
    }
}
