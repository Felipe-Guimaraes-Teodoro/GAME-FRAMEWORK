use std::path::Path;

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

    
    let cobble_tex = Texture::Path("examples/assets/images/cobble_tex.png".into());
    let roblux_tex = Texture::Path("examples/assets/images/hqdefault.jpg".into());
    let mut c = Cuboid::new(vec3(200., 200., 200.), vec4(1.0, 0.0, 0.0, 1.0)).mesh();
    c.set_shader_type(&ShaderType::Full);
    // c.set_texture(cobble_tex.clone());
    c.setup_mesh();
    renderer.add_mesh("c", c).unwrap();

    let mut s = Sphere::new(32, 500., Vec4::ONE).mesh();
    s.set_shader_type(&ShaderType::Full);
    // s.set_texture(cobble_tex);
    s.setup_mesh();
    s.add_position(vec3(1500., 0., 0.));
    s.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("s", s).unwrap();

    let mut t = Sphere::new(32, 10000., vec4(0.1, 0.2, 0.3, 1.0)).mesh();
    // t.set_texture(roblux_tex);
    for face in t.indices.chunks_mut(6) {
        face.reverse();
    }
    t.setup_mesh();
    t.add_position(vec3(1500., 0., 0.));
    t.scale(vec3(20.0, 20.0, 20.0));
    renderer.add_mesh("t", t).unwrap();

    let mut suzanne = tiny_game_framework::Model::new("examples/assets/models/suzanne.obj");
    suzanne.meshes[0].set_shader_type(&ShaderType::Full);
    renderer.add_model("suzanne", suzanne);

    renderer.add_light("light1", Light {position: vec3(100000.0, 100000.0, 100000.0), color: vec3(0.0, 0.0, 1.0)});
    renderer.add_light("light2", Light {position: vec3(-100000.0, 100000.0, 100000.0), color: vec3(0.0, 1.0, 0.0)});
    renderer.add_light("light3", Light {position: vec3(-100000.0, 100000.0, -100000.0), color: vec3(1.0, 0.0, 0.0)});

    let mut font = unsafe {
        Font::init(800.0, 800.0, "examples/assets/fonts/comic.ttf")
    };

    renderer.camera.speed = 0.5;

    let mut frames = vec![];
    let mut fullscreen = false;

    let mut dist = 0.0;
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
        // t.position = cam_pos;



        let frame = el.ui.frame(&mut el.window);

        { // ui shenanigans
            let blnk = (rand_betw(0, 100) as f32 * 0.4) as u8;
            frame.get_foreground_draw_list().add_text([0.0, 0.0], ImColor32::from_rgb(42 + blnk, 126 + blnk, 200 + blnk), "HELLO, WORLD!");
            // frame.show_demo_window(&mut true);
            frame.menu_item_config(format!("f: {:.2} | dt(ms): {:.2}", 1.0/el.dt, el.dt*1000.0)).build();
            frame.text(format!("t: {:.1}", el.time));
            frame.separator();
            frame.text("TGF © FEROMONEO && GOUD \n\nbuild LATEST ~.134");
            frame.separator();
            frame.text(format!("camera_pos: {:.1} \ncamera_rot: {:?}", &renderer.camera.pos, (&renderer.camera.pitch, &renderer.camera.yaw)));
            frame.slider("timescale", -50.0, 50.0, &mut el.timescale);
            if frame.button("set timescale = 1.0") {
                el.timescale = 1.0;
            }
            if frame.button("set timescale = 0.0") {
                el.timescale = 0.0;
            }
            frame.slider("speed", 0.0, 100.0, &mut renderer.camera.speed);

            frame.window("graph(f × time)").build(|| {
                frames.push(1.0 / el.dt);
                frame.plot_lines(" ", &frames).graph_size([256.0, 64.0]).build();
                if frames.len() > 256 { frames.remove(0); }
            });

            renderer_inspector(&mut renderer, frame);
        }

        if el.is_key_down(Key::LeftAlt) {
            el.window.set_cursor_mode(glfw::CursorMode::Normal);
        } else {
            el.window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        is_set_fullscreen(&mut el, &mut fullscreen);

        let l1 = renderer.get_light_mut("light1").unwrap();
        l1.position = vec3(el.time.cos() * 10000.0, el.time.sin() * 10000.0, 10000.0);

        let l2 = renderer.get_light_mut("light2").unwrap();
        l2.position = vec3(10000.0, el.time.sin() * 10000.0, el.time.cos() * 10000.0);

        let l3 = renderer.get_light_mut("light3").unwrap();
        l3.position = vec3(el.time.sin() * 10000.0, el.time.cos() * 10000.0, 10000.0);

        let direction = renderer.camera.front;
        let right = direction.cross(vec3(0.0, 1.0, 0.0));
        let mut suzanne = renderer.get_model_mut("suzanne").unwrap();
        dist += el.event_handler.scroll.y;
        let mut s = &mut suzanne.meshes[0];
        let goal = cam_pos + direction * dist * 10.0 + right * 200.0;
        s.position.x = lerp(s.position.x, goal.x, 0.25);
        s.position.y = lerp(s.position.y, goal.y, 0.25);
        s.position.z = lerp(s.position.z, goal.z, 0.25);

        unsafe { 
            if el.is_key_down(Key::F1) {
                PolygonMode(FRONT, LINE);
            } else {
                PolygonMode(FRONT, FILL);
            }
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            
            renderer.draw(&el);
            el.ui.draw();
            font.render_text("hiiiii", 10.0, 10.0, 1.0, vec3(1.0, 1.0, 1.0));
        }
    }
}


fn is_set_fullscreen(el: &mut EventLoop, fullscreen: &mut bool) {
    if el.event_handler.key_just_pressed(Key::F11) {
        if !*fullscreen {
            el.glfw.with_primary_monitor(|glfw, monitor| {
                let monitor = monitor.unwrap();
                let mode = monitor.get_video_mode().unwrap();
                el.window.set_monitor(
                    glfw::WindowMode::FullScreen(&monitor), 
                    0, 
                    0, 
                    mode.width, 
                    mode.height, 
                    Some(mode.refresh_rate),
                );
            });
        } else {
            el.glfw.with_primary_monitor(|glfw, monitor| {
                let monitor = monitor.unwrap();
                let mode = monitor.get_video_mode().unwrap();
                el.window.set_monitor(
                    glfw::WindowMode::Windowed, 
                    200, 
                    200, 
                    800, 
                    800, 
                    Some(mode.refresh_rate),
                );
            });
        }
        *fullscreen = !*fullscreen;
    }
}

fn pitch_yaw_to_direction(pitch: f32, yaw: f32) -> Vec3 {
    let pitch_rad = pitch.to_radians();
    let yaw_rad = yaw.to_radians();

    vec3(
        yaw_rad.cos() * pitch_rad.cos(),
        pitch_rad.sin(),
        yaw_rad.sin() * pitch_rad.cos(),
    )
}