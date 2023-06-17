use crate::log;
use crate::Emulator;
use crate::SCALE_FACTOR;
use chip8_lib::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_lib::Error;
use std::env;
use std::path::Path;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder};

pub fn create_window<T>(target: &EventLoopWindowTarget<T>) -> Window {
    WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(
            (SCALE_FACTOR * SCREEN_WIDTH) as f64,
            (SCALE_FACTOR * SCREEN_HEIGHT) as f64,
        ))
        .build(target)
        .unwrap()
}

pub fn load_rom_file() -> Result<Emulator, Error> {
    let mut emu = Emulator::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log!("No ROM file provided");
        return Err(Error::NoRomLoaded);
    }
    let rom_path = Path::new(&args[1]);
    emu.load_rom_file(rom_path)?;
    Ok(emu)
}
