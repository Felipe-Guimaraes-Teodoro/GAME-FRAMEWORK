mod graphics;
mod events;
mod utils;

use tiny_game_framework::*;

use crate::utils::{lerp, Vector3D};

fn main() {
    run();
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
