use gl::types::GLuint;
use gl::{BeginQuery, ColorMask, CullFace, DepthFunc, Enable, FrontFace, GenQueries, PolygonMode, ANY_SAMPLES_PASSED, BACK, CULL_FACE, CW, DEPTH_BUFFER_BIT, DEPTH_TEST, FALSE, FILL, FRONT, LESS, LINE, TRUE};
use glam::{Quat, Vec3};
use glfw::{Action, Key};
use imgui::Ui;
use tiny_game_framework::glam::Vec2;
use tiny_game_framework::imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, Cuboid, EventHandler, EventLoop, Font, InstanceData, Shader, LIGHT_MESH_SHADER_FS, LIGHT_MESH_SHADER_VS};
use tiny_game_framework::Renderer;


fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();
    renderer.camera.speed = 0.5;
    renderer.camera.set_projection(tiny_game_framework::ProjectionType::Orthographic);
    
    unsafe {
        Enable(DEPTH_TEST);
        DepthFunc(LESS);
        Enable(CULL_FACE);
        CullFace(BACK);
        FrontFace(CW);
    }
    
    let mut font = unsafe {
        Font::init(resolution.x, resolution.y, "C:/Users/Usuario/Documents/Xfer/Serum Presets/Skins/Default/Fonts/Nunito-Regular.ttf")
    };

    let w = 100;
    let h = 100;
    let n = w*h;
    let data = {
        let mut positions = vec![];

        for i in 0..w {
            for j in 0..h {
                let ofs = rand_vec3() * 100.0 - 1.0;
                let rot = Quat::from_vec4(rand_vec4());
                let sca = rand_vec3() + 0.1; 
                positions.push(InstanceData::new(ofs, rot, sca));
            }
        }

        positions
    };
    
    let instance_mesh = Cuboid::new(vec3(0.1, 0.1, 0.1), vec4(1.0, 1.0, 1.0, 1.0)).mesh().to_instance(data, n);
    renderer.add_instance_mesh("mesh", instance_mesh).unwrap();


    let c = Cuboid::new(vec3(600., 600., 600.0), vec4(1.0, 1.0, 1.0, 1.0)).mesh();
    let mut l = Cuboid::new(vec3(25.0, 25.0, 25.0), Vec4::ONE).mesh();
    for face in l.indices.chunks_mut(6) {
        face.reverse();
    }
    for vertex in &mut l.vertices {
       vertex.normal = -vertex.normal;
    }
    unsafe {
        l.setup_mesh();
    }
    renderer.add_mesh("c", c).unwrap();
    renderer.add_mesh("l", l).unwrap();

    let mut fullscreen = false;
    let mut frames = vec![];

    while !el.window.should_close() {
        el.update();
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.update(renderer.camera.pos);

        let time = el.time;
        renderer.get_mesh_mut("l").unwrap().position = vec3(time.cos(), time.sin(), time.cos()) * resolution.x;

        if el.is_key_down(glfw::Key::Num0) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::None);
        }

        if el.is_key_down(glfw::Key::Num1) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        }

        //let s = renderer.get_mesh_mut("s").unwrap();
        // s.position = vec3(el.time.sin(), el.time.cos(), 0.0) * 100.0;

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
            frame.slider("timescale", -10.0, 10.0, &mut el.timescale);
            if frame.button("set timescale = 1.0") {
                el.timescale = 1.0;
            }
            if frame.button("set timescale = 0.0") {
                el.timescale = 0.0;
            }
            
            frame.window("graph(f × time)").build(|| {
                frames.push(1.0 / el.dt);
                frame.plot_lines(" ", &frames).graph_size([256.0, 64.0]).build();
                if frames.len() > 256 { frames.remove(0); }
            });
        }
        
        if el.is_key_down(Key::LeftAlt) {
            el.window.set_cursor_mode(glfw::CursorMode::Normal);
        } else {
            el.window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        is_set_fullscreen(&mut el, &mut fullscreen);
        
        unsafe {
            if el.is_key_down(Key::F1) {
                PolygonMode(FRONT, LINE);
            } else {
                PolygonMode(FRONT, FILL);
            }

            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            renderer.draw(&el);

            font.render_text(&format!("random char: {:?}", rand_betw('a', 'z')), 25.0, 25.0, 1.0, vec3(1.0, 1.0, 1.0));
            font.render_text("build 0.135", 25.0, 75.0, 1.0, vec3(1.0, 1.0, 1.0));
            font.render_text("Tiny Game Framework", 25.0, 125.0, 1.0, vec3(1.0, 1.0, 1.0));


            el.ui.draw();
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