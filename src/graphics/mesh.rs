use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{bind_buffer, cstr, gen_attrib_pointers, Vector3D};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};

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
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let shader = Shader::new_pipeline(DEFAULT_MESH_SHADER_VS, DEFAULT_MESH_SHADER_FS);
        let mut mesh = Mesh {
            vertices, indices,
            VAO: 0, VBO: 0, EBO: 0,
            position: Vector3D::ZERO,
            shader,
        };

        unsafe { mesh.setup_mesh() }

        mesh
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
    
    pub unsafe fn draw(&self) {
        BindVertexArray(self.VAO);
        self.shader.use_shader();
        self.shader.uniform_vec3f(cstr!("pos"), &self.position);
        DrawElements(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, ptr::null());
        BindVertexArray(0);
        UseProgram(0);
    }
}

impl Renderer {
    pub fn add_mesh_from_vertices_and_indices(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<u32>) {
        // before adding a mesh with certain name, 
        // assure it has not been already added
        if self.meshes.get(name).is_some() { return };

        let mesh = Mesh::new(vertices, indices);
        self.meshes.insert(name.to_owned(), mesh);
    }

    pub fn add_mesh(&mut self, name: &str, mesh:Mesh) {
        // before adding a mesh with certain name, 
        // assure it has not been already added
        if self.meshes.get(name).is_some() { return };

        self.meshes.insert(name.to_owned(), mesh);
    }

    pub fn get_mesh(&mut self, name: &str) -> Option<&mut Mesh> {
        self.meshes.get_mut(name)
    }
}