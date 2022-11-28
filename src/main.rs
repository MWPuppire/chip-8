extern crate enum_map;
extern crate rand;
extern crate minifb;
extern crate instant;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
extern crate web_sys;
#[cfg(target_arch = "wasm32")]
extern crate js_sys;

pub mod cpu;
pub mod common;
pub mod register;
pub mod display;
pub mod instruction;
pub mod font;
pub mod window;
pub mod emulator;
pub mod log;

use crate::emulator::Emulator;

#[cfg(not(target_arch = "wasm32"))]
use std::env;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log!("No ROM file provided");
        return;
    }

    let mut emu = Emulator::try_new().unwrap();
    let rom_path = Path::new(&args[1]);
    emu.load_rom_file(rom_path).unwrap();
    loop {
        emu.update().unwrap();
    }
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let rom_input = document.get_element_by_id("rom-file").unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()?;
    let rom_input = Rc::new(rom_input);

    let emu = Rc::new(RefCell::new(Emulator::try_new().unwrap()));

    let emu_captured = emu.clone();
    let load_rom = Closure::wrap(Box::new(move |value| {
        let buf = js_sys::ArrayBuffer::from(value);
        let buf = js_sys::Uint8Array::new(&buf);
        let vec = buf.to_vec();
        emu_captured.borrow_mut().load_rom(&vec[..]).unwrap();
    }) as Box<dyn FnMut(JsValue)>);

    let rom_captured = rom_input.clone();
    let input_callback = Closure::wrap(Box::new(move |_| {
        let files = rom_captured.files().unwrap();
        if files.length() == 0 {
            return;
        }
        let file = files.item(0).unwrap();
        file.array_buffer().then(&load_rom);
    }) as Box<dyn FnMut(JsValue)>);
    rom_input.add_event_listener_with_callback("change", input_callback.as_ref().unchecked_ref())?;
    input_callback.forget();

    let update_cb: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let update_captured = update_cb.clone();

    *update_cb.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        emu.borrow_mut().update();
        let window = web_sys::window().unwrap();
        window.request_animation_frame(update_captured.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    }) as Box<dyn FnMut()>));
    window.request_animation_frame(update_cb.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;

    Ok(())
}
