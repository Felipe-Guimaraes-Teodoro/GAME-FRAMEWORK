use gl::{BlendFunc, CullFace, DepthFunc, Enable, FrontFace, PolygonMode, BACK, CULL_FACE, CW, DEPTH_BUFFER_BIT, DEPTH_TEST, FILL, FRONT, LESS, LINE};
use glam::{Quat, Vec3};
use glfw::Key;
use imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, renderer_inspector, Circle, Cuboid, EventLoop, Font, InstanceData, Light, Quad, ShaderType, Sphere, Texture};
use tiny_game_framework::Renderer;

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);

    unsafe {
        Enable(DEPTH_TEST);
    }
    
    let cobble_tex = "examples/assets/images/cobble_tex.png";
    let roblux_tex = "examples/assets/images/hqdefault.jpg";

    renderer.add_texture("cobble".to_owned(), cobble_tex.to_owned());
    renderer.add_texture("roblux".to_owned(), roblux_tex.to_owned());

    let mut c = Cuboid::new(vec3(200., 200., 200.), vec4(1.0, 0.0, 0.0, 1.0)).mesh();
    c.set_shader_type(&ShaderType::Full);
    c.set_texture("cobble", &renderer);
    c.setup_mesh();
    renderer.add_mesh("c", c).unwrap();

    let mut c2 = Cuboid::new(Vec3::ONE*150., vec4(0.0, 0.0, 1.0, 0.0)).mesh();
    c2.set_shader_type(&ShaderType::Full);
    c2.set_texture("cobble", &renderer);
    c2.setup_mesh();
    c2.add_position(vec3(300., 0., 0.));
    renderer.add_mesh("c2", c2).unwrap();

    //renderer.get_mesh("c").unwrap().clone().add_child(renderer.get_mesh("c2").unwrap().clone());

    let mut s = Sphere::new(128, 500., Vec4::ONE).mesh();
    s.set_shader_type(&ShaderType::Full);
    s.set_texture("cobble", &renderer);
    s.setup_mesh();
    s.add_position(vec3(1500., 0., 0.));
    s.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("s", s).unwrap();

    let mut t = Sphere::new(32, 10000., vec4(0.1, 0.2, 0.3, 1.0)).mesh();
    t.set_texture("roblux", &renderer);
    for face in t.indices.chunks_mut(6) {
        face.reverse();
    }

    t.setup_mesh();
    t.add_position(vec3(1500., 0., 0.));
    t.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("t", t).unwrap();


    renderer.add_light("light1", Light {position: vec3(100000.0, 100000.0, 100000.0), color: vec3(0.0, 0.0, 1.0)});
    renderer.add_light("light2", Light {position: vec3(-100000.0, 100000.0, 100000.0), color: vec3(0.0, 1.0, 0.0)});
    renderer.add_light("light3", Light {position: vec3(-100000.0, 100000.0, -100000.0), color: vec3(1.0, 0.0, 0.0)});


    renderer.camera.speed = 0.5;
    let mut fullscreen = false;

    let mut a = 0.;
    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        // Use this to debug when working with alpha
        a += el.event_handler.scroll.y / 50.;

        if a < 0.{
            a = 0.
        }
        else if a > 1.{
            a = 1.
        }

        let c2_mesh = renderer.get_mesh_mut("c2").unwrap();
        c2_mesh.set_color(vec4(rand_betw(0., 1.), rand_betw(0., 1.), rand_betw(0., 1.), a));
        println!("{}", a);

        let m = renderer.get_mesh_mut("c").unwrap();
        let mut pos = Vec3::ZERO;
        if el.is_key_down(Key::Up){
            pos += vec3(0., 1., 0.)*el.dt*500.;
        }
        if el.is_key_down(Key::Down){
            pos -= vec3(0., 1., 0.)*el.dt*500.;
        }
        if el.is_key_down(Key::Left){
            pos -= vec3(1., 0., 0.)*el.dt*500.;
        }
        if el.is_key_down(Key::Right){
            pos += vec3(1., 0., 0.)*el.dt*500.;
        }
        m.add_position(pos);

        let cam_pos = renderer.camera.pos * resolution.x;

        let t = renderer.get_mesh_mut("t").unwrap();
        t.position = cam_pos;

        if el.is_key_down(Key::LeftAlt) {
            el.window.set_cursor_mode(glfw::CursorMode::Normal);
        } else {
            el.window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        if el.is_key_down(Key::F11){
            el.set_fullscreen(&fullscreen);
            fullscreen = !fullscreen;
        }

        let l1 = renderer.get_light_mut("light1").unwrap();
        l1.position = vec3(el.time.cos() * 10000.0, el.time.sin() * 10000.0, 10000.0);

        let l2 = renderer.get_light_mut("light2").unwrap();
        l2.position = vec3(10000.0, el.time.sin() * 10000.0, el.time.cos() * 10000.0);

        let l3 = renderer.get_light_mut("light3").unwrap();
        l3.position = vec3(el.time.sin() * 10000.0, el.time.cos() * 10000.0, 10000.0);

        unsafe {
            if el.is_key_down(Key::F1) {
                PolygonMode(FRONT, LINE);
            } else {
                PolygonMode(FRONT, FILL);
            }
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            renderer.draw(&el);
        }
    }
}
