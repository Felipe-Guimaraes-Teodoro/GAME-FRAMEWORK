use gl::{Clear, ClearColor, CullFace, DepthFunc, Enable, FrontFace, PolygonMode, BACK, COLOR_BUFFER_BIT, CULL_FACE, CW, DEPTH_BUFFER_BIT, DEPTH_TEST, FILL, FRONT, LESS, LINE};
use glam::{vec3, Vec3};
use glfw::Key;
use tiny_game_framework::{EventLoop, Light, Model, Renderer};
use tiny_game_framework::glam::{Vec2, vec2};

fn main() {
    let resolution = vec2(500., 500.);
    let mut el = EventLoop::new(resolution.x as u32, resolution.y as u32);
    let mut renderer = Renderer::new();

    unsafe {
        Enable(DEPTH_TEST);
        DepthFunc(LESS);
        Enable(CULL_FACE);
        CullFace(BACK);
        FrontFace(CW);
    }

    el.window.set_cursor_mode(glfw::CursorMode::Disabled);

    let suzanne = Model::new("examples/assets/models/suzanne.obj");

    renderer.add_model("suzanne", suzanne);


    // renderer.add_model("suzanne", model);
    renderer.add_light("l1", Light {position: vec3(10.0, 10.0, 5.0), color: Vec3::ONE});

    while !el.window.should_close() {
        el.update();

        renderer.camera.input(&el.window, &el.window.glfw);
        renderer.camera.mouse_callback(el.event_handler.mouse_pos.x, el.event_handler.mouse_pos.y, &el.window);
        renderer.camera.update(renderer.camera.pos);

        let mut l = renderer.get_light_mut("l1").unwrap();
        l.position = vec3(el.time.sin() * 100.0, el.time.cos() * 100.0, 0.0);

        unsafe {
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            ClearColor(0.1, 0.2, 0.3, 1.0);

            if el.is_key_down(Key::F1) {
                PolygonMode(FRONT, LINE);
            } else {
                PolygonMode(FRONT, FILL);
            }

            renderer.draw(&el);
        }
    }
}
