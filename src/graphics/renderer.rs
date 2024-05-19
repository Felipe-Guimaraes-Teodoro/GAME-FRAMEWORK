use std::collections::HashMap;

use crate::Vector2D;
use crate::Vector3D;

use crate::EventLoop;

use crate::utils::Vector4D;

use crate::utils::rand_betw;

use super::Mesh;

#[derive(PartialEq, Debug, Clone, Copy)]
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

    pub unsafe fn draw(&self, el: &EventLoop) {
        for value in &self.meshes {
            value.1.draw(&el);
        }
    }
}
