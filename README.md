# rs-chip8-emu

## Introduction
This is a CHIP-8 emulator in Rust. I started this off as a project to learn Rust programming. At the moment, it's able to run the TICTAC program.

The I/O is performed in the main thread while the simulation runs in a dedicated thread. It currently uses SDL as a backend, but the interface has been seperated and I'm looking at libraries for a text UI backend. At the moment, the issue is that I can't find a text UI library that will give seperate key-down and key-up events.

## References
* [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
* [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
