use gl::UseProgram;
use glam::{vec3, Vec3};

use std::ffi::CString;

use crate::{cstr, Renderer, Shader};

#[derive(Copy, Clone)]
pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
}

impl Renderer {
    pub unsafe fn send_light_uniforms(&self, shader: &Shader) {
        shader.use_shader();
        shader.uniform_vec3f(cstr!("viewPos"), &self.camera.pos);
        shader.uniform_1i(cstr!("num_lights"), self.lights.len() as i32);
        let mut i = 0;
        for light in self.lights.values() {
            shader.uniform_vec3f(cstr!(format!("lightColor[{}]", i)), &light.color);
            shader.uniform_vec3f(cstr!(format!("lightPos[{}]", i)), &light.position);

            i+=1;
        }
        UseProgram(0);
    }

    pub fn add_light(&mut self, name: &str, light: Light) {
        self.lights.insert(name.to_string(), light);

    }
}