use chip8_lib::display;
use chip8_lib::Error;
use chip8_lib::CPU;

const PIXEL_SCALE: minifb::Scale = minifb::Scale::X8;
const WINDOW_WIDTH: u32 = display::SCREEN_WIDTH as u32;
const WINDOW_HEIGHT: u32 = display::SCREEN_HEIGHT as u32;

// hex key - physical key
// 1 2 3 C - 1 2 3 4
// 4 5 6 D - Q W E R
// 7 8 9 E - A S D F
// A 0 B F - Z X C V
const KEY_MAP: [minifb::Key; 16] = [
    minifb::Key::Key1, minifb::Key::Key2, minifb::Key::Key3, minifb::Key::Q,
    minifb::Key::W,    minifb::Key::E,    minifb::Key::A,    minifb::Key::S,
    minifb::Key::D,    minifb::Key::X,    minifb::Key::Z,    minifb::Key::C,
    minifb::Key::Key4, minifb::Key::R,    minifb::Key::F,    minifb::Key::V,
];

pub struct Window {
    window: minifb::Window,
}

impl Window {
    pub fn try_new() -> Result<Window, Error> {
        if let Ok(window) = minifb::Window::new(
            "CHIP-8",
            WINDOW_WIDTH as usize,
            WINDOW_HEIGHT as usize,
            minifb::WindowOptions {
                scale: PIXEL_SCALE,
                ..minifb::WindowOptions::default()
            }
        ) {
            Ok(Window {
                window: window,
            })
        } else {
            Err(Error::WindowFailure)
        }
    }

    pub fn render_display(&mut self, cpu: &CPU) {
        let buffer = cpu.screen.to_buffer();
        self.window.update_with_buffer(
            &buffer[..],
            WINDOW_WIDTH as usize,
            WINDOW_HEIGHT as usize
        ).unwrap();
    }

    pub fn key_pressed(&self, key: u8) -> bool {
        if key > 0xF {
            false
        } else {
            let key = KEY_MAP[key as usize];
            self.window.is_key_pressed(key, minifb::KeyRepeat::No)
        }
    }

    pub fn key_released(&self, key: u8) -> bool {
        if key > 0xF {
            false
        } else {
            let key = KEY_MAP[key as usize];
            self.window.is_key_released(key)
        }
    }
}
