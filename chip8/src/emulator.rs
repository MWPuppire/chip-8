use chip8_lib::{Error, CPU, Chip8Mode};
use crate::log;

use std::path::Path;
use std::fs;
use std::time::Duration;

use instant::Instant;

#[derive(Clone, Debug)]
pub struct Emulator {
    cpu: CPU,
    has_rom: bool,
    breakpoints: Vec<u16>,
    last_time: Instant,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: CPU::new(Chip8Mode::Cosmac),
            has_rom: false,
            breakpoints: vec!(),
            last_time: Instant::now(),
        }
    }

    fn step(&mut self) -> Duration {
        let last = self.last_time;
        self.last_time = Instant::now();
        last.elapsed()
    }

    pub fn update(&mut self) -> Result<(), Error> {
        let dt = self.step();
        if !self.has_rom {
            return Err(Error::NoRomLoaded);
        }

        self.cpu.emulate_breakpoints(dt, &self.breakpoints[..])?;
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

    pub fn display_buffer(&self, scale_factor: usize) -> Vec<u32> {
        self.cpu.screen.to_buffer(scale_factor, scale_factor)
    }

    pub fn key_press(&mut self, key: u8, press: bool) {
        if press {
            self.cpu.press_key(key);
        } else {
            self.cpu.release_key(key);
        }
    }
}
