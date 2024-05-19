use std::collections::HashMap;

use glam::{Vec2, Vec3, Vec4};

use crate::EventLoop;


use crate::utils::rand_betw;

use super::Mesh;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec4) -> Self {
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
