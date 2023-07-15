use crate::{Emulator, SCALE_FACTOR};
use chip8_lib::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use chip8_lib::Error;
use std::sync::atomic::{AtomicBool, Ordering};
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
        .with_title("CHIP-8")
        .build(target)
        .unwrap()
}

pub async fn load_rom_file(emu: &mut Emulator) -> Result<(), Error> {
    static USED_ARG_ROM: AtomicBool = AtomicBool::new(false);
    if USED_ARG_ROM
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        // skip program name
        let mut args = std::env::args().skip(1);
        if let Some(path) = args.next() {
            info!("Using ROM file in argv for first initialization");
            return emu.load_rom_file(path);
        }
    }
    let file = rfd::AsyncFileDialog::new()
        .add_filter("CHIP-8 ROM", &["ch8"])
        .pick_file()
        .await
        .ok_or(Error::NoRomLoaded)?;
    let contents = file.read().await;
    emu.load_rom(&contents)
}
