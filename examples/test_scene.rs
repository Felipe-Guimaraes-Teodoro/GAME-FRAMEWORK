use tiny_game_framework::{Circle, EventLoop, Renderer, Vector2D, Vector3D, Vector4D};

fn main() {
    let resolution = Vector2D::new(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let c = Circle::new(16, Vector3D::ZERO, 0.1, resolution, Vector4D::new(1., 1., 1., 1.));
    c.add_to_renderer("my mesh", &mut renderer);

    while !el.window.should_close() {
        el.update();

        unsafe {
            renderer.draw();
        }
    }
}
