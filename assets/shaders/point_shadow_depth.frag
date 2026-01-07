#version 330 core
in vec4 FragPos;

uniform vec3 lightPos;
uniform float far_plane;

void main()
{
    float lightDistance = length(FragPos.xyz - lightPos);
    
    // Normalize distance to [0, 1]
    lightDistance = lightDistance / far_plane;
    
    // Write to depth buffer
    gl_FragDepth = lightDistance;
}
