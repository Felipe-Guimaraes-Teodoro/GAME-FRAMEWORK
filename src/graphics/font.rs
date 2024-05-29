// https://learnopengl.com/code_viewer_gh.php?code=src/7.in_practice/2.text_rendering/text_rendering.cpp
use freetype as ft;
use ft::ffi::{FT_Done_Face, FT_Done_FreeType, FT_Load_Char, FT_Set_Pixel_Sizes, FT_LOAD_RENDER};
use std::{collections::HashMap, iter::Map, os::raw::c_void};

use gl::{BlendFunc, Enable, GenBuffers, UseProgram, ARRAY_BUFFER, BLEND, CULL_FACE, FALSE, FLOAT, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use glam::{Mat4, Vec2, Vec3};
use once_cell::sync::Lazy;

use std::ffi::CString;

use gl::types::{GLint, GLsizei};

use crate::{bind_buffer, cstr, Shader};

pub static TEXT_SHADER: Lazy<Shader> = Lazy::new(|| {
    Shader::new_pipeline(
            // VS
        "#version 330 core
        layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
        out vec2 TexCoords;
        
        uniform mat4 projection;
        
        void main()
        {
            gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);
            TexCoords = vertex.zw;
        }"
        ,  // FS
        "#version 330 core
        in vec2 TexCoords;
        out vec4 color;
        
        uniform sampler2D text;
        uniform vec3 textColor;
        
        void main()
        {    
            vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text, TexCoords).r);
            color = vec4(textColor, 1.0) * sampled;
        }"
    )
});

struct Character {
    texture: u32, 
    size: (i32, i32),
    bearing: (i32, i32),
    advance: u32,
}

pub struct Font {
    VBO: u32,
    VAO: u32,
    characters: HashMap<char, Character>,
}

impl Font {
    pub unsafe fn init(width: f32, height: f32, path: &str) -> Self {
        let (mut VBO, mut VAO) = (0, 0);
        let mut characters: HashMap<char, Character> = HashMap::new();

        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let projection = Mat4::orthographic_rh_gl(0.0, width, 0.0, height, -100.0, 100.0);
        TEXT_SHADER.use_shader();
        TEXT_SHADER.uniform_mat4fv(cstr!("projection"), &projection.to_cols_array());
        gl::UseProgram(0);

        let ft = ft::Library::init().expect("Failed to initialize FreeType library");
        let mut face = ft.new_face(path, 0).expect("Failed to load font face");
        FT_Set_Pixel_Sizes(face.raw_mut(), 0, 48);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        for c in 0..128u8 {
            if FT_Load_Char(face.raw_mut(), c as u32, FT_LOAD_RENDER) != 0 {
                eprintln!("Failed to load glyph for character {}", c);
                continue;
            }
            
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as GLint,
                face.glyph().bitmap().width() as GLsizei,
                face.glyph().bitmap().rows() as GLsizei,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                face.glyph().bitmap().buffer().as_ptr() as *const c_void,
            );
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            let character = Character {
                texture,
                size: (face.glyph().bitmap().width(), face.glyph().bitmap().rows()),
                bearing: (face.glyph().bitmap_left(), face.glyph().bitmap_top()),
                advance: face.glyph().advance().x as u32,
            };
            characters.insert(c as char, character);
        }
        
        // clean freetype resources (for some reason this causes heap corruption)
        //FT_Done_Face(face.raw_mut());
        //FT_Done_FreeType(ft.raw());
        
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<f32>() * 6 * 4) as isize, std::ptr::null(), gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE as gl::types::GLboolean, (4 * std::mem::size_of::<f32>()) as GLsizei, std::ptr::null());
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        Self {
            VBO,
            VAO,
            characters,
        }
    }

    pub fn render_text(&mut self, text: &str, mut x: f32, y: f32, scale: f32, color: Vec3) {
        unsafe {
            TEXT_SHADER.use_shader();
            TEXT_SHADER.uniform_vec3f(cstr!("textColor"), &color);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.VAO);
            
            for c in text.chars() {
                if let Some(ch) = self.characters.get(&c) {
                    let xpos = x + ch.bearing.0 as f32 * scale;
                    let ypos = y - (ch.size.1 - ch.bearing.1) as f32 * scale;
                    
                    let w = ch.size.0 as f32 * scale;
                    let h = ch.size.1 as f32 * scale;
                    
                    let vertices: [f32; 6 * 4] = [
                        xpos + w, ypos, 1.0, 1.0,
                        xpos, ypos, 0.0, 1.0,
                        xpos, ypos + h, 0.0, 0.0,
    
                        xpos + w, ypos + h, 1.0, 0.0,
                        xpos + w, ypos, 1.0, 1.0,
                        xpos, ypos + h, 0.0, 0.0,
                    ];
                        
                    gl::BindTexture(gl::TEXTURE_2D, ch.texture);
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.VBO);
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        (vertices.len() * std::mem::size_of::<f32>()) as isize,
                        vertices.as_ptr() as *const gl::types::GLvoid,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
    
                    x += (ch.advance >> 6) as f32 * scale; // Bitshift by 6 to get value in pixels (2^6 = 64)
                }
            }
    
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}