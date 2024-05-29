use std::{collections::HashMap, ffi::CString};

use gl::UseProgram;
use glam::{vec3, Vec2, Vec3, Vec4};

use crate::{cstr, load_texture, Camera, EventLoop, InstanceMesh, Light, Shader, Texture, DEFAULT_SHADER, FULL_SHADER, INSTANCE_SHADER, LIGHT_SHADER};

use super::Mesh;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
    pub tex_coords: Vec2,
    pub normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec4, tex_coords: Vec2, normal: Vec3) -> Self {
        Self {
            position,
            color,
            tex_coords,
            normal,
        }
    }
}

pub struct Renderer {
    pub meshes: HashMap<String, Mesh>,
    pub instance_meshes: HashMap<String, InstanceMesh>,
    pub lights: HashMap<String, Light>,

    pub camera: Camera,
}

impl Renderer {
    pub fn new() -> Self {
        let mut camera = Camera::new();
        camera.set_projection(crate::ProjectionType::Orthographic);

        Self {
            meshes: HashMap::new(),
            instance_meshes: HashMap::new(),
            lights: HashMap::new(),

            camera,
        }
    }

    pub unsafe fn draw(&self, el: &EventLoop) {
        let time = el.time;

        INSTANCE_SHADER.use_shader();
        self.camera.send_uniforms(&INSTANCE_SHADER);
        UseProgram(0);

        DEFAULT_SHADER.use_shader();
        self.camera.send_uniforms(&DEFAULT_SHADER);
        UseProgram(0);

        LIGHT_SHADER.use_shader();
        self.camera.send_uniforms(&LIGHT_SHADER);
        self.send_light_uniforms(&LIGHT_SHADER);
        UseProgram(0);

        FULL_SHADER.use_shader();
        self.camera.send_uniforms(&FULL_SHADER);
        self.send_light_uniforms(&FULL_SHADER);
        UseProgram(0);

        for value in &self.instance_meshes {
            value.1.draw(&el);
        }
        
        for value in &self.meshes {
            value.1.draw(&el);
        }
    }
}
