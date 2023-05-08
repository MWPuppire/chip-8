extern crate instant;
extern crate chip8_lib;
extern crate softbuffer;
extern crate winit;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
extern crate web_sys;
#[cfg(target_arch = "wasm32")]
extern crate js_sys;

pub mod emulator;
pub mod log;
pub mod debug_window;

#[cfg(not(target_arch = "wasm32"))]
mod desktop_frontend;
#[cfg(not(target_arch = "wasm32"))]
use desktop_frontend as frontend;

#[cfg(target_arch = "wasm32")]
mod web_frontend;
#[cfg(target_arch = "wasm32")]
use web_frontend as frontend;

use softbuffer::GraphicsContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use chip8_lib::Error;
use crate::emulator::Emulator;
use crate::debug_window::DebugWindow;

pub const SCALE_FACTOR: usize = 8;

fn main() {
    let event_loop = EventLoop::new();
    let window = frontend::create_window(&event_loop);
    let mut graphics_context = unsafe {
        GraphicsContext::new(&window, &window)
    }.unwrap();
    let mut running = true;
    let mut debug = DebugWindow::new();
    let mut emu = frontend::load_rom_file().unwrap();

    event_loop.run(move |event, target, control_flow| {
        match event {
            Event::MainEventsCleared => {
                if running {
                    let res = emu.update();
                    match res {
                        Err(Error::NoRomLoaded) => {
                            log!("No rom loaded");
                            if debug.is_open() {
                                running = false;
                            } else {
                                *control_flow = ControlFlow::Exit;
                            }
                        },
                        Err(Error::Breakpoint(_)) => {
                            debug.open(target);
                            debug.put_text("Breakpoint reached");
                            running = false;
                        },
                        Err(Error::UnknownOpcode(op)) => {
                            log!("Unknown opcode 0x{:04x}", op);
                            *control_flow = ControlFlow::Exit;
                        },
                        _ => {},
                    }
                }
                let dpi_scale = window.scale_factor();
                let full_scale = SCALE_FACTOR * dpi_scale as usize;
                let buf = emu.display_buffer(full_scale);
                graphics_context.set_buffer(&buf,
                    (chip8_lib::display::SCREEN_WIDTH * full_scale) as u16,
                    (chip8_lib::display::SCREEN_HEIGHT * full_scale) as u16
                );
            },
            Event::WindowEvent {
                event,
                window_id,
            } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        },
                        WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _
                        } => {
                            // hex key - physical key
                            // 1 2 3 C - 1 2 3 4
                            // 4 5 6 D - Q W E R
                            // 7 8 9 E - A S D F
                            // A 0 B F - Z X C V
                            use winit::event::{VirtualKeyCode, ElementState};
                            let pressed = input.state == ElementState::Pressed;
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Key1) => emu.key_press(0x1, pressed),
                                Some(VirtualKeyCode::Key2) => emu.key_press(0x2, pressed),
                                Some(VirtualKeyCode::Key3) => emu.key_press(0x3, pressed),
                                Some(VirtualKeyCode::Q)    => emu.key_press(0x4, pressed),
                                Some(VirtualKeyCode::W)    => emu.key_press(0x5, pressed),
                                Some(VirtualKeyCode::E)    => emu.key_press(0x6, pressed),
                                Some(VirtualKeyCode::A)    => emu.key_press(0x7, pressed),
                                Some(VirtualKeyCode::S)    => emu.key_press(0x8, pressed),
                                Some(VirtualKeyCode::D)    => emu.key_press(0x9, pressed),
                                Some(VirtualKeyCode::X)    => emu.key_press(0x0, pressed),
                                Some(VirtualKeyCode::Z)    => emu.key_press(0xA, pressed),
                                Some(VirtualKeyCode::C)    => emu.key_press(0xB, pressed),
                                Some(VirtualKeyCode::Key4) => emu.key_press(0xC, pressed),
                                Some(VirtualKeyCode::R)    => emu.key_press(0xD, pressed),
                                Some(VirtualKeyCode::F)    => emu.key_press(0xE, pressed),
                                Some(VirtualKeyCode::V)    => emu.key_press(0xF, pressed),
                                _ => {},
                            };
                        },
                        WindowEvent::DroppedFile(path) => {
                            emu.load_rom_file(&path).unwrap();
                        },
                        _ => {},
                    }
                } else if debug.is_open() && window_id == debug.id() {
                    debug.handle_event(event);
                }
            },
            _ => {},
        }
    });
}
