use glam::{vec2, vec3, Vec3, Vec4};
use once_cell::sync::Lazy;

use crate::{graphics::{Mesh, Vertex}, Renderer, Shader, ShaderType, Texture};

pub struct Quad{
    pub size: Vec3,
    pub color: Vec4,
    pub texture: Texture,
    shader_type: ShaderType,
}

impl Quad{
    pub fn new(size: Vec3, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
        Self{
            size,
            color,
            texture,
            shader_type,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let vertices = vec![
            Vertex::new(vec3(0.0, 0.0, 0.0), self.color, vec2(0.0, 0.0)),                    // Bottom-left
            Vertex::new(vec3(0.0, self.size.y, 0.0), self.color, vec2(0.0, 1.0)),             // Top-left
            Vertex::new(vec3(self.size.x, 0.0, 0.0), self.color, vec2(1.0, 0.0)),             // Bottom-right
            Vertex::new(vec3(self.size.x, self.size.y, 0.0), self.color, vec2(1.0, 1.0)),      // Top-right
        ];


        let indices = vec![0, 2, 1, 2, 3, 1];
        
        Mesh::new(&vertices, &indices, self.texture.clone(), &self.shader_type)
    }
}

pub struct Circle{
    pub iterations: i32,
    pub radius: f32,
    pub color: Vec4,
    pub texture: Texture,
    shader_type: ShaderType,
}

impl Circle {
    pub fn new(iterations: i32, radius: f32, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
        let mut fixed_iterations = iterations;
        if iterations <= 3{
            fixed_iterations = 4;
        }

        Self {
            iterations: fixed_iterations,
            radius,
            color,
            texture,
            shader_type,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        
        for i in 0..self.iterations {
            let angle = 2.0 * std::f32::consts::PI * (i as f32 / self.iterations as f32);
            let tex_coord = vec2(f32::cos(angle) * 0.5 + 0.5, f32::sin(angle) * 0.5 + 0.5); // Normalize to [0, 1] range
        
            vertices.push(Vertex::new(
                vec3(f32::sin(angle), f32::cos(angle), 0.0) * self.radius,
                self.color,
                tex_coord,
            ));
        }
        
        let mut indices = vec![];
        for i in 1..=self.iterations-2 {
            indices.push(0); 
            indices.push(i as u32); 
            indices.push((i % self.iterations + 1) as u32);
        }

        Mesh::new(&vertices, &indices, self.texture.clone(), &self.shader_type)
    }
}

pub struct Triangle{
    pub size: f32,
    pub color: Vec4,
    pub texture: Texture,
    shader_type: ShaderType,
}

impl Triangle{
    pub fn new(size: f32, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
        Self {
            size,
            color,
            texture,
            shader_type,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;
        for i in 0..3 {
            let angle = 2.0 * std::f32::consts::PI * (i as f32 / 3.0);
            let tex_coord = match i {
                0 => vec2(0.5, 1.0),   // Bottom vertex
                1 => vec2(0.0, 0.0),   // Left vertex
                2 => vec2(1.0, 0.0),   // Right vertex
                _ => panic!("Unexpected index"),
            };
        
            vertices.push(Vertex::new(
                vec3(f32::sin(angle), f32::cos(angle), 0.0) * self.size,
                self.color,
                tex_coord,
            ));
        }

        let mut indices = vec![];
            indices.push(0); 
            indices.push(1 as u32); 
            indices.push(2 as u32);
            // Shamelessly (ok theres a bit of shame) stole my own circle rendering code so I just set it to three vertices

        Mesh::new(&vertices, &indices, self.texture.clone(), &self.shader_type)
    }
}

pub struct Line{
    begin: Vec3,
    end: Vec3,
    width: f32,
    color: Vec4,
    texture: Texture,
    shader_type: ShaderType,
}

// yeah we doin this later for sure for sure

// impl Line{
//     pub fn new(begin: Vec3, end: Vec3, width: f32, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
//         Self{
//             begin,
//             end,
//             width,
//             color,
//             texture,
//             shader_type,
//         }
//     }

//     pub fn mesh(&self) -> Mesh{
//         let vertices = vec![
//             Vertex::new(vec3(self.begin.x+self.width, self.begin.y+self.width, 0.0), self.color),
//             Vertex::new(vec3(self.end.x+self.width, self.end.y+self.width, 0.0), self.color),
//             Vertex::new(vec3(self.begin.x-self.width, self.begin.y-self.width, 0.0), self.color),
//             Vertex::new(vec3(self.end.x-self.width, self.end.y-self.width, 0.0), self.color),
//         ];

//         let indices = vec![2, 1, 0, 2, 1, 3];
//         Mesh::new(&vertices, &indices, self.texture.clone())

//             // let mut vertices: Vec<Vertex> = Vec::new();
//             // let mut indices: Vec<u32> = Vec::new();

//             // let WIDTH = 300.;

//             // let x1 = self.begin[0];
//             // let x2 = self.end[0];
//             // let y1 = self.begin[1];
//             // let y2 = self.end[1];

//             // let dx = x2 - x1;
//             // let dy = y2 - y1;
//             // let l = dx.hypot (dy)/WIDTH;
//             // let u = dx * WIDTH * 0.5 / l / WIDTH;
//             // let v = dy * WIDTH * 0.5 / l / WIDTH;

//             // vertices.push(Vertex { position: vec3(x1 + v,  y1 - u, 0.0), color: self.color });
//             // vertices.push(Vertex { position: vec3(x1 - v,  y1 + u, 0.0), color: self.color });
//             // vertices.push(Vertex { position: vec3(x2 - v,  y2 + u, 0.0), color: self.color });
//             // vertices.push(Vertex { position: vec3(x2 + v,  y2 - u, 0.0), color: self.color });

//             // indices.push(2);
//             // indices.push(1);
//             // indices.push(0);
//             // indices.push(2);
//             // indices.push(0);
//             // indices.push(3);

//             // renderer.add_mesh(name, Mesh::new(&vertices, &indices, self.texture, &self.shader_type)).unwrap();
//     }
// }

pub struct Cuboid{
    pub size: Vec3,
    pub color: Vec4,
    pub texture: Texture,
    shader_type: ShaderType,
}

impl Cuboid{
    pub fn new(size: Vec3, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
        Self{
            size,
            color,
            texture,
            shader_type,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let half_size = self.size/2.;
        let x = half_size.x;
        let y = half_size.y;
        let z = half_size.z;

        let vertices = vec![
            Vertex::new(vec3(-x, -y, z), self.color, vec2(0.0, 0.0)),    // 0
            Vertex::new(vec3(x, -y, z), self.color, vec2(1.0, 0.0)),     // 1
            Vertex::new(vec3(x, y, z), self.color, vec2(1.0, 1.0)),      // 2
            Vertex::new(vec3(-x, y, z), self.color, vec2(0.0, 1.0)),     // 3

            // Back face
            Vertex::new(vec3(-x, -y, -z), self.color, vec2(0.0, 0.0)),   // 4
            Vertex::new(vec3(x, -y, -z), self.color, vec2(1.0, 0.0)),    // 5
            Vertex::new(vec3(x, y, -z), self.color, vec2(1.0, 1.0)),     // 6
            Vertex::new(vec3(-x, y, -z), self.color, vec2(0.0, 1.0)),    // 7

            // Left face
            Vertex::new(vec3(-x, -y, -z), self.color, vec2(0.0, 0.0)),   // 8
            Vertex::new(vec3(-x, -y, z), self.color, vec2(1.0, 0.0)),    // 9
            Vertex::new(vec3(-x, y, z), self.color, vec2(1.0, 1.0)),     // 10
            Vertex::new(vec3(-x, y, -z), self.color, vec2(0.0, 1.0)),    // 11

            // Right face
            Vertex::new(vec3(x, -y, -z), self.color, vec2(0.0, 0.0)),    // 12
            Vertex::new(vec3(x, -y, z), self.color, vec2(1.0, 0.0)),     // 13
            Vertex::new(vec3(x, y, z), self.color, vec2(1.0, 1.0)),      // 14
            Vertex::new(vec3(x, y, -z), self.color, vec2(0.0, 1.0)),     // 15

            // Top face
            Vertex::new(vec3(-x, y, -z), self.color, vec2(0.0, 0.0)),    // 16
            Vertex::new(vec3(x, y, -z), self.color, vec2(1.0, 0.0)),     // 17
            Vertex::new(vec3(x, y, z), self.color, vec2(1.0, 1.0)),      // 18
            Vertex::new(vec3(-x, y, z), self.color, vec2(0.0, 1.0)),     // 19

            // Bottom face
            Vertex::new(vec3(-x, -y, -z), self.color, vec2(0.0, 0.0)),   // 20
            Vertex::new(vec3(x, -y, -z), self.color, vec2(1.0, 0.0)),    // 21
            Vertex::new(vec3(x, -y, z), self.color, vec2(1.0, 1.0)),     // 22
            Vertex::new(vec3(-x, -y, z), self.color, vec2(0.0, 1.0)),    // 23
        ];

        let indices = vec![
            // Front face
            0, 1, 2,    // Triangle 1
            2, 3, 0,    // Triangle 2

            // Back face
            4, 5, 6,    // Triangle 3
            6, 7, 4,    // Triangle 4

            // Left face
            8, 9, 10,   // Triangle 5
            10, 11, 8,  // Triangle 6

            // Right face
            12, 13, 14, // Triangle 7
            14, 15, 12, // Triangle 8

            // Top face
            16, 17, 18, // Triangle 9
            18, 19, 16, // Triangle 10

            // Bottom face
            20, 21, 22, // Triangle 11
            22, 23, 20, // Triangle 12
        ];
        
        Mesh::new(&vertices, &indices, self.texture.clone(), &self.shader_type)
    }
}

pub struct Sphere{
    pub iterations: i32,
    pub radius: f32,
    pub color: Vec4,
    pub texture: Texture,
    shader_type: ShaderType,
}

impl Sphere {
    pub fn new(iterations: i32, radius: f32, color: Vec4, texture: Texture, shader_type: ShaderType) -> Self{
        let mut fixed_iterations = iterations;
        if iterations <= 3{
            fixed_iterations = 4;
        }

        Self {
            iterations: fixed_iterations,
            radius,
            color,
            texture,
            shader_type,
        }
    }

    pub fn mesh(&self) -> Mesh {
        let mut vertices = vec![];
        let pi = std::f32::consts::PI;

        for lat in 0..=self.iterations {
            let theta = pi * lat as f32 / self.iterations as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=self.iterations {
                let phi = 2.0 * pi * lon as f32 / self.iterations as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta * self.radius;
                let y = cos_theta * self.radius;
                let z = sin_phi * sin_theta * self.radius;

                // Calculate texture coordinates (cylindrical projection)
                let s = lon as f32 / self.iterations as f32;
                let t = 1.0 - (lat as f32 / self.iterations as f32);

                vertices.push(Vertex::new(vec3(x, y, z), self.color, vec2(s, t)));
            }
        }

        let mut indices = vec![];
        for lat in 0..self.iterations {
            for lon in 0..self.iterations {
                let first = lat * (self.iterations + 1) + lon;
                let second = first + self.iterations + 1;

                indices.push(first as u32);
                indices.push(second as u32);
                indices.push((first + 1) as u32);

                indices.push(second as u32);
                indices.push((second + 1) as u32);
                indices.push((first + 1) as u32);
            }
        }

        Mesh::new(&vertices, &indices, self.texture.clone(), &self.shader_type)
    }
}