use tiny_game_framework::*;

fn main() {
    let mut el = EventLoop::new(800, 600);
    let mut renderer = Renderer::new();
    
    test_scene(&mut renderer);

    while !el.window.should_close() {
        el.update();
        renderer.update();

        unsafe {
            renderer.draw();
        }
    }
}