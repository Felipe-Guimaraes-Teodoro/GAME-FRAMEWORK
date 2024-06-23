use std::{ptr, ffi::CString};

use gl::{*, types::*};
use glam::{vec2, vec3, Mat4, Quat, Vec3, Vec4};
use once_cell::sync::Lazy;

use crate::{bind_buffer, cstr, gen_attrib_pointers, rand_betw, rand_vec3, EventLoop, InstanceData, Renderer, Shader, Vertex, PARTICLE_SHADER_FS, PARTICLE_SHADER_VS};

pub static PARTICLE_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(PARTICLE_SHADER_VS, PARTICLE_SHADER_FS)
});

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ParticleInstanceData {
    pub position: Vec3,
    pub lifespan: f32,
    pub velocity: Vec3,
}


#[derive(Default)]
pub struct Particle {
    /* particle stuff */
    pub position: Vec3,
    pub velocity: Vec3,

    pub has_gravity: bool,
    pub size: f32,

    pub count: i32,

    pub spread: f32,

    /* rendering stuff */

    // image_path: String,
    pub mesh: ParticleMesh,
}

pub struct ParticleMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    pub VAO: u32,
    EBO: u32,
    VBO: u32,
    
    pub instance_data: Vec<ParticleInstanceData>,

    pub instance_buffer: u32,

    shader: Shader,
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self::new(5, Vec3::ZERO)
    }
}

impl ParticleMesh {
    pub fn new(count: i32, initial_position: Vec3) -> Self {
        let vertices = vec![
            Vertex::new(vec3(0.0, 0.0, 0.0), Vec4::ONE, vec2(0.0, 0.0), vec3(0., 0., 1.)), // Bottom-left
            Vertex::new(vec3(0.0, 1.0, 0.0), Vec4::ONE, vec2(0.0, 1.0), vec3(0., 0., 1.)), // Top-left
            Vertex::new(vec3(1.0, 0.0, 0.0), Vec4::ONE, vec2(1.0, 0.0), vec3(0., 0., 1.)), // Bottom-right
            Vertex::new(vec3(1.0, 1.0, 0.0), Vec4::ONE, vec2(1.0, 1.0), vec3(0., 0., 1.)), // Top-right
        ];

        let indices = vec![0, 2, 1, 2, 3, 1];

        let mut instance_data = vec![];
        for _ in 0..count {
            instance_data.push(ParticleInstanceData {
                position: initial_position,
                lifespan: 1.0, 
                velocity: Vec3::ZERO,
            });
        }

        let mut mesh = Self {
            vertices,
            indices,
            VAO: 0,
            VBO: 0,
            EBO: 0,
            instance_buffer: 0,
            instance_data,
            shader: *PARTICLE_SHADER,
        };

        mesh
    }

    pub fn update_instance_data(&mut self, data: Vec<ParticleInstanceData>) {
        self.instance_data = data;
        unsafe {
            BindBuffer(ARRAY_BUFFER, self.instance_buffer);
            BufferSubData(
                ARRAY_BUFFER, 
                0, 
                (self.instance_data.len() * std::mem::size_of::<ParticleInstanceData>()) as isize, 
                self.instance_data.as_ptr() as *const _
            );
        }
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
        
        gen_attrib_pointers!(ParticleInstanceData, 2 => position: 3);
        VertexAttribDivisor(2, 1);
        
        BindVertexArray(0);
    }

    pub fn destroy(&mut self) {
        unsafe {
            DeleteVertexArrays(1, &self.VAO);
            DeleteBuffers(1, &self.EBO);
            DeleteBuffers(1, &self.VBO);
        }
    }

    pub unsafe fn draw(&self, model_matrix: Mat4) {
        BindVertexArray(self.VAO);
        self.shader.use_shader();

        self.shader.uniform_mat4fv(cstr!("model"), &model_matrix.to_cols_array());

        DrawElementsInstanced(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, ptr::null(), self.instance_data.len() as i32);
        BindVertexArray(0);
        UseProgram(0);
    }
}


impl Particle {
    pub fn new(
        position: Vec3, 
        velocity: Vec3, 
        count: i32,
        size: f32,
        has_gravity: bool,
        spread: f32,
    ) -> Self {
        let mut mesh = ParticleMesh::new(count, position);

        let mut subscribed_instance_data = vec![];
        for _ in 0..count {
            subscribed_instance_data.push(ParticleInstanceData {
                position: Vec3::ZERO,
                velocity: velocity * rand_vec3() * spread,
                lifespan: rand_betw(0.0, 5.0), // initialize with some random lifespam
            });
        }

        mesh.update_instance_data(subscribed_instance_data);
        unsafe { mesh.setup_mesh(); };

        Self {
            position,
            velocity,
            count,
            size,
            has_gravity,
            spread,
            mesh,
        }
    }

    pub fn update(&mut self, el: &EventLoop) {
        let instance_data = &self.mesh.instance_data;
        let mut subscribed_instance_data = vec![];

        for i in 0..self.count {
            let mut particle = instance_data[i as usize];
            particle.position += particle.velocity * el.dt * 1.0 / self.size;
            particle.lifespan -= el.dt;

            if particle.lifespan <= 0.0 {
                particle.position = Vec3::ZERO;
                particle.velocity = self.velocity * rand_vec3() * self.spread;
                particle.lifespan = rand_betw(0.5, 5.0);
            }
            if self.has_gravity {
                particle.velocity.y -= 1.0 * el.dt;                
            }
            subscribed_instance_data.push(particle);
        }

        self.mesh.update_instance_data(subscribed_instance_data);
        // gg
    }


    pub fn draw(&self) {
        let model_matrix = 
            Mat4::from_translation(self.position) *
            Mat4::from_quat(Quat::default()) *
            Mat4::from_scale(vec3(self.size, self.size, self.size));
        
            
        unsafe {
            self.mesh.draw(model_matrix);
        }
    }
}

impl Renderer {
    pub fn add_particle(&mut self, name: &str, particle: Particle) {
        self.particles.insert(name.to_owned(), particle);// .unwrap();
    }

    pub fn remove_particle(&mut self, name: &str) {
        let particle = self.particles.get_mut(name).unwrap();
        particle.mesh.destroy(); // gotta make sure to clear the opengl's resources first
        self.particles.remove(name).unwrap();
    }
}