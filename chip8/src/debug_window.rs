use winit::window::{Window, WindowId};
use winit::event::WindowEvent;
use winit::event_loop::EventLoopWindowTarget;

#[cfg(target_arch = "wasm32")]
extern crate web_sys;

pub struct DebugWindow {
    window: Option<Window>,
}

impl DebugWindow {
    pub fn new() -> Self {
        DebugWindow {
            window: None,
        }
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
    pub fn handle_event<'a>(&mut self, event: WindowEvent<'a>) {
        match event {
            WindowEvent::CloseRequested => self.close(),
            _ => {},
        }
    }
    pub fn put_text<'a>(&mut self, _: &'a str) {
        // TO-DO
    }

    #[cfg(target_arch = "wasm32")]
    fn insert_canvas(window: &Window) {
        use winit::platform::web::WindowExtWebSys;
        let canvas = window.canvas();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body.unwrap();
        body.append_child(&canvas).unwrap();
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn insert_canvas(_: &Window) {
        unimplemented!();
    }

    #[cfg(target_arch = "wasm32")]
    fn remove_canvas(window: Window) {
        use winit::platform::web::WindowExtWebSys;
        let canvas = window.canvas();
        canvas.remove();
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn remove_canvas(_: Window) {
        unimplemented!();
    }
}
