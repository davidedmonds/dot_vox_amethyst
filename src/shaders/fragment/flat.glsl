#version 150 core

in VertexData {
    vec3 position;
    vec4 color;
} vertex;

out vec4 color;

void main() {
    color = vertex.color;
}
