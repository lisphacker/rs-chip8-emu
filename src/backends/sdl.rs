
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
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();
        
        SDL {
            canvas: canvas,
            event_pump: event_pump,
            iostate: Arc::new(Mutex::new(IOState {
                key_pressed: [false; 16]
            }))
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            let iostate = &self.iostate;
            
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        SDL::process_keycode(keycode, iostate, true);
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        SDL::process_keycode(keycode, iostate, false);
                    }
                    _ => {}
                }
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn process_keycode(key: Keycode, iostate: &RcRefIOState, key_state: bool) {
        let mut io = iostate.lock().unwrap();
        
        match key {
            Keycode::Num0 => io.key_pressed[0] = key_state,
            Keycode::Num1 => io.key_pressed[1] = key_state,
            Keycode::Num2 => io.key_pressed[2] = key_state,
            Keycode::Num3 => io.key_pressed[3] = key_state,
            Keycode::Num4 => io.key_pressed[4] = key_state,
            Keycode::Num5 => io.key_pressed[5] = key_state,
            Keycode::Num6 => io.key_pressed[6] = key_state,
            Keycode::Num7 => io.key_pressed[7] = key_state,
            Keycode::Num8 => io.key_pressed[8] = key_state,
            Keycode::Num9 => io.key_pressed[9] = key_state,
            Keycode::A => io.key_pressed[10] = key_state,
            Keycode::B => io.key_pressed[11] = key_state,
            Keycode::C => io.key_pressed[12] = key_state,
            Keycode::D => io.key_pressed[13] = key_state,
            Keycode::E => io.key_pressed[14] = key_state,
            Keycode::F => io.key_pressed[15] = key_state,
            _          => {}
                
        };
    }
}

