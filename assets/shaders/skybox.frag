#version 330 core
out vec4 FragColor;

in vec3 TexCoords;

uniform samplerCube skybox;

void main() {
    // Sample cubemap
    FragColor = texture(skybox, TexCoords);
}
