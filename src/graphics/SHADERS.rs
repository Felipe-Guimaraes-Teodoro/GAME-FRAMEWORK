pub static DEFAULT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord; // New input for texture coordinates

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

uniform vec3 pos;

uniform float time;

out vec4 fColor;
out vec2 TexCoord; // Pass texture coordinates to the fragment shader

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    fColor = aColor;
    TexCoord = aTexCoord; // Pass texture coordinates
}
"#;

pub static DEFAULT_MESH_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

in vec4 fColor;
in vec2 TexCoord; // Receive texture coordinates

uniform sampler2D texture1; // Texture sampler

void main()
{
    vec4 texColor = texture(texture1, TexCoord); // Sample the texture
    FragColor = texColor * fColor; // Combine texture color and vertex color
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

pub static LIGHT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

uniform vec3 pos;

uniform float time;

out vec4 fColor;
out vec2 TexCoord;
out vc3 Normal;
out vec3 crntPos;

void main() {
    gl_Position = proj * view * model * vec4(crntPos, 1.0);
    Normal = aNormal;
    fColor = aColor;
    TexCoord = aTexCoord;
    crntPos = vec3(model * vec4(aPos, 1.0f));
}
"#;

pub static LIGHT_MESH_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

in vec4 fColor;
in vec2 TexCoord;
in vec3 Normal;
in vec3 crntPos;

uniform sampler2D texture1;
uniform vec3 lightColor;
uniform vec3 lightPos;

void main()
{
    vec3 normal = normalize(Normal);
    vec3 lightDirection = normalize(lightPos - crntPos);

    float diffuse = max(dot(normal, lightDirection), 0.f);

    vec4 texColor = texture(texture1, TexCoord);
    FragColor = texColor * fColor * vec4(lightColor, 1.) * diffuse;
}
"#;
