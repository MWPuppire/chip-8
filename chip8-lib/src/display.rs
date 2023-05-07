pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[cfg(feature = "std")]
use std::{vec, vec::Vec};

pub struct Display {
    buffer: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn write_pixel(&mut self, x: u8, y: u8) -> bool {
        let x = (x & 63) as usize;
        let y = (y & 31) as usize;
        let toggle = self.buffer[y][x];
        self.buffer[y][x] = !self.buffer[y][x];
        toggle
    }

    pub fn read_pixel(&self, x: u8, y: u8) -> bool {
        let x = (x & 63) as usize;
        let y = (y & 31) as usize;
        self.buffer[y][x]
    }

    pub fn clear(&mut self) {
        self.buffer.fill([false; SCREEN_WIDTH]);
    }

    #[cfg(feature = "std")]
    pub fn to_buffer(&self, scale_x: usize, scale_y: usize) -> Vec<u32> {
        if scale_x == 0 || scale_y == 0 {
            return vec![];
        }
        let mut out = Vec::with_capacity(scale_x * scale_y * SCREEN_WIDTH * SCREEN_HEIGHT);
        for row in self.buffer.iter() {
            for _ in 0..scale_y {
                out.extend(row.into_iter().flat_map(|set|
                    vec![if *set { 0xFFFFFFFFu32 } else { 0x00000000u32 }; scale_x].into_iter()
                ));
            }
        }
        out
    }
}
