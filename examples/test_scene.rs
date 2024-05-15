use gl::{Clear, COLOR_BUFFER_BIT};
use tiny_game_framework::*;

fn main() {
    let mut el = EventLoop::new(500, 500);
    let mut renderer = Renderer::new();
    
    test_scene(&mut renderer);

    let mut time: f32 = 0.0;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();
        renderer.update();

        if el.is_key_down(glfw::Key::I) {
            println!("{:?}", el.event_handler.mouse_pos.x / el.event_handler.width);
        }

        renderer.get_mesh("1").unwrap().position =
            Vector3D::new(time.sin(), time.cos(), 0.0);

        let mouse_pos = Vector3D::new(el.event_handler.mouse_pos.x / el.event_handler.width, el.event_handler.mouse_pos.y / el.event_handler.height, 0.0);
        let mesh_2 = renderer.get_mesh("2").unwrap();
        mesh_2.position =
            Vector3D::new(
                lerp(mesh_2.position.x, mouse_pos.x * 2.0 - 1.0, 0.1), 
                -lerp(-mesh_2.position.y, mouse_pos.y * 2.0 - 1.0, 0.1), 
                0.001
            );
            

        unsafe {
            Clear(COLOR_BUFFER_BIT);

            renderer.draw();
        }

        time += now.elapsed().as_secs_f32();
    }
}