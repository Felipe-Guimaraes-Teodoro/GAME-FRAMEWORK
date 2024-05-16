use std::collections::HashMap;

use crate::Vector3D;

use crate::utils::rand_betw;

use super::Mesh;

#[derive(PartialEq, Debug)]
pub struct Vertex {
    pub position: Vector3D,
    //pub color: Vector3D,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Vector3D::new(x, y, z),
            // color: Vector3D::new(rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), rand_betw(0.0, 1.0)),
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
