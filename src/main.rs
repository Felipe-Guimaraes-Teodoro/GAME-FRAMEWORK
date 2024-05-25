use glam::Vec3;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{Cuboid, EventLoop};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Line};

fn main() {
    let resolution = vec2(600., 600.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let c = Cuboid::new(vec3(100., 100., 100.), vec4(1.0, 0.0, 0.0, 0.0)).mesh();
    renderer.add_mesh("c", c).unwrap();

    renderer.add_mesh("l", Line::new(vec3(-300., -300., 0.), vec3(300., 300., 0.), 100., Vec4::ONE).mesh()).unwrap();

    renderer.add_mesh("zero", Circle::new(8, 50., vec4(0., 1., 0., 1.)).mesh()).unwrap();
    renderer.get_mesh_mut("zero").unwrap().set_position(Vec3::ZERO);

    while !el.window.should_close() {
        el.update();

        let mouse_pos = el.event_handler.mouse_pos;

        renderer.get_mesh_mut("c").unwrap().position = vec3(mouse_pos.x, mouse_pos.y, 0.);

        unsafe {
            Clear(COLOR_BUFFER_BIT);
            renderer.draw(&el);
        }
    }
}
