use chip8_lib::Error;
use chip8_lib::CPU;
use crate::window::Window;
use crate::log;

use std::path::Path;
use std::fs;

use instant::Instant;

pub struct Emulator {
    cpu: CPU,
    has_rom: bool,
    breakpoints: Vec<u16>,
    window: Window,
    last_time: Instant,
}

impl Emulator {
    pub fn try_new() -> Result<Emulator, Error> {
        Ok(Emulator {
            cpu: CPU::new(),
            has_rom: false,
            breakpoints: vec!(),
            window: Window::try_new()?,
            last_time: Instant::now(),
        })
    }

    fn step(&mut self) -> f64 {
        let last = self.last_time;
        self.last_time = Instant::now();
        last.elapsed().as_secs_f64()
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let dt = self.step();
        if !self.has_rom {
            return Err(Error::NoRomLoaded);
        }

        self.window.render_display(&self.cpu);
        for key in 0x0..0xF {
            if self.window.key_pressed(key) {
                self.cpu.press_key(key);
            } else if self.window.key_released(key) {
                self.cpu.release_key(key);
            }
        }

        self.cpu.emulate_until(dt, &self.breakpoints[..])?;
        if self.cpu.should_beep() {
            // TO-DO actually beep
            log!("beep!");
        }
        Ok(())
    }

    pub fn load_rom_file(&mut self, file: &Path) -> Result<(), Error> {
        let contents = fs::read(file);
        if let Ok(contents) = contents {
            self.cpu.load_rom(&contents[..])?;
            self.has_rom = true;
            Ok(())
        } else {
            Err(Error::InvalidFile)
        }
    }

    pub fn load_rom(&mut self, contents: &[u8]) -> Result<(), Error> {
        self.cpu.load_rom(contents)?;
        self.has_rom = true;
        Ok(())
    }
}