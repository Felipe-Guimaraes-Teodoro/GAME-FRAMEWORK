//use tiny_game_framework::*;

use tiny_game_framework::{EventLoop, Vector2D};
use gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Line, Matrix4x4, Vector3D};
use tiny_game_framework::Vector4D;

fn main() {
    let resolution = Vector2D::new(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let c = Circle::new(16, Vector3D::ZERO, 50., resolution, Vector4D::ZERO+1);
    c.add_to_renderer("circle", &mut renderer);

    let l = Line::new(Vector3D::ZERO, Vector3D::new(resolution.x, resolution.y, 0.), 50., resolution, Vector4D::ZERO+1.);
    l.add_to_renderer("line", &mut renderer);

    let mut dt = 0.;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

        unsafe {
            Clear(COLOR_BUFFER_BIT);

            renderer.draw();
        }

        dt = now.elapsed().as_secs_f32();
    }
}
