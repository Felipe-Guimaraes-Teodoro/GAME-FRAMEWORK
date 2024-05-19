use tiny_game_framework::{EventLoop, Renderer};
use tiny_game_framework::glam::{Vec2, vec2};

fn main() {
    let resolution = vec2(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    while !el.window.should_close() {
        el.update();

        unsafe {
            renderer.draw(&el);
        }
    }
}
