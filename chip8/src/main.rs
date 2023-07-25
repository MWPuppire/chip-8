extern crate cfg_if;
extern crate chip8_lib;
extern crate instant;
extern crate rfd;
extern crate softbuffer;
#[macro_use]
extern crate tracing;
extern crate winit;

// TODO once debug window is further along, `Tee` the logs to the debug window
// in addition to their normal output.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        extern crate web_sys;
        extern crate js_sys;
        extern crate console_error_panic_hook;
        extern crate tracing_wasm;
        mod web_frontend;
        use web_frontend as frontend;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        pub fn init_logging() {
            tracing_wasm::set_as_global_default();
            console_error_panic_hook::set_once();
        }
    } else {
        extern crate tracing_subscriber;
        extern crate tokio;
        mod desktop_frontend;
        use desktop_frontend as frontend;

        pub fn init_logging() {
            tracing_subscriber::fmt::init();
        }
    }
}

pub mod debug_window;
pub mod emulator;

use softbuffer::GraphicsContext;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::debug_window::DebugWindow;
use crate::emulator::Emulator;
use chip8_lib::Error;

pub const SCALE_FACTOR: usize = 8;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(main))]
#[cfg_attr(not(target_arch = "wasm32"), tokio::main)]
async fn main() {
    init_logging();

    let event_loop = EventLoop::new();
    let window = frontend::create_window(&event_loop);
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut running = true;
    let mut debug = DebugWindow::new();
    let mut emu = Emulator::new();
    frontend::load_rom_file(&mut emu).await.unwrap();

    event_loop.run(move |event, target, control_flow| {
        match event {
            Event::MainEventsCleared => {
                if running {
                    let res = emu.update();
                    match res {
                        Err(Error::NoRomLoaded) => {
                            error!("No rom loaded");
                            if debug.is_open() {
                                running = false;
                            } else {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        Err(Error::Breakpoint(_)) => {
                            debug.open(target);
                            info!("Breakpoint reached");
                            running = false;
                        }
                        Err(Error::UnknownOpcode(op)) => {
                            error!("Unknown opcode 0x{:04x}", op);
                            *control_flow = ControlFlow::Exit;
                        }
                        Err(Error::NotDefined(op)) => {
                            error!("`{}` isn't defined for {}", op, emu.cpu.mode);
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                let dpi_scale = window.scale_factor();
                let full_scale = SCALE_FACTOR * dpi_scale as usize;
                let buf = emu.display_buffer(full_scale);
                graphics_context.set_buffer(
                    &buf,
                    (chip8_lib::display::SCREEN_WIDTH * full_scale) as u16,
                    (chip8_lib::display::SCREEN_HEIGHT * full_scale) as u16,
                );
            }
            Event::WindowEvent { event, window_id } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        } => {
                            // hex key - physical key
                            // 1 2 3 C - 1 2 3 4
                            // 4 5 6 D - Q W E R
                            // 7 8 9 E - A S D F
                            // A 0 B F - Z X C V
                            use winit::event::{ElementState, VirtualKeyCode};
                            let pressed = input.state == ElementState::Pressed;
                            match input.virtual_keycode {
                                Some(VirtualKeyCode::Key1) => emu.key_press(0x1, pressed),
                                Some(VirtualKeyCode::Key2) => emu.key_press(0x2, pressed),
                                Some(VirtualKeyCode::Key3) => emu.key_press(0x3, pressed),
                                Some(VirtualKeyCode::Q) => emu.key_press(0x4, pressed),
                                Some(VirtualKeyCode::W) => emu.key_press(0x5, pressed),
                                Some(VirtualKeyCode::E) => emu.key_press(0x6, pressed),
                                Some(VirtualKeyCode::A) => emu.key_press(0x7, pressed),
                                Some(VirtualKeyCode::S) => emu.key_press(0x8, pressed),
                                Some(VirtualKeyCode::D) => emu.key_press(0x9, pressed),
                                Some(VirtualKeyCode::X) => emu.key_press(0x0, pressed),
                                Some(VirtualKeyCode::Z) => emu.key_press(0xA, pressed),
                                Some(VirtualKeyCode::C) => emu.key_press(0xB, pressed),
                                Some(VirtualKeyCode::Key4) => emu.key_press(0xC, pressed),
                                Some(VirtualKeyCode::R) => emu.key_press(0xD, pressed),
                                Some(VirtualKeyCode::F) => emu.key_press(0xE, pressed),
                                Some(VirtualKeyCode::V) => emu.key_press(0xF, pressed),
                                _ => {}
                            };
                        }
                        WindowEvent::DroppedFile(path) => {
                            emu.load_rom_file(&path).unwrap();
                        }
                        _ => {}
                    }
                } else if debug.is_open() && window_id == debug.id() {
                    debug.handle_event(event);
                }
            }
            _ => {}
        }
    });
}
