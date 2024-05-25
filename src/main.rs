use glam::Vec3;
use tiny_game_framework::glam::Vec2;
use tiny_game_framework::imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, EventLoop, InstanceData};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Triangle, Line};

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    Circle::new(16, 0.012, vec4(1.0, 0.0, 0.0, 0.0)).add_to_renderer("c", &mut renderer);
    Line::new(vec3(0.5, 0.5, 0.), vec3(0.9, 0.5, 0.), 0.01, Vec4::ONE).add_to_renderer("l", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;

    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

        renderer.get_mesh_mut("c").unwrap().position = vec3(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, 0.);

        unsafe {
            Clear(COLOR_BUFFER_BIT);
            renderer.draw(&el);
        }
        
        time += dt;
        dt = now.elapsed().as_secs_f32();
    }
}
