//use tiny_game_framework::*;

use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Quat, Vec3, Vec4};
use tiny_game_framework::{rand_betw, rand_vec2, rand_vec3, EventLoop};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Quad, Triangle, Line};
fn main() {
    let resolution = vec2(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();
    let q = Quad::new(vec3(0.5, 0.5, 0.), vec4(1., 0., 0., 1.));
    q.add_to_renderer("quad", &mut renderer);

    let c = Quad::new(vec3(0.1, 0.1, 0.), vec4(0., 1., 0., 1.));
    c.add_to_renderer("circle", &mut renderer);
    renderer.get_mesh_mut("circle").unwrap().position = vec3(0., 0., 0.);


    let l = Line::new(Vec3::ZERO, vec3(resolution.x, resolution.y, 0.), 50., Vec4::ZERO+1.);
    l.add_to_renderer("line", &mut renderer);

    // let tri = Triangle::new(0.5, Vector4D::new(0.2, 0.6, 0.9, 0.7));
    // tri.add_to_renderer("triangle", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

        unsafe {
            Clear(COLOR_BUFFER_BIT);
            let circle = renderer.get_mesh_mut("circle").unwrap();
            
            circle.add_position(vec3(0.0, el.event_handler.scroll.y*10., 0.));

            if el.is_key_down(glfw::Key::W) {
                circle.add_position(vec3(0.0, 100.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::S) {
                circle.add_position(vec3(0.0, -100.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::A) {
                circle.add_position(vec3(-100.0, 0.0, 0.0)*dt)
            }
            if el.is_key_down(glfw::Key::D) {
                circle.add_position(vec3(100.0, 0.0, 0.0)*dt)
            }


            let quad = renderer.get_mesh_mut("quad").unwrap();
            quad.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0 ,0.0, time);
            quad.scale += vec3(1.0 * dt, -1.0 * dt, 0.0);

            renderer.draw(&el);
        }

        time += dt;
        dt = now.elapsed().as_secs_f32();
    }
}
