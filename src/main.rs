use gl::{Enable, DEPTH_BUFFER_BIT, DEPTH_TEST};
use glam::{Quat, Vec3};
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, Cuboid, EventLoop, InstanceData, Light, Line, ShaderType, Sphere, Texture};
use tiny_game_framework::Renderer;

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);
    
    unsafe {
        Enable(DEPTH_TEST);
    }
    
    let cobble_tex = Texture::Path("examples/assets/images/cobble_tex.png".into());
    let roblux_tex = Texture::Path("examples/assets/images/roblux.jpg".into());
    let mut c = Cuboid::new(vec3(200., 200., 200.), vec4(1.0, 0.0, 0.0, 1.0)).mesh();
    c.set_shader_type(&ShaderType::Full);
    c.set_texture(cobble_tex);
    c.setup_mesh();
    renderer.add_mesh("c", c).unwrap();

    let mut s = Sphere::new(32, 500., Vec4::ONE).mesh();
    s.set_shader_type(&ShaderType::Full);
    s.set_texture(cobble_tex);
    s.setup_mesh();
    s.add_position(vec3(1500., 0., 0.));
    s.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("s", s).unwrap();

    let mut t = Sphere::new(32, 10000., Vec4::ONE).mesh();
    t.set_texture(roblux_tex);
    t.setup_mesh();
    t.add_position(vec3(1500., 0., 0.));
    t.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("t", t).unwrap();

    renderer.add_light("light1", Light {position: vec3(1000.0, 1000.0, 1000.0), color: vec3(1.0, 1.0, 1.0)});

    renderer.camera.speed = 0.5;

    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        let cam_pos = renderer.camera.pos * resolution.x;

        let s = renderer.get_mesh_mut("s").unwrap();
        s.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, el.time / 20.0, 0.0);

        let t = renderer.get_mesh_mut("t").unwrap();
        t.position = cam_pos;

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
