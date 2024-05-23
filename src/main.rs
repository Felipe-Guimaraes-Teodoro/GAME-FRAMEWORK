use tiny_game_framework::glam::Vec2;
use tiny_game_framework::imgui::ImColor32;
use tiny_game_framework::gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::glam::{vec2, vec3, vec4, Vec4};
use tiny_game_framework::{lerp, rand_betw, rand_vec2, EventLoop, InstanceData};
use tiny_game_framework::Renderer;

use tiny_game_framework::{Circle, Triangle};

fn main() {
    let resolution = vec2(800., 800.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    let w = 200;
    let h = 200;
    let n = w*h;
    let data = {
        let mut positions = vec![];

        for i in 0..w {
            for j in 0..h {
                let ofs = Vec2::ONE * rand_vec2() + 0.5;
                positions.push(InstanceData { model: vec2(i as f32 / w as f32 * 2.0, j as f32 / h as f32 * 2.0) - ofs });
            }
        }

        positions
    };

    let other_data = {
        let mut positions = vec![];

        for i in 0..50 {
            for j in 0..50 {
                let ofs = Vec2::ONE * rand_vec2() + 0.5;
                positions.push(InstanceData { model: vec2(i as f32 / w as f32 * 2.0, j as f32 / h as f32 * 2.0) - ofs });
            }
        }

        positions
    };

    let instance_mesh = Circle::new(5, 0.0006225, Vec4::ONE).mesh().to_instance(data, n);

    let another_instance_mesh = Triangle::new(0.02, vec4(0.1, 0.3, 0.5, 0.1)).mesh().to_instance(other_data, 2500);

    renderer.add_instance_mesh("mesh", instance_mesh).unwrap();
    renderer.add_instance_mesh("name", another_instance_mesh).unwrap();

    Circle::new(16, 0.012, vec4(1.0, 0.0, 0.0, 0.0)).add_to_renderer("c", &mut renderer);

    let mut dt = 0.;
    let mut time = 0.;
    let mut wish_pos = vec3(0.0, 0.0, 0.0);

    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();

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
            wish_pos.x = lerp(c.position.x, el.event_handler.mouse_pos.x, 0.2);
            wish_pos.y = lerp(c.position.y, el.event_handler.mouse_pos.y, 0.2);
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
        frame.show_demo_window(&mut true);
        frame.menu_item_config(format!("f: {:.2} | dt(ms): {:.2}", 1.0/dt, dt*1000.0)).build();
        frame.separator();
        frame.text("TGF © FEROMONEO && GOUD \n\nbuild LATEST ~.134");

        unsafe {
            Clear(COLOR_BUFFER_BIT);
            renderer.draw(&el);
            el.ui.draw();
        }
        
        time += dt;
        dt = now.elapsed().as_secs_f32();
    }
}
