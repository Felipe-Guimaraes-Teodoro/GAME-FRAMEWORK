mod graphics;
mod events;
mod utils;

//use tiny_game_framework::*;

use events::EventLoop;
use gl::{Clear, COLOR_BUFFER_BIT};
use graphics::Renderer;
use tiny_game_framework::Matrix4x4;

use crate::utils::{lerp, Vector3D};

fn main() {
    let mut el = EventLoop::new(500, 500);
    let mut renderer = Renderer::new();
    
    // test_scene(&mut renderer);
    
    let thread = std::thread::spawn(|| {
        loop {
            let now = std::time::Instant::now();
            _tests();
            println!("ELA: {:?}", now.elapsed());

            // std::thread::sleep_ms(256);
        }
    });

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

fn _tests() {
    let v1 = Vector3D::new(0.0, 2.0, 0.0);
    let v2 = Vector3D::new(2.0, 0.0, 0.0);

    // println!("sum: {:?}", v1 + v2);
    // println!("dot: {:?}", Vector3D::dot(v1, v2));
    // println!("manitude: {:?}", Vector3D::magnitude(v1));
    // println!("normalize: {:?}", Vector3D::normalize(v1));
    // println!("cross: {:?}", Vector3D::cross(v1, v2));

    for i in 0..10 {
        let l = lerp(0.0, 100.0, i as f32);
        
        // println!("lerp: {:?}", l);
    }

    // println!("testing matrixes: ");

    let m1 = Matrix4x4::perspective(120.0, 1.0, 0.1, 100.0);
    let m2 = Matrix4x4::perspective(90.0, 1.0, 0.1, 100.0);

    // println!("mul: {:?}", m1 * m2);
}
