pub static DEFAULT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

// uniform mat4 model;
// uniform mat4 view;
// uniform mat4 projection;

uniform vec3 pos;

void main() {
    gl_Position = vec4(aPos + pos, 1.0);
}
"#;

pub static DEFAULT_MESH_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0);
}
"#;