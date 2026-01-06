use super::ShaderType;

pub enum VertexShaderKind {
    Standard,
    Texture,
    ModelViewProjection,
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
    Texture,
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
                    FragColor = vec4(ourColor, 1.0);
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
