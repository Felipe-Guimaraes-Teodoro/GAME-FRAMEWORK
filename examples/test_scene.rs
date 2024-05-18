use std::sync::Once;

use tiny_game_framework::*;

fn main() {
    let mut el = EventLoop::new(500, 500);
    let mut renderer = Renderer::new();

    unsafe {
        Enable(BLEND);
        BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA); 
    }

    fn nome_aleatorio() -> String {
        format!("{:?}", rand_betw(0.0_f64, 1000.0_f64))
    }

    let mut c = Circle::new(32, Vector3D::new(0.0, 0.0, 0.0), 0.5, Vector4D::new(0.5, 1.0, 0.5, 0.2));
    c.add_to_renderer("oi", &mut renderer);
    
    let t = Triangle::new(Vector3D::new(1.0, 0.0, 0.0), 0.5, Vector4D::new(1.0, 1.0, 1.0, 0.6));
    t.add_to_renderer("trianglu", &mut renderer);

    let q = Quad::new(Vector3D::new(-1.0, 0.0, 0.0), Vector3D::new(0.5, 0.5, 0.5), Vector4D::new(0.5, 0.1, 0.1, 0.9));
    q.add_to_renderer("quda", &mut renderer);

    // let mut t = Triangle
    for i in 0..256 {
        let c = Circle::new(32, Vector3D::new(0.0, 0.0, 0.0), 0.1, Vector4D::new(1.0, 1.0, 1.0, 1.0));
        c.add_to_renderer(&format!("{:?}", i), &mut renderer);
    }
    
    let o = Once::new(); 
    let mut time: f32 = 0.0;
    while !el.window.should_close() {
        let now = std::time::Instant::now();

        el.update();
        renderer.update();

        if el.is_key_down(glfw::Key::I) {
            println!("{:?}", el.event_handler.mouse_pos.x / el.event_handler.width);
        }

        let mouse_pos = Vector3D::new(el.event_handler.mouse_pos.x / el.event_handler.width, el.event_handler.mouse_pos.y / el.event_handler.height, 0.0);

        unsafe {
            Clear(COLOR_BUFFER_BIT);

            renderer.draw();
            
            o.call_once(|| {
                for i in 0..256 {
                    renderer.destroy_mesh(&format!("{:?}", i));
                }
            });
        }

        time += now.elapsed().as_secs_f32();
    }
}