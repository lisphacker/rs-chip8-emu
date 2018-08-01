
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
use sdl2::rect::Rect;

use backends::Backend;
use chip8::core::{KeyboardInterface, DisplayInterface};
use chip8::core::{RcRefKeyboardInterface, RcRefDisplayInterface};
use chip8::display_buffer::DisplayBuffer;

pub struct IOState {
    key_pressed: [bool; 16],
    display_buffer: DisplayBuffer,
    display_changed: bool
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
    fn dimensions(&self) -> (usize, usize) {
        self.display_buffer.dimensions()
    }
    
    fn clear(&mut self) {
        self.display_changed = true;
        self.display_buffer.clear();
    }
    
    fn read_pixel(&self, x: u8, y: u8) -> u8 {
        self.display_buffer.read_pixel(x, y)
    }
    
    fn write_pixel(&mut self, x: u8, y: u8, val: u8) {
        self.display_changed = true;
        self.display_buffer.write_pixel(x, y, val);
    }
    
    fn write_pixel_xor(&mut self, x: u8, y: u8, val: u8) -> bool {
        self.display_changed = true;
        self.display_buffer.write_pixel_xor(x, y, val)
    }

    fn write_pixel_row(&mut self, x: u8, y : u8, rowval: u8) {
        self.display_changed = true;
        self.display_buffer.write_pixel(x, y, rowval);
    }
    
    fn write_pixel_row_xor(&mut self, x: u8, y : u8, rowval: u8) -> bool {
        self.display_changed = true;
        self.display_buffer.write_pixel_xor(x, y, rowval)
    }
}

pub struct SDL {
    canvas:      Canvas<Window>,
    event_pump:  EventPump,
    pub iostate: RcRefIOState
}

const PIXEL_WIDTH: usize = 10;
const PIXEL_HEIGHT: usize = 10;

impl SDL {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let display_buffer = DisplayBuffer::new();
        let sz = display_buffer.dimensions();
        
        let window = video_subsystem.window("rust-sdl2 demo: Video",
                                            (sz.0 * PIXEL_WIDTH) as u32,
                                            (sz.1 * PIXEL_HEIGHT) as u32)
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
                key_pressed: [false; 16],
                display_buffer: display_buffer,
                display_changed: true
            }))
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

    fn update_display(iostate: &RcRefIOState, canvas: &mut Canvas<Window>) {
        let mut io = iostate.lock().unwrap();

        if !io.display_changed {
            return;
        }

        let sz = io.dimensions();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 128, 0));

        for y in 0..sz.1 {
            for x in 0..sz.0 {
                if io.read_pixel(x as u8, y as u8) != 0 {
                    canvas.fill_rect(Rect::new((x * PIXEL_WIDTH) as i32,
                                               (y * PIXEL_HEIGHT) as i32,
                                               PIXEL_WIDTH as u32,
                                               PIXEL_HEIGHT as u32)).expect("canvas.fill_rectfailed");
                }
            }
        }
        canvas.present();
    }
}


impl Backend for SDL {
    fn get_keyboard_interface(&self) -> RcRefKeyboardInterface {
        self.iostate.clone()
    }
    
    fn get_display_interface(&self) -> RcRefDisplayInterface {
        self.iostate.clone()
    }

    fn run(&mut self) {
        'running: loop {
            let iostate = &self.iostate;
            let mut canvas = &mut self.canvas;
            
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

            SDL::update_display(iostate, &mut canvas);

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

