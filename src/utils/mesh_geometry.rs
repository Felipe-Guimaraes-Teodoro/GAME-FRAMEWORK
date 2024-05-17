use once_cell::sync::Lazy;

use crate::{graphics::{Mesh, Vertex}, Renderer, Vector3D};
use crate::utils::Vector4D;
use crate::utils::Vector2D;

pub fn quad_indices() -> Vec<u32> {
    vec![0, 2, 1, 2, 3, 1]
}

pub fn quad_vertices(width: f32, height: f32) -> Vec<Vertex> {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // vec![
    //     Vertex::new(Vector3D::ZERO, Vector4D::ZERO),
    //     Vertex::new(Vector3D::new(half_width, 0.0, 0.0), Vector3D::ZERO),
    //     Vertex::new(Vector3D::new(0.0, half_height, 0.0), Vector3D::ZERO),
    //     Vertex::new(Vector3D::new(half_width, half_height, 0.0), Vector3D::ZERO),
    // ]

    todo!()
}

pub struct Circle{
    pub resolution: i32,
    pub position: Vector2D,
    pub radius: f32,
    pub color: Vector4D,
}

impl Circle {
    pub fn new(resolution: i32, position: Vector2D, radius: f32, color: Vector4D) -> Self{
        Self {
            resolution,
            position,
            radius,
            color,
        }
    }
    
    pub fn add_to_renderer(self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        let angle_increment = 2.0 * pi / self.resolution as f32;
        // Vertex::new(position, color);
        for i in 0..self.resolution {
            let angle = i as f32 * angle_increment;
            vertices.push(Vertex::new(Vector3D::new(self.position.x + f32::sin(2.*pi*i as f32/self.resolution as f32),
                                                    self.position.y + f32::cos(2.*pi*i as f32/self.resolution as f32),
                                                    1./self.radius)*self.radius,
                                                    self.color));
        }
        let mut indices = vec![];
        for i in 1..=self.resolution-2 {
            indices.push(0); 
            indices.push(i as u32); 
            indices.push((i % self.resolution + 1) as u32);
        }
        
        let mesh = Mesh::new(vertices, indices);
        renderer.add_mesh(name, mesh)        
    }
}
