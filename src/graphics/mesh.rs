use std::ptr;

use crate::{bind_buffer, cstr, events::EventLoop, gen_attrib_pointers, InstanceData, InstanceMesh, ShaderType, FULL_SHADER_FS, FULL_SHADER_VS, LIGHT_MESH_SHADER_FS, LIGHT_MESH_SHADER_VS};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};
use glam::{Mat4, Quat, Vec3, Vec4};
use once_cell::sync::Lazy;

pub static DEFAULT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(DEFAULT_MESH_SHADER_VS, DEFAULT_MESH_SHADER_FS)
});

pub static LIGHT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(LIGHT_MESH_SHADER_VS, LIGHT_MESH_SHADER_FS)
});

pub static FULL_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(FULL_SHADER_VS, FULL_SHADER_FS)
});

#[derive(PartialEq, Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub VAO: u32,
    EBO: u32,
    VBO: u32,

    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,

    pub texture: u32,
    shader: Shader,
    pub parent: Option<Box<Mesh>>,
    pub children: Vec<Box<Mesh>>,
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        let mesh = Mesh {
            vertices: vertices.to_vec(), indices: indices.to_vec(),
            VAO: 0, VBO: 0, EBO: 0,
            position: Vec3::ZERO,
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
            scale: Vec3::ONE,
            texture: 0,
            shader: *DEFAULT_SHADER,
            parent: None,
            children: Vec::new(),
        };

        mesh
    }

    pub fn set_shader_type(&mut self, shader_type: &ShaderType) {
        let shader: Shader;

        match shader_type{
            ShaderType::Default => {
                shader = *DEFAULT_SHADER;
            }
            ShaderType::Light => {
                shader = *LIGHT_SHADER;
            }
            ShaderType::Full => {
                shader = *FULL_SHADER;
            }
        }

        self.shader = shader;
    }

    pub fn set_texture(&mut self, texture_name: &str, renderer: &Renderer) {
        self.texture = renderer.get_texture(texture_name.to_owned());
        unsafe {
            self.shader.uniform_1i(cstr!("has_texture"), 1);
        }
    }

    pub fn to_instance(&mut self, data: Vec<InstanceData>, n: usize) -> InstanceMesh {
        let mut new_mesh = InstanceMesh::new(&self.vertices, &self.indices, n);

        new_mesh.instance_data = data;

        unsafe { new_mesh.setup_mesh() };

        drop(self);

        new_mesh
    }

    pub fn set_parent(&mut self, parent: Mesh){
        self.parent = Some(Box::new(parent));
    }

    pub fn add_child(&mut self, mut child: Mesh){
        child.set_parent(self.clone());
        self.children.push(Box::new(child));
    }

    pub fn set_color(&mut self, color: Vec4){
        for vert in self.vertices.iter_mut(){
            vert.color = color;
        }
    }

    pub fn set_position(&mut self, position: Vec3){
        self.position = position;
        for child in self.children.as_mut_slice(){
            child.set_position(position + child.position)
        }
    }

    pub fn add_position(&mut self, position: Vec3){
        self.position += position;
        for child in self.children.as_mut_slice(){
            child.add_position(position)
        }
    }

    pub fn set_scale(&mut self, scale: Vec3){
        self.scale = scale;
        for child in self.children.as_mut_slice(){
            child.set_scale(scale);
        }
    }

    pub fn scale(&mut self, scale: Vec3){
        self.scale *= scale;
        for child in self.children.as_mut_slice(){
            child.scale(scale);
        }
    }

    pub fn set_rotation(&mut self, rotation: Quat){
        self.rotation = rotation;
        for child in self.children.as_mut_slice(){
            child.set_rotation(rotation);
        }
    }

    pub fn rotate(&mut self, rotation: Quat){
        self.rotation = self.rotation + rotation;
        for child in self.children.as_mut_slice(){
            child.rotate(rotation);
        }
    }

    pub fn setup_mesh(&mut self) {
        unsafe {
            GenVertexArrays(1, &mut self.VAO);
            GenBuffers(1, &mut self.VBO);
            GenBuffers(1, &mut self.EBO);
    
            BindVertexArray(self.VAO);
    
            bind_buffer!(ARRAY_BUFFER, self.VBO, self.vertices);
            bind_buffer!(ELEMENT_ARRAY_BUFFER, self.EBO, self.indices);
            gen_attrib_pointers!(Vertex, 0 => position: 3, 1 => color: 4, 2 => tex_coords: 2, 3 => normal: 3);
    
            gl::BindTexture(gl::TEXTURE_2D, self.texture);

            BindVertexArray(0);
        }
    }
    
    pub unsafe fn draw(&self) {
        let model_matrix = 
            Mat4::from_translation(self.position) *
            Mat4::from_quat(self.rotation) *
            Mat4::from_scale(self.scale);

        BindVertexArray(self.VAO);
        self.shader.use_shader();


        // Set uniforms and draw
        self.shader.uniform_mat4fv(cstr!("model"), &model_matrix.to_cols_array());
        self.shader.uniform_vec3f(cstr!("pos"), &self.position);
        
        BindTexture(TEXTURE_2D, self.texture);

        DrawElements(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, ptr::null());

        BindVertexArray(0);
        UseProgram(0);
    }
}

impl Renderer {
    pub fn add_mesh_from_vertices_and_indices(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<u32>) -> Result<(), String> {
        if self.meshes.contains_key(name) {
            return Err(format!("Mesh with name '{}' already exists", name));
        }

        let mesh = Mesh::new(&vertices, &indices);
        
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