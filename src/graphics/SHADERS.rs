pub static DEFAULT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord; // New input for texture coordinates
layout (location = 3) in vec2 aNormal;

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

pub static LIGHT_MESH_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor; 
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

uniform vec3 pos;

uniform float  time;

out vec4 fColor;
out vec3 Normal;
out vec3 FragPos;

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    FragPos = vec3(model * vec4(aPos, 1.0));
    fColor = aColor;
    Normal = mat3(transpose(inverse(model))) * aNormal;  
}
"#;

pub static LIGHT_MESH_SHADER_FS: &str = r#"
#version 330 core

out vec4 FragColor;

in vec4 fColor;
in vec3 Normal;
in vec3 FragPos;  

uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;

void main()
{
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos); 

    float diff = max(dot(norm, lightDir), 0.0);

    vec3 diffuse = diff * lightColor;
    float specularStrength = 0.5;

    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm); 

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
    vec3 specular = specularStrength * spec * lightColor;  
    
    vec3 result = (ambient + diffuse + specular) * vec3(fColor);
    FragColor = vec4(result, 1.0);
}
"#;

pub static FULL_SHADER_VS: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

out vec4 fColor;
out vec3 Normal;
out vec3 FragPos;
out vec2 TexCoord; // Pass texture coordinates to the fragment shader

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    fColor = aColor;
    TexCoord = aTexCoord; // Pass texture coordinates
    FragPos = vec3(model * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(model))) * aNormal;  
}
"#;

pub static FULL_SHADER_FS: &str = r#"
#version 330 core
out vec4 FragColor;

in vec4 fColor;
in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;  

uniform vec3 lightColor[5];
uniform vec3 lightPos[5];
uniform vec3 viewPos;

uniform int has_texture;
uniform int num_lights;

uniform sampler2D texture1;

void main()
{
    // vec4 texColor = fColor;

    //if (has_texture == 1) {
        // }
    vec4 texColor = texture(texture1, TexCoord) * fColor;

    vec3 ambientStrength = vec3(0.1); 
    vec3 specularStrength = vec3(0.5);

    vec3 result = vec3(0.0);

    for (int i = 0; i < num_lights; ++i) {
        vec3 lightDir = normalize(lightPos[i] - FragPos); 
        vec3 norm = normalize(Normal);

        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor[i];

        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 reflectDir = reflect(-lightDir, norm); 

        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
        vec3 specular = specularStrength * spec * lightColor[i];

        result += (ambientStrength + diffuse + specular) * texColor.rgb;
    }

    FragColor = vec4(result, texColor.a);
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