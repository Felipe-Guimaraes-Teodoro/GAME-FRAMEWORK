use gl::{Enable, DEPTH_BUFFER_BIT, DEPTH_TEST};
use glam::Vec3;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, Cuboid, EventLoop, InstanceData, Line, Quad, ShaderType, Sphere, Texture};
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
    
    let c = Cuboid::new(vec3(1000., 1000., 1000.), Vec4::ONE, Texture::Path("src/graphics/images/cobble_tex.png".to_owned()), ShaderType::Light()).mesh();
    renderer.add_mesh("c", c).unwrap();

    let mut s = Sphere::new(32, 500., Vec4::ONE, Texture::Path("src/graphics/images/hqdefault.jpg".to_owned()), ShaderType::Default()).mesh();
    s.add_position(vec3(1500., 0., 0.));
    renderer.add_mesh("s", s).unwrap();

    let mut t = Triangle::new(5000., vec4(1., 0., 0., 1.), Texture::Path("src/graphics/images/amongus.jpg".to_owned()), ShaderType::Default()).mesh();
    t.add_position(vec3(-1500., 0., 0.));
    renderer.add_mesh("t", t).unwrap();

    let mut q = Quad::new(vec3(350., 492., 0.)*2., Vec4::ONE, Texture::None, ShaderType::Default()).mesh();
    q.add_position(vec3(0., -3500., 0.));
    renderer.add_mesh("q", q).unwrap();

    let mut light = Quad::new(Vec3::ONE*10., Vec4::ONE, Texture::None, ShaderType::Default()).mesh();
    light.set_position(vec3(0., 550., -600.));
    renderer.add_mesh("light", light).unwrap();

    renderer.camera.speed = 0.5;

    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        let frame = el.ui.frame(&mut el.window);

        frame.menu_item_config(format!("f: {:.2} | dt(ms): {:.2}", 1.0/el.dt, el.dt*1000.0)).build();
        frame.text(format!("t: {:.1}", el.time));

        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            renderer.draw(&el);
            el.ui.draw();
        }
    }
}
