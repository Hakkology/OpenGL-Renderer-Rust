
pub enum VertexShaderKind {
    Standard,
    Texture,
    ModelViewProjection,
    UI,
    Lit,
    Skybox,
    File(String),
    Custom(String),
}

impl VertexShaderKind {
    pub fn get_source(&self) -> String {
        match self {
            VertexShaderKind::Standard => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec3 aColor;

                out vec3 ourColor;

                void main() {
                    gl_Position = vec4(aPos, 1.0);
                    ourColor = aColor; 
                }
            "#.to_string(),
            VertexShaderKind::Texture => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec3 aColor;
                layout (location = 2) in vec2 aTexCoord;

                out vec3 ourColor;
                out vec2 TexCoord;

                void main() {
                    gl_Position = vec4(aPos, 1.0);
                    ourColor = aColor;
                    TexCoord = aTexCoord;
                }
            "#.to_string(),
            VertexShaderKind::ModelViewProjection => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec3 aColor;
                layout (location = 2) in vec2 aTexCoord;

                out vec3 ourColor;
                out vec2 TexCoord;

                uniform mat4 model;
                uniform mat4 view;
                uniform mat4 projection;

                void main() {
                    gl_Position = projection * view * model * vec4(aPos, 1.0);
                    ourColor = aColor;
                    TexCoord = aTexCoord;
                }
            "#.to_string(),
            VertexShaderKind::Lit => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec3 aNormal;
                layout (location = 2) in vec2 aTexCoord;

                out vec3 Normal;
                out vec3 FragPos;
                out vec2 TexCoord;

                uniform mat4 model;
                uniform mat4 view;
                uniform mat4 projection;

                void main() {
                    FragPos = vec3(model * vec4(aPos, 1.0));
                    Normal = mat3(transpose(inverse(model))) * aNormal;
                    TexCoord = aTexCoord;
                    gl_Position = projection * view * vec4(FragPos, 1.0);
                }
            "#.to_string(),
            VertexShaderKind::UI => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                layout (location = 1) in vec2 aTexCoord;

                out vec2 TexCoord;

                uniform mat4 projection; // Orthographic

                void main() {
                    gl_Position = projection * vec4(aPos.xy, 0.0, 1.0);
                    TexCoord = aTexCoord;
                }
            "#.to_string(),
            VertexShaderKind::Skybox => r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                out vec3 TexCoords;
                uniform mat4 projection;
                uniform mat4 view;
                void main() {
                    TexCoords = aPos;
                    vec4 pos = projection * view * vec4(aPos, 1.0);
                    gl_Position = pos.xyww;
                }
            "#.to_string(),
            VertexShaderKind::File(path) => {
                std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read vertex shader file: {}", path))
            },
            VertexShaderKind::Custom(src) => src.clone(),
        }
    }
}

pub enum FragmentShaderKind {
    ColorFromUniform,
    VertexColor,
    LitDirectional,
    LitTextured,
    UIText,
    UIColor,
    Texture,
    Skybox,
    TextureWithColorMix,
    File(String),
    Custom(String),
}

