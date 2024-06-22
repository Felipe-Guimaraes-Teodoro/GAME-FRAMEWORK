use std::path::Path;

use glam::{vec2, vec3, vec4, Vec3, Vec4};
use tobj::LoadOptions;

use crate::{EventLoop, Mesh, Renderer, Texture, Vertex};

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub loaded_textures: Vec<Texture>,
}

impl Model {
    pub fn new(path: &str) -> Self {
        let mut model = Model::default();
        model.load(path);

        model
    }

    pub fn extract_mesh(&mut self, path: &str, u: usize) -> Mesh {
        let path = Path::new(path);
        
        let obj = tobj::load_obj(path, &LoadOptions::default()).expect("Failed to load OBJ file");
        let (models, _) = obj;

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;
            let indices: Vec<u32> = mesh.indices.clone();

            let mut vertices = Vec::with_capacity(num_vertices);

            let (p, n, t, c) = (&mesh.positions, &mesh.normals, &mesh.texcoords, &mesh.vertex_color);

            for i in 0..num_vertices {
                let pos = vec3(p[i*3], p[i*3+1], p[i*3+2]);
                let tex_coords = vec2(t[i*2], t[i*2+1]);
                let normal = if n.len() >= (i + 1) * 3 {
                    vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2])
                } else {
                    Vec3::ZERO
                };
                let color = if c.len() >= (i + 1) * 3 {
                    vec4(c[i * 3], c[i * 3 + 1], c[i * 3 + 2], 1.0)
                } else {
                    Vec4::ONE
                };
                vertices.push(
                    Vertex::new(pos, color, tex_coords, normal)
                );
            }

            
            self.meshes.push(Mesh::new(&vertices, &indices));
        }

        self.meshes[u].clone()
    }

    pub fn load(&mut self, path: &str) {
        let path = Path::new(path);

        let obj = tobj::load_obj(path, &LoadOptions::default()).expect("Failed to load OBJ file");
        let (models, _) = obj;

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;
            let indices: Vec<u32> = mesh.indices.clone();

            let mut vertices = Vec::with_capacity(num_vertices);

            let (p, n, t, c) = (&mesh.positions, &mesh.normals, &mesh.texcoords, &mesh.vertex_color);

            for i in 0..num_vertices {
                let pos = vec3(p[i*3], p[i*3+1], p[i*3+2]);
                let tex_coords = vec2(t[i*2], t[i*2+1]);
                let normal = if n.len() >= (i + 1) * 3 {
                    vec3(n[i * 3], n[i * 3 + 1], n[i * 3 + 2])
                } else {
                    Vec3::ZERO
                };
                let color = if c.len() >= (i + 1) * 3 {
                    vec4(c[i * 3], c[i * 3 + 1], c[i * 3 + 2], 1.0)
                } else {
                    Vec4::ONE
                };
                vertices.push(
                    Vertex::new(pos, color, tex_coords, normal)
                );
            }

            let mut final_mesh = Mesh::new(&vertices, &indices);

            for face in &mut final_mesh.indices.chunks_mut(6) {
                face.reverse();
            }

            final_mesh.setup_mesh();

            self.meshes.push(final_mesh);
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Texture {
        let texture = Texture::Path(path.to_owned());
        self.loaded_textures.push(texture.clone());
        texture
    }

    pub unsafe fn draw(&self) {
        for mesh in &self.meshes {
            mesh.draw();
        }
    } 
}


impl Renderer {
    pub fn add_model(&mut self, name: &str, model: Model) {
        self.models.insert(name.to_string(), model);
    }

    pub fn get_model(&self, name: &str) -> Option<&Model> {
        self.models.get(name)
    }

    pub fn get_model_mut(&mut self, name: &str) -> Option<&mut Model> {
        self.models.get_mut(name)
    }
}