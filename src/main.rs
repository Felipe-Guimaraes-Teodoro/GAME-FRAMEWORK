mod graphics;
mod events;
mod utils;

//use tiny_game_framework::*;

use events::EventLoop;
use gl::{Clear, COLOR_BUFFER_BIT};
use graphics::{test_scene, Renderer};

use crate::utils::{lerp, Vector3D};

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
    
    println!("sum: {:?}", v1 + v2);
    println!("dot: {:?}", Vector3D::dot(v1, v2));
    println!("manitude: {:?}", Vector3D::magnitude(v1));
    println!("normalize: {:?}", Vector3D::normalize(v1));
    println!("cross: {:?}", Vector3D::cross(v1, v2));

    for i in 0..10 {
        let l = lerp(0.0, 100.0, i as f32);

        println!("lerp: {:?}", l);
    }
}
