use winit::event::WindowEvent;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowId};

#[cfg(target_arch = "wasm32")]
extern crate web_sys;

#[derive(Debug)]
pub struct DebugWindow {
    window: Option<Window>,
}

impl DebugWindow {
    pub fn new() -> Self {
        DebugWindow { window: None }
    }
    pub fn open<T>(&mut self, target: &EventLoopWindowTarget<T>) {
        if self.window.is_some() {
            return;
        }
        let window = Window::new(target).unwrap();
        if cfg!(target_arch = "wasm32") {
            Self::insert_canvas(&window);
        }
        self.window = Some(window);
    }
    pub fn close(&mut self) {
        if self.window.is_none() {
            return;
        }
        let window = self.window.take().unwrap();
        if cfg!(target_arch = "wasm32") {
            Self::remove_canvas(window);
        }
    }
    pub fn is_open(&self) -> bool {
        self.window.is_some()
    }
    pub fn id(&self) -> WindowId {
        if let Some(win) = &self.window {
            win.id()
        } else {
            unsafe { WindowId::dummy() }
        }
    }
    pub fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.close(),
            _ => {}
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            fn insert_canvas(window: &Window) {
                use winit::platform::web::WindowExtWebSys;
                let canvas = window.canvas();
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let body = document.body().unwrap();
                body.append_child(&canvas).unwrap();
            }
            fn remove_canvas(window: Window) {
                use winit::platform::web::WindowExtWebSys;
                let canvas = window.canvas();
                canvas.remove();
            }
        } else {
            fn insert_canvas(_: &Window) {
                unimplemented!();
            }
            fn remove_canvas(_: Window) {
                unimplemented!();
            }
        }
    }
}

impl Drop for DebugWindow {
    fn drop(&mut self) {
        if let Some(win) = self.window.take() {
            Self::remove_canvas(win);
        }
    }
}
