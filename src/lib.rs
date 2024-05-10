mod graphics;
mod events;
use events::{EventHandler, EventLoop};
use glfw::Action;

pub fn run() {
    let mut el = EventLoop::new();

    while !el.window.should_close() {
        el.update();

        if el.is_key_down(glfw::Key::B) {
            println!("B is pressed");
        }

    }
}