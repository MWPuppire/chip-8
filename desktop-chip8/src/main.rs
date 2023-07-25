extern crate chip8_core;
extern crate instant;
extern crate rfd;
extern crate softbuffer;
#[macro_use]
extern crate tracing;
extern crate winit;
extern crate tracing_subscriber;

pub mod debug_window;
pub mod emulator;

use softbuffer::GraphicsContext;
use std::sync::atomic::{AtomicBool, Ordering};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};
use crate::debug_window::DebugWindow;
use crate::emulator::Emulator;
use chip8_core::Error;
use chip8_core::display::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub const SCALE_FACTOR: usize = 8;

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

pub fn load_rom_file(emu: &mut Emulator) -> Result<(), Error> {
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
    let path = rfd::FileDialog::new()
        .add_filter("CHIP-8 ROM", &["ch8"])
        .pick_file()
        .ok_or(Error::NoRomLoaded)?;
    let contents = std::fs::read(path).unwrap();
    emu.load_rom(&contents)
}

fn main() {
    // TODO once debug window is further along, `Tee` the logs to the debug
    // window in addition to their normal output.
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    let mut graphics_context = unsafe { GraphicsContext::new(&window, &window) }.unwrap();
    let mut running = true;
    let mut debug = DebugWindow::new();
    let mut emu = Emulator::new();
    load_rom_file(&mut emu).unwrap();

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
                        Err(e) => {
                            error!("{}", e);
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
                    (SCREEN_WIDTH * full_scale) as u16,
                    (SCREEN_HEIGHT * full_scale) as u16,
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
