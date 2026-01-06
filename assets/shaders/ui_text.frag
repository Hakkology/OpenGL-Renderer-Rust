#version 330 core
in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D u_Texture;
uniform vec3 u_Color;

void main() {
    float sampled = texture(u_Texture, TexCoord).r;
    FragColor = vec4(u_Color, sampled);
}
