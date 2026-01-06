#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec2 Position;

void main()
{
    // Normalize the normal vector
    vec3 norm = normalize(Normal);
    
    // Use the x and y coordinates of the position for red and green channels
    // Use the z component of the normal for the blue channel
    FragColor = vec4(Position.x + 0.5, Position.y + 0.5, norm.z, 1.0);
}