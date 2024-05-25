use glam::{Quat, Vec3};
use tiny_game_framework::glam::Vec2;
use tiny_game_framework::imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, rand_vec3, rand_vec4, EventLoop, InstanceData, Line};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Triangle};

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();
    renderer.camera.set_projection(tiny_game_framework::ProjectionType::Orthographic);
    
    let w = 100;
    let h = 100;
    let n = w*h;
    let data = {
        let mut positions = vec![];

        for i in 0..w {
            for j in 0..h {
                let ofs = rand_vec3() * 10.0 - 1.0;
                let rot = Quat::from_vec4(rand_vec4());
                let sca = rand_vec3() + 0.1; 
                positions.push(InstanceData::new(ofs, rot, sca));
            }
        }

        positions
    };

    let instance_mesh = Circle::new(5, 0.065, Vec4::ONE).mesh().to_instance(data, n);

    renderer.add_instance_mesh("mesh", instance_mesh).unwrap();

    Circle::new(16, 0.012, vec4(1.0, 0.0, 0.0, 0.0)).add_to_renderer("c", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;
    let mut wish_pos = vec3(0.0, 0.0, 0.0);

    while !el.window.should_close() {
        let now = std::time::Instant::now();
        el.update();
        renderer.camera.update(renderer.camera.pos);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.input(&el.window, &el.window.glfw);
        el.window.set_cursor_mode(glfw::CursorMode::Disabled);

        let c = renderer.get_mesh_mut("c").unwrap();

        if el.is_key_down(glfw::Key::W) {
            wish_pos.y += 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::A) {
            wish_pos.x -= 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::S) {
            wish_pos.y -= 400.0 * dt;
        }
        if el.is_key_down(glfw::Key::D) {
            wish_pos.x += 400.0 * dt;
        }

        if el.event_handler.lmb {
            wish_pos.x = el.event_handler.mouse_pos.x;
            wish_pos.y = el.event_handler.mouse_pos.y;
        }

        c.position.x = lerp(c.position.x, wish_pos.x, 0.2);
        c.position.y = lerp(c.position.y, wish_pos.y, 0.2);

        if el.is_key_down(glfw::Key::Num0) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::None);
        }

        if el.is_key_down(glfw::Key::Num1) {
            el.window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        }

        let frame = el.ui.frame(&mut el.window);

        let blnk = (rand_betw(0, 100) as f32 * 0.4) as u8;
        frame.get_foreground_draw_list().add_text([0.0, 0.0], ImColor32::from_rgb(42 + blnk, 126 + blnk, 200 + blnk), "HELLO, WORLD!");
        // frame.show_demo_window(&mut true);
        frame.menu_item_config(format!("f: {:.2} | dt(ms): {:.2}", 1.0/dt, dt*1000.0)).build();
        frame.separator();
        frame.text("TGF © FEROMONEO && GOUD \n\nbuild LATEST ~.134");
        frame.separator();
        frame.text(format!("camera_pos: {:.1} \ncamera_rot: {:?}", &renderer.camera.pos, (&renderer.camera.pitch, &renderer.camera.yaw)));
        
        unsafe {
            Clear(COLOR_BUFFER_BIT);
            renderer.draw(&el);
            el.ui.draw();
        }
        
        time += dt;
        dt = now.elapsed().as_secs_f32();
    }
}
