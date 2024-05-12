use std::collections::HashMap;

use crate::Vector3D;

use super::Mesh;

#[derive(PartialEq, Debug)]
pub struct Vertex {
    pub position: Vector3D,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3D::new(x, y, z),
        }
    }
}

pub struct Renderer {
    pub meshes: HashMap<String, Mesh>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        
    }

    pub unsafe fn draw(&self) {
        for value in &self.meshes {
            value.1.draw();
        }
    }
}
