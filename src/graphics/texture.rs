use std::{path::PathBuf, rc::Rc};

use gl::types::{GLint, GLsizei, GLuint, GLvoid};

#[derive(PartialEq, Debug, Clone)]
pub enum Texture{
    Path(String),
    Loaded(GLuint),
    None,
}

pub unsafe fn load_texture(path: &str) -> GLuint {
    let img = image::open(path).expect("Failed to load image");
    let img = img.flipv();
    let width = img.width();
    let height = img.height();
    let raw_pixels = img.to_rgba8().into_raw();

    let mut texture: GLuint = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as GLint,
        width as GLsizei,
        height as GLsizei,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        raw_pixels.as_ptr() as *const GLvoid,
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);

    texture
}