impl FragmentShaderKind {
    pub fn get_source(&self) -> String {
        match self {
            FragmentShaderKind::ColorFromUniform => r#"
                #version 330 core
                out vec4 FragColor;
                
                uniform vec4 u_Color;

                void main() {
                    FragColor = u_Color;
                }
            "#.to_string(),
             FragmentShaderKind::VertexColor => r#"
                #version 330 core
                out vec4 FragColor;
                in vec3 ourColor;
                void main() {
                    FragColor = vec4(ourColor * 0.5 + 0.5, 1.0);
                }
            "#.to_string(),
            FragmentShaderKind::LitDirectional => r#"
                #version 330 core
                out vec4 FragColor;

                in vec3 Normal;
                in vec3 FragPos;

                uniform vec3 lightDir;
                uniform vec3 lightColor;
                uniform float ambientStrength;
                uniform float diffuseStrength;
                uniform float specularStrength;
                uniform float shininess;
                uniform vec3 viewPos;
                uniform vec3 objectColor;

                void main() {
                    // Ambient
                    vec3 ambient = ambientStrength * lightColor;

                    // Diffuse 
                    vec3 norm = normalize(Normal);
                    vec3 lightDirNorm = normalize(-lightDir);
                    float diff = max(dot(norm, lightDirNorm), 0.0);
                    vec3 diffuse = diffuseStrength * diff * lightColor;

                    // Specular
                    vec3 viewDir = normalize(viewPos - FragPos);
                    vec3 reflectDir = reflect(-lightDirNorm, norm);
                    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
                    vec3 specular = specularStrength * spec * lightColor;

                    vec3 result = (ambient + diffuse + specular) * objectColor;
                    FragColor = vec4(result, 1.0);
                }
            "#.to_string(),
            FragmentShaderKind::LitTextured => r#"
                #version 330 core
                out vec4 FragColor;

                in vec3 Normal;
                in vec3 FragPos;
                in vec2 TexCoord;

                uniform vec3 lightDir;
                uniform vec3 lightColor;
                uniform float ambientStrength;
                uniform float diffuseStrength;
                uniform float specularStrength;
                uniform float shininess;
                uniform vec3 viewPos;
                uniform sampler2D u_Texture;

                void main() {
                    vec4 texColor = texture(u_Texture, TexCoord);

                    // Ambient
                    vec3 ambient = ambientStrength * lightColor;

                    // Diffuse 
                    vec3 norm = normalize(Normal);
                    vec3 lightDirNorm = normalize(-lightDir);
                    float diff = max(dot(norm, lightDirNorm), 0.0);
                    vec3 diffuse = diffuseStrength * diff * lightColor;

                    // Specular
                    vec3 viewDir = normalize(viewPos - FragPos);
                    vec3 reflectDir = reflect(-lightDirNorm, norm);
                    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
                    vec3 specular = specularStrength * spec * lightColor;

                    vec3 result = (ambient + diffuse + specular) * texColor.rgb;
                    FragColor = vec4(result, texColor.a);
                }
            "#.to_string(),
            FragmentShaderKind::UIText => r#"
                #version 330 core
                in vec2 TexCoord;
                out vec4 FragColor;

                uniform sampler2D u_Texture;
                uniform vec3 u_Color;

                void main() {
                    float sampled = texture(u_Texture, TexCoord).r;
                    FragColor = vec4(u_Color, sampled);
                }
            "#.to_string(),
            FragmentShaderKind::UIColor => r#"
                #version 330 core
                out vec4 FragColor;
                uniform vec4 u_Color;
                void main() {
                    FragColor = u_Color;
                }
            "#.to_string(),
            FragmentShaderKind::Texture => r#"
                #version 330 core
                out vec4 FragColor;
                in vec2 TexCoord;

                uniform sampler2D u_Texture;

                void main() {
                    FragColor = texture(u_Texture, TexCoord);
                }
            "#.to_string(),
            FragmentShaderKind::Skybox => r#"
                #version 330 core
                out vec4 FragColor;
                in vec3 TexCoords;
                uniform samplerCube skybox;
                void main() {
                    FragColor = texture(skybox, TexCoords);
                }
            "#.to_string(),
            FragmentShaderKind::TextureWithColorMix => r#"
                #version 330 core
                out vec4 FragColor;
                in vec2 TexCoord;

                uniform sampler2D u_Texture;
                uniform vec4 u_Color;

                void main() {
                    FragColor = texture(u_Texture, TexCoord) * u_Color;
                }
            "#.to_string(),
            FragmentShaderKind::File(path) => {
                std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read fragment shader file: {}", path))
            },
            FragmentShaderKind::Custom(src) => src.clone(),
        }
    }
}
