use once_cell::sync::Lazy;

use crate::{graphics::{Mesh, Vertex}, Renderer, Vector3D};
use crate::utils::Vector4D;
use crate::utils::Vector2D;

pub struct Quad{
    pub position: Vector3D,
    pub size: Vector3D,
    pub color: Vector4D,
}

impl Quad{
    pub fn new(position: Vector3D, size: Vector3D, color: Vector4D) -> Self{
        Self{
            position,
            size,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer){
        let vertices = vec![
            Vertex::new(Vector3D::new(self.position.x, self.position.y, 0.0), self.color),
            Vertex::new(Vector3D::new(self.position.x, self.position.y + self.size.y, 0.0), self.color),
            Vertex::new(Vector3D::new(self.position.x + self.size.x, self.position.y, 0.0), self.color),
            Vertex::new(Vector3D::new(self.position.x + self.size.x, self.position.y + self.size.y, 0.0), self.color),
        ];

        let indices = vec![0, 2, 1, 2, 3, 1];
        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap();
    }
}

pub struct Circle{
    pub resolution: i32,
    pub position: Vector3D,
    pub radius: f32,
    pub color: Vector4D,
}

impl Circle {
    pub fn new(resolution: i32, position: Vector3D, radius: f32, color: Vector4D) -> Self{
        let mut fixed_resolution = resolution;

        if resolution <= 3{
            fixed_resolution = 4;
        }

        Self {
            resolution: fixed_resolution,
            position,
            radius,
            color,
        }
    }
    
    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
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

        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap();
    }
}

pub struct Triangle{
    pub position: Vector3D,
    pub size: f32,
    pub color: Vector4D,
}

impl Triangle{
    pub fn new(position: Vector3D, size: f32, color: Vector4D) -> Self{
        Self {
            position,
            size,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        let angle_increment = 2.0 * pi / 3. as f32;
        for i in 0..3 {
            let angle = i as f32 * angle_increment;
            vertices.push(Vertex::new(Vector3D::new(self.position.x + f32::sin(2.*pi*i as f32/3. as f32),
                                                    self.position.y + f32::cos(2.*pi*i as f32/3. as f32),
                                                    1./self.size)*self.size,
                                                    self.color));
        }
        let mut indices = vec![];
            indices.push(0); 
            indices.push(1 as u32); 
            indices.push(2 as u32);
            // Shamelessly (ok theres a bit of shame) stole my own circle rendering code so I just set it to three vertices

        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap(); 
    }
}
