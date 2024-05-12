use once_cell::sync::Lazy;

use crate::graphics::Vertex;


pub fn quad_indices() -> Vec<u32> {
    vec![0, 2, 1, 2, 3, 1]
}

pub fn quad_vertices(width: f32, height: f32) -> Vec<Vertex> {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    vec![
        Vertex::new(0.0, 0.0, 0.0),
        Vertex::new(half_width, 0.0, 0.0),
        Vertex::new(0.0, half_height, 0.0),
        Vertex::new(half_width, half_height, 0.0),
    ]
}
