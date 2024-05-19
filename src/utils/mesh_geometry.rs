use once_cell::sync::Lazy;

use crate::{graphics::{Mesh, Vertex}, Renderer, Vector3D};
use crate::utils::Vector4D;
use crate::utils::Vector2D;

pub struct Quad{
    pub size: Vector3D,
    pub color: Vector4D,
}

impl Quad{
    pub fn new(size: Vector3D, color: Vector4D) -> Self{
        Self{
            size,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer){
        let vertices = vec![
            Vertex::new(Vector3D::new(0., 0., 0.0), self.color),
            Vertex::new(Vector3D::new(0., self.size.y, 0.0), self.color),
            Vertex::new(Vector3D::new(self.size.x, 0., 0.0), self.color),
            Vertex::new(Vector3D::new(self.size.x, self.size.y, 0.0), self.color),
        ];

        let indices = vec![0, 2, 1, 2, 3, 1];
        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap();
    }
}

pub struct Circle{
    pub iterations: i32,
    pub radius: f32,
    pub color: Vector4D,
}

impl Circle {
    pub fn new(iterations: i32, radius: f32, color: Vector4D) -> Self{
        let mut fixed_iterations = iterations;
        if iterations <= 3{
            fixed_iterations = 4;
        }

        Self {
            iterations: fixed_iterations,
            radius,
            color,
        }
    }
    
    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        
        for i in 0..self.iterations {
            vertices.push(Vertex::new(Vector3D::new(f32::sin(2.*pi*i as f32/self.iterations as f32),
                                                    f32::cos(2.*pi*i as f32/self.iterations as f32),
                                                    1./self.radius)*self.radius,
                                                    self.color));
        }
        let mut indices = vec![];
        for i in 1..=self.iterations-2 {
            indices.push(0); 
            indices.push(i as u32); 
            indices.push((i % self.iterations + 1) as u32);
        }

        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap();
    }
}

pub struct Triangle{
    pub size: f32,
    pub color: Vector4D,
}

impl Triangle{
    pub fn new(size: f32, color: Vector4D) -> Self{
        Self {
            size,
            color,
        }
    }

    pub fn add_to_renderer(&self, name: &str, renderer: &mut Renderer) {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        for i in 0..3 {
            vertices.push(Vertex::new(Vector3D::new(f32::sin(2.*pi*i as f32/3. as f32),
                                                    f32::cos(2.*pi*i as f32/3. as f32),
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

pub struct Line{
    begin: Vector3D,
    end: Vector3D,
    width: f32,
    color: Vector4D,
}

impl Line{
    pub fn new(begin: Vector3D, end: Vector3D, width: f32, color: Vector4D) -> Self{
        Self{
            begin,
            end,
            width,
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
        renderer.add_mesh(name, Mesh::new(&vertices, &indices)).unwrap();
    }
}
