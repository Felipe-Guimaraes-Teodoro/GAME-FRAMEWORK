use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{bind_buffer, cstr, events::EventLoop, gen_attrib_pointers, load_texture, Camera, InstanceData, InstanceMesh, ShaderType, Texture, LIGHT_MESH_SHADER_FS, LIGHT_MESH_SHADER_VS};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};
use glam::{vec3, Mat4, Quat, Vec3};
use once_cell::sync::Lazy;

pub static DEFAULT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(DEFAULT_MESH_SHADER_VS, DEFAULT_MESH_SHADER_FS)
});

pub static LIGHT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(LIGHT_MESH_SHADER_VS, LIGHT_MESH_SHADER_FS)
});

#[derive(PartialEq, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub VAO: u32,
    EBO: u32,
    VBO: u32,

    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,

    pub texture: Texture,

    pub normal: Vec3,

    shader: Shader,
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>, texture: Texture, shader_type: &ShaderType) -> Self {
        let shader: Shader;
        match shader_type{
            ShaderType::Default() => {
                shader = *DEFAULT_SHADER;
            }
            ShaderType::Light() => {
                shader = *LIGHT_SHADER;
            }
        }

        let mut mesh = Mesh {
            vertices: vertices.to_vec(), indices: indices.to_vec(),
            VAO: 0, VBO: 0, EBO: 0,
            position: Vec3::ZERO,
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
            scale: Vec3::ONE,
            texture,
            normal: vec3(1., 1., 1.),
            shader: shader,
        };

        unsafe { mesh.setup_mesh() }

        mesh
    }

    pub fn to_instance(&mut self, data: Vec<InstanceData>, n: usize) -> InstanceMesh {
        let mut new_mesh = InstanceMesh::new(&self.vertices, &self.indices, n);

        new_mesh.instance_data = data;

        unsafe { new_mesh.setup_mesh() };

        drop(self);

        new_mesh
    }

    pub fn set_position(&mut self, position: Vec3){
        self.position = position;
    }

    pub fn add_position(&mut self, position: Vec3){
        self.position += position;
    }

    pub fn scale(&mut self, scale: Vec3){
        self.scale *= scale;
    }

    pub unsafe fn setup_mesh(&mut self) {
        GenVertexArrays(1, &mut self.VAO);
        GenBuffers(1, &mut self.VBO);
        GenBuffers(1, &mut self.EBO);

        BindVertexArray(self.VAO);

        bind_buffer!(ARRAY_BUFFER, self.VBO, self.vertices);
        bind_buffer!(ELEMENT_ARRAY_BUFFER, self.EBO, self.indices);
        gen_attrib_pointers!(Vertex, 0 => position: 3, 1 => color: 4, 2 => tex_coords: 2, 3 => normal: 3);

        if let Texture::Path(ref path) = self.texture {
            self.texture = Texture::Loaded(load_texture(path));
        }
        else{
            self.texture = Texture::Loaded(0);
        }

        BindVertexArray(0);
    }
    
    pub unsafe fn draw(&self, el: &EventLoop) {
        let (w, h) = el.window.get_framebuffer_size();
        let resolution = w.max(h) as f32;

        let norm_position = self.position / resolution;
        let norm_scale = self.scale / resolution;

        let model_matrix = 
            Mat4::from_translation(norm_position) *
            Mat4::from_quat(self.rotation) *
            Mat4::from_scale(norm_scale);

        BindVertexArray(self.VAO);
        self.shader.use_shader();

        // Bind the texture if it is loaded
        if let Texture::Loaded(texture_id) = self.texture {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        // Set uniforms and draw
        self.shader.uniform_mat4fv(cstr!("model"), &model_matrix.to_cols_array());
        self.shader.uniform_vec3f(cstr!("pos"), &norm_position);
        
        self.shader.uniform_vec3f(cstr!("lightPos"), &vec3(0., 550., -600.));
        self.shader.uniform_vec3f(cstr!("lightColor"), &vec3(1., 0., 1.));

        DrawElements(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, ptr::null());

        BindVertexArray(0);
        UseProgram(0);
    }
}

impl Renderer {
    pub fn add_mesh_from_vertices_and_indices(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<u32>, texture: Texture, shader_type: &ShaderType) -> Result<(), String> {
        if self.meshes.contains_key(name) {
            return Err(format!("Mesh with name '{}' already exists", name));
        }

        let mesh = Mesh::new(&vertices, &indices, texture, shader_type);
        
        self.meshes.insert(name.to_owned(), mesh);
        Ok(())
    }

    pub fn add_mesh(&mut self, name: &str, mesh: Mesh) -> Result<(), String> {
        if self.meshes.contains_key(name) {
            return Err(format!("Mesh with name '{}' already exists", name));
        }

        self.meshes.insert(name.to_owned(), mesh);
        Ok(())
    }

    pub fn get_mesh_mut(&mut self, name: &str) -> Option<&mut Mesh> {
        self.meshes.get_mut(name)
    }

    pub fn get_mesh(&self, name: &str) -> Option<&Mesh> {
        self.meshes.get(name)
    }

    pub fn destroy_mesh(&mut self, name: &str) -> Result<(), String> {
        if self.meshes.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("No mesh found with name '{}'", name))
        }
    }
}


impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            DeleteVertexArrays(1, &self.VAO);
            DeleteBuffers(1, &self.EBO);
            DeleteBuffers(1, &self.VBO);
        }
    }
}