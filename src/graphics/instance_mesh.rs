use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{bind_buffer, cstr, events::EventLoop, gen_attrib_pointers, INSTANCE_MESH_SHADER_FS, INSTANCE_MESH_SHADER_VS};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};
use glam::{vec3, Mat4, Quat, Vec2, Vec3};
use once_cell::sync::Lazy;

pub static INSTANCE_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(INSTANCE_MESH_SHADER_VS, INSTANCE_MESH_SHADER_FS)
});


#[derive(PartialEq, Debug)]
pub struct InstanceData {
    // pub position: Vec3,
    // pub rotation: Quat,
    // pub scale: Vec3,

    // model: Vec<[[f32; 4]; 4]>,
    pub model: Vec2,
}

impl InstanceData {

}

#[derive(PartialEq, Debug)]
pub struct InstanceMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    pub VAO: u32,
    EBO: u32,
    VBO: u32,

    pub instance_buffer: u32,

    pub n: usize,

    pub instance_data: Vec<InstanceData>,

    shader: Shader,
}

impl InstanceMesh {
    pub fn new(vertices: &Vec<Vertex>, indices: &Vec<u32>, n: usize) -> Self {
        let mut mesh = InstanceMesh {
            vertices: vertices.to_vec(), indices: indices.to_vec(),
            VAO: 0, VBO: 0, EBO: 0, 
            instance_buffer: 0,
            shader: *INSTANCE_SHADER,
            n,
            instance_data: vec![],
        };

        // unsafe { mesh.setup_mesh() }

        mesh
    }

    pub fn set_shader(&mut self, shader: &Shader) {
        self.shader = *shader;
    }

    pub unsafe fn setup_mesh(&mut self) {
        GenVertexArrays(1, &mut self.VAO);
        GenBuffers(1, &mut self.VBO);
        GenBuffers(1, &mut self.EBO);
        
        BindVertexArray(self.VAO);
        
        bind_buffer!(ARRAY_BUFFER, self.VBO, self.vertices);
        bind_buffer!(ELEMENT_ARRAY_BUFFER, self.EBO, self.indices);
        gen_attrib_pointers!(Vertex, 0 => position: 3, 1 => color: 4);

        GenBuffers(1, &mut self.instance_buffer);
        bind_buffer!(ARRAY_BUFFER, self.instance_buffer, self.instance_data);
        
        gen_attrib_pointers!(InstanceData, 2 => model: 2);
        VertexAttribDivisor(2, 1);  
        
        BindVertexArray(0);
    }

    pub unsafe fn draw(&self, el: &EventLoop) {
        let (w, h) = el.window.get_framebuffer_size();

        BindVertexArray(self.VAO);
        self.shader.use_shader();
        DrawElementsInstanced(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, ptr::null(), self.n as i32);
        BindVertexArray(0);
        UseProgram(0);
    }
}

impl Renderer {
    /* 
    pub fn add_instance_mesh_from_vertices_and_indices(&mut self, name: &str, vertices: Vec<Vertex>, indices: Vec<u32>) -> Result<(), String> {
        if self.meshes.contains_key(name) {
            return Err(format!("Mesh with name '{}' already exists", name));
        }

        let mesh = InstanceMesh::new(&vertices, &indices, 1); // change  this shid later
        self.meshes.insert(name.to_owned(), mesh);
        Ok(())
    }
    */
    pub fn add_instance_mesh(&mut self, name: &str, mesh: InstanceMesh) -> Result<(), String> {
        if self.instance_meshes.contains_key(name) {
            return Err(format!("Mesh with name '{}' already exists", name));
        }

        self.instance_meshes.insert(name.to_owned(), mesh);
        Ok(())
    }

    /* 
    pub fn get_instance_mesh_mut(&mut self, name: &str) -> Option<&mut InstanceMesh> {
        self.meshes.get_mut(name)
    }

    pub fn get_instance_mesh(&self, name: &str) -> Option<&InstanceMesh> {
        self.meshes.get(name)
    }

    pub fn destroy_instance_mesh(&mut self, name: &str) -> Result<(), String> {
        if self.meshes.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("No mesh found with name '{}'", name))
        }
    }
    */
}

impl Drop for InstanceMesh {
    fn drop(&mut self) {
        unsafe {
            DeleteVertexArrays(1, &self.VAO);
            DeleteBuffers(1, &self.EBO);
            DeleteBuffers(1, &self.VBO);
        }
    }
}