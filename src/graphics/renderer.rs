use std::collections::HashMap;

use crate::Vector3D;

use super::Mesh;

pub struct Vertex {
    pub position: Vector3D,
    pub color: Vector3D,
}

pub struct Renderer {
    meshes: HashMap<Mesh, String>,
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

    }
}
