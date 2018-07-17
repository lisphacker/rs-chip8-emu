
mod chip8;

use chip8::Chip8;


fn main() {
    let mut chip8 = Chip8::new();
    
    if chip8.load_file("programs/games/TICTAC").is_err() {
        println!("Unable to load ROM");
        return;
    }

    loop {
        chip8.cycle();
    }
}
