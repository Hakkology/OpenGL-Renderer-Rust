#version 330 core
out vec4 FragColor;
uniform vec4 u_Color;

void main() {
    // Düz renk çıktısı
    FragColor = u_Color;
}
