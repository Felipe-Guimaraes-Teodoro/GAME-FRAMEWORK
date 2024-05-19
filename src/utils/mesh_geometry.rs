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
    pub fn new(position: Vector3D, size: Vector3D, resolution: Vector2D, color: Vector4D) -> Self{
        let fixed_position = position/Vector3D::new(resolution.x, resolution.y, 1.);
        let fixed_size = size/Vector3D::new(resolution.x, resolution.y, 1.);
        
        Self{
            position: fixed_position,
            size: fixed_size,
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
        renderer.add_mesh(name, Mesh::new(vertices, indices));
    }
}

pub struct Circle{
    pub iterations: i32,
    pub position: Vector3D,
    pub radius: f32,
    pub color: Vector4D,
}

impl Circle {
    pub fn new(iterations: i32, position: Vector3D, radius: f32, resolution: Vector2D, color: Vector4D) -> Self{
        let mut fixed_iterations = iterations;

        if iterations <= 3{
            fixed_iterations = 4;
        }

        let fixed_position = position/Vector3D::new(resolution.x, resolution.y, 1.);
        let fixed_radius = radius/resolution.x.max(resolution.y);
        
        Self {
            iterations: fixed_iterations,
            position: fixed_position,
            radius: fixed_radius,
            color,
        }
    }
    
    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        
        for i in 0..self.iterations {
            vertices.push(Vertex::new(Vector3D::new(self.position.x + f32::sin(2.*pi*i as f32/self.iterations as f32),
                                                    self.position.y + f32::cos(2.*pi*i as f32/self.iterations as f32),
                                                    1./self.radius)*self.radius,
                                                    self.color));
        }
        let mut indices = vec![];
        for i in 1..=self.iterations-2 {
            indices.push(0); 
            indices.push(i as u32); 
            indices.push((i % self.iterations + 1) as u32);
        }

        renderer.add_mesh(name, Mesh::new(vertices, indices))        
    }
}

pub struct Triangle{
    pub position: Vector3D,
    pub size: f32,
    pub color: Vector4D,
}

impl Triangle{
    pub fn new(position: Vector3D, size: f32, resolution: Vector2D, color: Vector4D) -> Self{
        let fixed_position = position/Vector3D::new(resolution.x, resolution.y, 1.);
        let fixed_size = size/resolution.x.max(resolution.y);

        Self {
            position: fixed_position,
            size: fixed_size,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        for i in 0..3 {
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

        renderer.add_mesh(name, Mesh::new(vertices, indices))        
    }
}

pub struct Line{
    begin: Vector3D,
    end: Vector3D,
    width: f32,
    color: Vector4D,
}

impl Line{
    pub fn new(begin: Vector3D, end: Vector3D, width: f32, resolution: Vector2D, color: Vector4D) -> Self{
        let fixed_begin = begin/Vector3D::new(resolution.x, resolution.y, 1.);
        let fixed_end = end/Vector3D::new(resolution.x, resolution.y, 1.);
        let fixed_width = width/resolution.x.max(resolution.y);

        Self{
            begin: fixed_begin,
            end: fixed_end,
            width: fixed_width,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer){
        let vertices = vec![
            Vertex::new(Vector3D::new(self.begin.x+self.width, self.begin.y+self.width, 0.0), self.color),
            Vertex::new(Vector3D::new(self.end.x+self.width, self.end.y+self.width, 0.0), self.color),
            Vertex::new(Vector3D::new(self.begin.x-self.width, self.begin.y-self.width, 0.0), self.color),
            Vertex::new(Vector3D::new(self.end.x-self.width, self.end.y-self.width, 0.0), self.color),
        ];

        let indices = vec![0, 2, 1, 2, 3, 1];
        renderer.add_mesh(name, Mesh::new(vertices, indices));
    }
}
