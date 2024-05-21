//use tiny_game_framework::*;

use glam::Vec2;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Quat, Vec3, Vec4};
use tiny_game_framework::{rand_betw, rand_vec2, rand_vec3, EventLoop, InstanceData, InstanceMesh, Mesh, Vertex};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Quad, Triangle, Line};
fn main() {
    let resolution = vec2(2560., 1920.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    /* 
    let l = Line::new(Vec3::ZERO, vec3(resolution.x, resolution.y, 0.), 50., Vec4::ZERO+1.);
    l.add_to_renderer("line", &mut renderer);
    */
    let w = 200;
    let h = 200;
    let n = w*h;
    let data = {
        let mut positions = vec![];

        for i in 0..w {
            for j in 0..h {
                let ofs = Vec2::ONE * rand_vec2() + 0.5;
                positions.push(InstanceData { model: vec2(i as f32 / w as f32 * 2.0, j as f32 / h as f32 * 2.0) - ofs });
            }
        }

        positions
    };
    let mut instance_mesh = Circle::new(5, 0.0006225, Vec4::ONE).mesh().to_instance(data, n);

    renderer.add_instance_mesh("mesh", instance_mesh).unwrap();

    Circle::new(16, 0.012, vec4(1.0, 0.0, 0.0, 0.0)).add_to_renderer("c", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

        let mut c = renderer.get_mesh_mut("c").unwrap();

        if el.is_key_down(glfw::Key::W) {
            c.position.y += 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::A) {
            c.position.x -= 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::S) {
            c.position.y -= 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::D) {
            c.position.x += 400.0 * dt;
        }

        if el.is_key_down(glfw::Key::Num0) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::None);
        }
        if el.is_key_down(glfw::Key::Num1) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        }


        unsafe {
            Clear(COLOR_BUFFER_BIT);
            renderer.draw(&el);
        }

        time += dt;
        dt = now.elapsed().as_secs_f32();

        if el.is_key_down(glfw::Key::R) {
            dbg!(1.0 / dt);
        }
    }
}
