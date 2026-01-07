#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices=18) out;

uniform mat4 shadowMatrices[6];

out vec4 FragPos; // Output per vertex

void main()
{
    // For each face of the cubemap (6 faces)
    for(int face = 0; face < 6; ++face)
    {
        gl_Layer = face; // Built-in variable: specifies which face to render
        for(int i = 0; i < 3; ++i) // For each triangle vertex
        {
            FragPos = gl_in[i].gl_Position;
            gl_Position = shadowMatrices[face] * FragPos;
            EmitVertex();
        }    
        EndPrimitive();
    }
}
