#version 330 core
layout (location = 0) in vec3 aPos;

uniform mat4 lightSpaceMatrix;
uniform mat4 model;

void main() {
    // Transform vertex to light space
    gl_Position = lightSpaceMatrix * model * vec4(aPos, 1.0);
}
