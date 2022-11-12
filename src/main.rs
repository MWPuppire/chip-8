extern crate enum_map;
extern crate rand;
extern crate minifb;

pub mod cpu;
pub mod common;
pub mod register;
pub mod display;
pub mod instruction;
pub mod font;
pub mod window;
pub mod emulator;

use crate::emulator::Emulator;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No ROM file provided");
        return;
    }

    let mut emu = Emulator::new().unwrap();
    let rom_path = Path::new(&args[1]);
    emu.load_rom(rom_path).unwrap();
    loop {
        emu.update().unwrap();
    }
}
