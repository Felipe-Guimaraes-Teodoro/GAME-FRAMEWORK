use std::{ffi::c_void, mem::{offset_of, size_of}, ptr};

use crate::{bind_buffer, cstr, events::EventLoop, gen_attrib_pointers, Camera, INSTANCE_MESH_SHADER_FS, INSTANCE_MESH_SHADER_VS};
use std::ffi::CString;

use super::{Renderer, Shader, Vertex, DEFAULT_MESH_SHADER_FS, DEFAULT_MESH_SHADER_VS};

use gl::{*, types::GLsizei};
use glam::{vec3, Mat4, Quat, Vec2, Vec3, Vec4};
use once_cell::sync::Lazy;

pub static INSTANCE_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(INSTANCE_MESH_SHADER_VS, INSTANCE_MESH_SHADER_FS)
});


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct InstanceData {
    // opengl doesnt allow to send the entirety of the
    // matrix at once, so we decompose it
    x_axis: Vec4,
    y_axis: Vec4,
    z_axis: Vec4,
    w_axis: Vec4,
}

impl InstanceData {
    pub fn from_position(pos: Vec3) -> Self {
        let model = Mat4::from_translation(pos);

        Self {
            x_axis: model.x_axis,
            y_axis: model.y_axis,
            z_axis: model.z_axis,
            w_axis: model.w_axis,
        }
    }

    pub fn from_rotation(rot: Quat) -> Self {
        let model = Mat4::from_quat(rot);

        Self {
            x_axis: model.x_axis,
            y_axis: model.y_axis,
            z_axis: model.z_axis,
            w_axis: model.w_axis,
        }

    }

    pub fn from_scale(sca: Vec3) -> Self {
        let model = Mat4::from_scale(sca);

        Self {
            x_axis: model.x_axis,
            y_axis: model.y_axis,
            z_axis: model.z_axis,
            w_axis: model.w_axis,
        }
    }

    pub fn new(pos: Vec3, rot: Quat, sca: Vec3) -> Self {
        let model = 
            Mat4::from_translation(pos) *
            Mat4::from_quat(rot) *
            Mat4::from_scale(sca);

        Self {
            x_axis: model.x_axis,
            y_axis: model.y_axis,
            z_axis: model.z_axis,
            w_axis: model.w_axis,
        }
    }

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
            n,
            instance_data: vec![],
            shader: *INSTANCE_SHADER
        };

        // unsafe { mesh.setup_mesh() }

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

        GenBuffers(1, &mut self.instance_buffer);
        bind_buffer!(ARRAY_BUFFER, self.instance_buffer, self.instance_data);
        
        gen_attrib_pointers!(InstanceData, 2 => x_axis: 4);
        gen_attrib_pointers!(InstanceData, 3 => y_axis: 4);
        gen_attrib_pointers!(InstanceData, 4 => z_axis: 4);
        gen_attrib_pointers!(InstanceData, 5 => w_axis: 4);
        VertexAttribDivisor(2, 1); 
        VertexAttribDivisor(3, 1);  
        VertexAttribDivisor(4, 1);  
        VertexAttribDivisor(5, 1);  
        
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