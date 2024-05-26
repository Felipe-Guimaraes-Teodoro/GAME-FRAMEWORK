use gl::{Enable, DEPTH_BUFFER_BIT, DEPTH_TEST};
use glam::{Quat, Vec3};
use glfw::{Action, Key};
use tiny_game_framework::glam::Vec2;
use tiny_game_framework::imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, Cuboid, EventLoop, InstanceData, Line, Sphere};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Triangle};

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);
    
    unsafe {
        Enable(DEPTH_TEST);
    }

    // let c = Cuboid::new(vec3(600., 600., 1.5), vec4(0.1, 0.0, 0.0, 1.0)).mesh();
    // renderer.add_mesh("c", c).unwrap();

    // let s = Sphere::new(16, 100.0, Vec4::ONE).mesh();
    // renderer.add_mesh("s", s).unwrap();

    let size = 10;
    let spacing = 30.;
    let mut counter = 0;

    for x in 0..size{
        for y in 0..size{
            for z in 0..size{
                let mut c = Cuboid::new(vec3(15., 15., 15.), Vec4::ONE).mesh();
                c.set_position(vec3(x as f32, y as f32, z as f32)*spacing);

                counter += 1;
                renderer.add_mesh(&format!("{:?}", counter), c).unwrap();
            }
        }
    }

    renderer.camera.speed = 0.5;

    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        let frame = el.ui.frame(&mut el.window);

        frame.menu_item_config(format!("f: {:.2} | dt(ms): {:.2}", 1.0/el.dt, el.dt*1000.0)).build();
        frame.text(format!("t: {:.1}", el.time));

        if el.is_key_down(Key::Q){
            renderer.get_mesh_mut("1").unwrap().position.x += 1.;
            println!("Q");
        }
        if el.is_key_down(Key::E){
            renderer.get_mesh_mut("1").unwrap().position.y += 1.;
            println!("E");
        }
        if el.is_key_down(Key::R){
            renderer.get_mesh_mut("1").unwrap().position.z += 1.;
            println!("R");
        }

        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            renderer.draw(&el);
            el.ui.draw();
        }
    }
}
