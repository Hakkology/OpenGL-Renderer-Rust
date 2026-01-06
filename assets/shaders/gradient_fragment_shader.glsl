#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

void main() {
    // Create a vertical gradient from orange to blue
    vec3 topColor = vec3(1.0, 0.5, 0.2); // Orange
    vec3 bottomColor = vec3(0.2, 0.4, 0.8); // Blue
    
    // Interpolate between the two colors based on the y-coordinate
    vec3 gradientColor = mix(bottomColor, topColor, TexCoords.y);
    
    FragColor = vec4(gradientColor, 1.0);
}