
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

use chip8::core::{KeyboardInterface, DisplayInterface};

pub struct IOState {
}

type RcRefIOState = Arc<Mutex<IOState>>;

impl KeyboardInterface for IOState {
    fn key_pressed(&self, key: u8) -> bool {
        false
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

pub struct SDL {
    canvas:      Canvas<Window>,
    event_pump:  EventPump,
    pub iostate: RcRefIOState
}

impl SDL {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();
        
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        
        SDL {
            canvas: canvas,
            event_pump: event_pump,
            iostate: Arc::new(Mutex::new(IOState {}))
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    _ => {}
                }
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

