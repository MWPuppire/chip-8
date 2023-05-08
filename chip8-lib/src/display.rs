#[cfg(feature = "std")]
use std::{vec, vec::Vec};

pub const LOWRES_SCREEN_WIDTH:   usize = 64;
pub const LOWRES_SCREEN_HEIGHT:  usize = 32;
pub const LOWRES_SCREEN_DIMENSIONS: (usize, usize) = (LOWRES_SCREEN_WIDTH, LOWRES_SCREEN_HEIGHT);
pub const HIGHRES_SCREEN_WIDTH:  usize = 128;
pub const HIGHRES_SCREEN_HEIGHT: usize = 64;
pub const HIGHRES_SCREEN_DIMENSIONS: (usize, usize) = (HIGHRES_SCREEN_WIDTH, HIGHRES_SCREEN_HEIGHT);
cfg_if::cfg_if! {
    if #[cfg(any(feature = "super-chip", feature = "xo-chip"))] {
        pub const SCREEN_WIDTH:  usize = HIGHRES_SCREEN_WIDTH;
        pub const SCREEN_HEIGHT: usize = HIGHRES_SCREEN_HEIGHT;
        pub const SCREEN_DIMENSIONS: (usize, usize) = HIGHRES_SCREEN_DIMENSIONS;
    } else {
        pub const SCREEN_WIDTH:  usize = LOWRES_SCREEN_WIDTH;
        pub const SCREEN_HEIGHT: usize = LOWRES_SCREEN_HEIGHT;
        pub const SCREEN_DIMENSIONS: (usize, usize) = LOWRES_SCREEN_DIMENSIONS;
    }
}

#[derive(Clone, Debug)]
pub struct Display {
    buffer: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    pub(crate) high_res: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn write_pixel_unchecked(&mut self, x: u8, y: u8) -> bool {
        let x = x as usize;
        let y = y as usize;
        let toggle = self.buffer[y][x];
        self.buffer[y][x] = !self.buffer[y][x];
        toggle
    }

    pub fn write_pixel(&mut self, x: u8, y: u8) -> bool {
        if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
            false
        } else {
            self.write_pixel_unchecked(x, y)
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "super-chip", feature = "xo-chip"))] {
            pub fn write_to_screen(&mut self, x: u8, y: u8) -> bool {
                if self.high_res {
                    if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
                        return false;
                    }
                    self.write_pixel_unchecked(x, y)
                } else {
                    let mut toggle = false;
                    let x = x << 1;
                    let y = y << 1;
                    if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
                        return false;
                    }
                    toggle |= self.write_pixel_unchecked(x + 0, y + 0);
                    toggle |= self.write_pixel_unchecked(x + 1, y + 0);
                    toggle |= self.write_pixel_unchecked(x + 0, y + 1);
                    toggle |= self.write_pixel_unchecked(x + 1, y + 1);
                    toggle
                }
            }
        } else {
            #[inline]
            pub fn write_to_screen(&mut self, x: u8, y: u8) -> bool {
                self.write_pixel(x, y)
            }
        }
    }

    pub fn read_pixel_unchecked(&self, x: u8, y: u8) -> bool {
        self.buffer[y as usize][x as usize]
    }

    pub fn read_pixel(&self, x: u8, y: u8) -> bool {
        if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
            false
        } else {
            self.read_pixel_unchecked(x, y)
        }
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
