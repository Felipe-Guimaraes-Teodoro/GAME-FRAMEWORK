//use tiny_game_framework::*;

use gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::{EventLoop, Vector2D};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Quad, Triangle, Line, Matrix4x4, Vector3D};
use tiny_game_framework::Vector4D;

fn main() {
    let resolution = Vector2D::new(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let c = Circle::new(16, 0.1, Vector4D::ZERO+1);
    c.add_to_renderer("circle", &mut renderer);
    renderer.get_mesh_mut("circle").unwrap().position = Vector3D::new(0., 0., 0.);

    let q = Quad::new(Vector3D::new(0.5, 0.5, 0.), Vector4D::new(1., 0., 0., 1.));
    q.add_to_renderer("quad", &mut renderer);

    let l = Line::new(Vector3D::ZERO, Vector3D::new(resolution.x, resolution.y, 0.), 50., Vector4D::ZERO+1.);
    l.add_to_renderer("line", &mut renderer);

    let tri = Triangle::new(0.5, Vector4D::new(0.2, 0.6, 0.9, 0.7));
    tri.add_to_renderer("triangle", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

        unsafe {
            Clear(COLOR_BUFFER_BIT);
            let circle = renderer.get_mesh_mut("circle").unwrap();
            
            circle.add_position(Vector3D::new(100.*dt, el.event_handler.scroll.y*10., 0.));
            
            if el.is_key_down(glfw::Key::W) {
                circle.add_position(Vector3D::new(0.0, 100.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::S) {
                circle.add_position(Vector3D::new(0.0, -100.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::A) {
                circle.add_position(Vector3D::new(-100.0, 0.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::D) {
                circle.add_position(Vector3D::new(100.0, 0.0, 0.0)*dt)
            }

            let quad = renderer.get_mesh_mut("quad").unwrap();
            quad.position.x = -200.;
            quad.position.y = f32::sin(time)*100.;

            renderer.draw(&el);
        }

        time += dt;
        dt = now.elapsed().as_secs_f32();
    }
}
