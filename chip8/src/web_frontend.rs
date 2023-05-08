use std::rc::Rc;
use std::sync::mpsc::channel;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{WindowBuilder, Window};
use winit::platform::web::WindowBuilderExtWebSys;
use winit::dpi::LogicalSize;
use crate::Emulator;
use crate::SCALE_FACTOR;
use crate::log;
use chip8_lib::Error;
use chip8_lib::display::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn create_window<T>(target: &EventLoopWindowTarget<T>) -> Window {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let body = document.body().unwrap();
    body.append_child(&canvas).unwrap();
    WindowBuilder::new()
        .with_resizable(false)
        .with_inner_size(LogicalSize::new(
            (SCALE_FACTOR * SCREEN_WIDTH) as f64,
            (SCALE_FACTOR * SCREEN_HEIGHT) as f64
        ))
        .with_canvas(Some(canvas))
        .build(target).unwrap()
}

pub fn load_rom_file() -> Result<Emulator, Error> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let rom_input = document.get_element_by_id("rom-file").unwrap()
        .dyn_into::<web_sys::HtmlInputElement>().unwrap();
    let rom_input = Rc::new(rom_input);

    let (sender, receiver) = channel();

    let load_rom = Closure::wrap(Box::new(move |value| {
        let mut emu = Emulator::new();
        let buf = js_sys::ArrayBuffer::from(value);
        let buf = js_sys::Uint8Array::new(&buf);
        let vec = buf.to_vec();
        emu.load_rom(&vec[..]).unwrap();
        sender.send(emu).unwrap();
    }) as Box<dyn FnMut(JsValue)>);

    let err_cb = Closure::wrap(Box::new(move |err| {
        log!("{:?}", err);
    }) as Box<dyn FnMut(JsValue)>);

    let rom_captured = rom_input.clone();
    let input_callback = Closure::wrap(Box::new(move |_| {
        let files = rom_captured.files().unwrap();
        if files.length() == 0 {
            return;
        }
        let file = files.item(0).unwrap();
        let _ = file.array_buffer().then(&load_rom).catch(&err_cb);
    }) as Box<dyn FnMut(JsValue)>);
    rom_input.add_event_listener_with_callback(
        "change",
        input_callback.as_ref().unchecked_ref()
    ).unwrap();
    input_callback.forget();

    let out = receiver.recv();
    Ok(out.unwrap())
}
