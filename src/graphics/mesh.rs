use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{bind_buffer, cstr, gen_attrib_pointers, Vector3D, Vector2D, events::EventLoop};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};
use once_cell::sync::Lazy;

pub static DEFAULT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(DEFAULT_MESH_SHADER_VS, DEFAULT_MESH_SHADER_FS)
});

#[derive(PartialEq, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub VAO: u32,
    EBO: u32,
    VBO: u32,

    pub position: Vector3D,

    shader: Shader,
}

impl Mesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Self {
        let mut mesh = Mesh {
            vertices: vertices.to_vec(), indices: indices.to_vec(),
            VAO: 0, VBO: 0, EBO: 0,
            position: Vector3D::ZERO,        
            shader: *DEFAULT_SHADER,
        };

        unsafe { mesh.setup_mesh() }

        mesh
    }

    pub fn set_shader(&mut self, shader: &Shader) {
        self.shader = *shader;
    }

    pub fn set_position(&mut self, position: Vector3D){
        self.position = position;
    }

    pub fn add_position(&mut self, position: Vector3D){
        self.position += position;
    }

    pub fn scale(&mut self, scale: f32){
        println!("{}, {}", self.position.x, self.position.y);
    }

    pub unsafe fn setup_mesh(&mut self) {
        GenVertexArrays(1, &mut self.VAO);
        GenBuffers(1, &mut self.VBO);
        GenBuffers(1, &mut self.EBO);

        BindVertexArray(self.VAO);

        bind_buffer!(ARRAY_BUFFER, self.VBO, self.vertices);
        bind_buffer!(ELEMENT_ARRAY_BUFFER, self.EBO, self.indices);
        gen_attrib_pointers!(Vertex, 0 => position: 3, 1 => color: 4);
        // gen_attrib_pointers!(Vertex, 0 => position: 3);

        BindVertexArray(0);
    
    }
    
    pub unsafe fn draw(&self, el: &EventLoop) {
        let (w, h) = el.window.get_framebuffer_size();
        let resolution = Vector3D::new(w as f32, h as f32, 1.0) / 2.0;
        
        BindVertexArray(self.VAO);
        self.shader.use_shader();
        self.shader.uniform_vec3f(cstr!("pos"), &(self.position / resolution));
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