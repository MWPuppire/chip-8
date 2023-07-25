use winit::event::WindowEvent;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowId};

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
        self.window = Some(Window::new(target).unwrap());
    }
    pub fn close(&mut self) {
        self.window = None;
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
}
