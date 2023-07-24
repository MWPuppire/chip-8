#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_big_array::BigArray;

pub const LOWRES_SCREEN_WIDTH: usize = 64;
pub const LOWRES_SCREEN_HEIGHT: usize = 32;
pub const LOWRES_SCREEN_DIMENSIONS: (usize, usize) = (LOWRES_SCREEN_WIDTH, LOWRES_SCREEN_HEIGHT);
pub const HIGHRES_SCREEN_WIDTH: usize = 128;
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Display {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    buffer: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    pub(crate) high_res: bool,
}

impl Display {
    #[inline]
    pub fn new() -> Display {
        Display {
            buffer: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
            high_res: false,
        }
    }

    pub fn write_pixel_unchecked(&mut self, x: u8, y: u8) -> bool {
        let x = x as usize;
        let y = y as usize;
        let pos = x + y * SCREEN_WIDTH;
        let toggle = self.buffer[pos];
        self.buffer[pos] = !self.buffer[pos];
        toggle
    }

    #[inline]
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

    #[inline]
    pub fn read_pixel_unchecked(&self, x: u8, y: u8) -> bool {
        let pos = x as usize + (y as usize) * SCREEN_WIDTH;
        self.buffer[pos]
    }

    #[inline]
    pub fn read_pixel(&self, x: u8, y: u8) -> bool {
        if x >= SCREEN_WIDTH as u8 || y >= SCREEN_HEIGHT as u8 {
            false
        } else {
            self.read_pixel_unchecked(x, y)
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer.fill(false);
    }

    #[cfg(feature = "alloc")]
    pub fn to_buffer(&self, scale_x: usize, scale_y: usize) -> Vec<u32> {
        if scale_x == 0 || scale_y == 0 {
            return vec![];
        }
        let mut out = Vec::with_capacity(scale_x * scale_y * SCREEN_WIDTH * SCREEN_HEIGHT);
        for row in self.buffer.chunks(SCREEN_WIDTH) {
            for _ in 0..scale_y {
                out.extend(row.iter().flat_map(|set| {
                    vec![if *set { 0xFFFFFFFFu32 } else { 0x00000000u32 }; scale_x].into_iter()
                }));
            }
        }
        out
    }

    // TODO test that scrolling functions properly; this feels sorta hacked
    // together
    #[cfg(any(feature = "super-chip", feature = "xo-chip"))]
    pub fn scroll(&mut self, scroll_x: i8, scroll_y: i8) {
        let x_neg = scroll_x < 0;
        let y_neg = scroll_y < 0;
        let scroll_x = scroll_x.abs() as usize % SCREEN_WIDTH;
        let scroll_y = scroll_y.abs() as usize % SCREEN_HEIGHT;
        if scroll_x != 0 {
            for i in 0..SCREEN_HEIGHT {
                let row_start = i * SCREEN_WIDTH;
                if x_neg {
                    self.buffer.copy_within(
                        (row_start - scroll_x)..(row_start + SCREEN_WIDTH),
                        row_start,
                    );
                    self.buffer[(row_start + SCREEN_WIDTH - scroll_x)..(row_start + SCREEN_WIDTH)]
                        .fill(false);
                } else {
                    self.buffer.copy_within(
                        row_start..(row_start + SCREEN_WIDTH - scroll_x),
                        row_start + scroll_x,
                    );
                    self.buffer[row_start..(row_start + scroll_x)].fill(false);
                }
            }
        }
        if scroll_y != 0 {
            for i in 0..SCREEN_WIDTH {
                if y_neg {
                    for j in (SCREEN_HEIGHT - 1)..=0 {
                        self.buffer[j * SCREEN_WIDTH + i] = self
                            .buffer
                            .get(
                                j.checked_sub(scroll_y).unwrap_or(SCREEN_HEIGHT) * SCREEN_WIDTH + i,
                            )
                            .copied()
                            .unwrap_or(false);
                    }
                } else {
                    for j in 0..SCREEN_HEIGHT {
                        self.buffer[j * SCREEN_WIDTH + i] = self
                            .buffer
                            .get((j + scroll_y) * SCREEN_WIDTH + i)
                            .copied()
                            .unwrap_or(false);
                    }
                }
            }
        }
    }
}

impl Default for Display {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
