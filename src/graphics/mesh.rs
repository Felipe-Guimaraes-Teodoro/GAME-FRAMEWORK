use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{Vector3D, cstr};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::*;

#[derive(PartialEq, Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

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

        BindBuffer(ARRAY_BUFFER, self.VBO);

        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        BufferData(ARRAY_BUFFER, size, data, STATIC_DRAW);

        BindBuffer(ELEMENT_ARRAY_BUFFER, self.EBO);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        BufferData(ELEMENT_ARRAY_BUFFER, size, data, STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;

        EnableVertexAttribArray(0);
        VertexAttribPointer(0, 3, FLOAT, FALSE, size, offset_of!(Vertex, position) as *const c_void);
        // EnableVertexAttribArray(1);
        // VertexAttribPointer(1, 3, FLOAT, FALSE, size, offset_of!(Vertex, color) as *const c_void);

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
    pub fn add_mesh(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<u32>) {
        // before adding a mesh with certain name, 
        // assure it has not been already added
        if self.meshes.get(name).is_some() { return };

        let mesh = Mesh::new(vertices, indices);
        self.meshes.insert(name.to_owned(), mesh);
    }

    pub fn get_mesh(&mut self, name: &str) -> Option<&mut Mesh> {
        self.meshes.get_mut(name)
    }
}