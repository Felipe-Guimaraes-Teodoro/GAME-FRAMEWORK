mod graphics;
mod events;
mod utils;
use events::EventLoop;
use graphics::Renderer;
use utils::*;

pub fn run() {
    let mut el = EventLoop::new();
    let mut renderer = Renderer::new();

    while !el.window.should_close() {
        el.update();
        renderer.update();

        unsafe {
            renderer.draw();
        }
    }
}