pub static DEFAULT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor; 

// uniform mat4 model;
// uniform mat4 view;
// uniform mat4 projection;

uniform vec3 pos;

out vec4 fColor;

void main() {
    gl_Position = vec4(aPos + pos, 1.0);
    fColor = aColor;
}
"#;

pub static DEFAULT_MESH_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

in vec4 fColor;

void main()
{
    FragColor = vec4(fColor);
}
"#;