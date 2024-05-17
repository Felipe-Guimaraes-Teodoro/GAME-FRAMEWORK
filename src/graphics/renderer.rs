use std::collections::HashMap;

use crate::Vector3D;

use crate::utils::Vector4D;

use crate::utils::rand_betw;

use super::Mesh;

#[derive(PartialEq, Debug)]
pub struct Vertex {
    pub position: Vector3D,
    pub color: Vector4D,
}

impl Vertex {
    pub fn new(position: Vector3D, color: Vector4D) -> Self {
        Self {
            position,
            color,
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
