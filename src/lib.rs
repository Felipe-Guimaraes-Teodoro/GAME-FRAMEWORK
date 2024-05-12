mod graphics;
mod events;
mod utils;
use events::EventLoop;
use graphics::{test_scene, Mesh, Renderer};
use utils::*;

pub fn run() {
    let mut el = EventLoop::new();
    let mut renderer = Renderer::new();
    
    test_scene(&mut renderer);

    while !el.window.should_close() {
        el.update();
        renderer.update();

        if el.is_key_down(glfw::Key::I) {
            println!("{:?}", renderer.meshes);
        }

        unsafe {
            renderer.draw();
        }
    }
}