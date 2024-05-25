pub static DEFAULT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor; 

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

uniform vec3 pos;

uniform float  time;

out vec4 fColor;

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
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

pub static INSTANCE_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor; 

layout (location = 2) in mat4 model;

uniform mat4 view;
uniform mat4 proj;

out vec4 fColor;

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    // gl_Position = model * vec4(aPos, 1.0);
    fColor = aColor;
}
"#;

pub static INSTANCE_MESH_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

in vec4 fColor;

uniform mat4 view;

void main()
{
    FragColor = vec4(fColor);
}
"#;