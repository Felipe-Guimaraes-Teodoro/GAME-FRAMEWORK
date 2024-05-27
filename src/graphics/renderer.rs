use std::collections::HashMap;

use gl::UseProgram;
use glam::{vec3, Vec2, Vec3, Vec4};

use crate::{cstr, Camera, EventLoop, InstanceMesh, Shader, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS, DEFAULT_SHADER, INSTANCE_MESH_SHADER_FS, INSTANCE_MESH_SHADER_VS, INSTANCE_SHADER, LIGHT_SHADER};
use std::ffi::CString;

use crate::utils::rand_betw;

use super::Mesh;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
    pub normal: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec4, normal: Vec3) -> Self {
        Self {
            position,
            color,
            normal,
        }
    }
}

pub struct Renderer {
    pub meshes: HashMap<String, Mesh>,
    pub instance_meshes: HashMap<String, InstanceMesh>,

    pub camera: Camera,
}

impl Renderer {
    pub fn new() -> Self {
        let mut camera = Camera::new();
        camera.set_projection(crate::ProjectionType::Orthographic);

        Self {
            meshes: HashMap::new(),
            instance_meshes: HashMap::new(),

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
        LIGHT_SHADER.uniform_vec3f(cstr!("viewPos"), &self.camera.pos);
        LIGHT_SHADER.uniform_vec3f(cstr!("lightColor"), &vec3(1.0, 1.0, 1.0));
        LIGHT_SHADER.uniform_vec3f(cstr!("lightPos"), &vec3(time.cos(), time.sin(), time.cos()));
        UseProgram(0);

        for value in &self.instance_meshes {
            value.1.draw(&el);
        }
        
        for value in &self.meshes {
            value.1.draw(&el);
        }
    }
}
