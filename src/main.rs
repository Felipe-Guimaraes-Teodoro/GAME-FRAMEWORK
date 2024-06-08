use gl::{CullFace, DepthFunc, Enable, FrontFace, PolygonMode, BACK, CULL_FACE, CW, DEPTH_BUFFER_BIT, DEPTH_TEST, FILL, FRONT, LESS, LINE};
use glam::{Quat, Vec3};
use glfw::Key;
use imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, renderer_inspector, Cuboid, EventLoop, Font, InstanceData, Light, ShaderType, Sphere, Texture};
use tiny_game_framework::Renderer;

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);

    unsafe {
        Enable(DEPTH_TEST);
        DepthFunc(LESS);
        Enable(CULL_FACE);
        CullFace(BACK);
        FrontFace(CW);
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

    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        if el.is_key_down(glfw::Key::Num0) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::None);
        }

        if el.is_key_down(glfw::Key::Num1) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        }

        let cam_pos = renderer.camera.pos * resolution.x;

        let s = renderer.get_mesh_mut("s").unwrap();
        s.rotation = Quat::from_euler(glam::EulerRot::XYZ, 0.0, el.time / 20.0, 0.0);

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
