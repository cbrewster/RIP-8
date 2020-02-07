mod chip;
mod cpu;
mod mmu;
mod keyboard;

use chip::Chip;
use mmu::{SCREEN_WIDTH, SCREEN_HEIGHT};
use minifb::{Key, Window, WindowOptions};
use std::io::Read;

const WINDOW_WIDTH: usize = 512;
const WINDOW_HEIGHT: usize = 256;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("USAGE: rip8 <path/to/rom>");
        return;
    }

    let rom = &args[1];

    let mut chip = Chip::new();

    for (i, byte) in std::fs::File::open(rom).unwrap().bytes().enumerate() {
        chip.mmu.write_byte((0x200 + i) as u16, byte.unwrap());
    }

    let mut window = Window::new(
        "RIP-8",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip.keyboard.update(&window);
        for _ in 0..10 {
            chip.execute();
        }
        chip.decrement_timers();
        window.update_with_buffer(chip.mmu.display_buffer(), SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}
