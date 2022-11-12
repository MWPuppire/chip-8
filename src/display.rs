pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;

pub struct Display {
    buffer: [[bool; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
}

impl Display {
    pub fn new() -> Display {
        Display {
            buffer: [[false; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
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
        self.buffer.fill([false; SCREEN_WIDTH as usize]);
    }

    pub fn to_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|row| row.map(|set|
            if set { 0xFFFFFFFFu32 } else { 0x00000000u32 }
        )).flatten().collect()
    }
}
