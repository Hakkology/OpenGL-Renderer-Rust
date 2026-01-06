#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormalOrTexCoord;

out vec2 TexCoords;
out vec3 Normal;
out vec2 Position;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    TexCoords = aNormalOrTexCoord.xy;
    Normal = aNormalOrTexCoord;
    Position = aPos.xy;
}
