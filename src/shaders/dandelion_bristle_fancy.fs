#version 430
out vec4 fragColor;

uniform vec4 color;
uniform float brightness;

in vec3 normal_interpolated;
in vec3 vertex_position;

void main() {
    float direction = -dot(normalize(normal_interpolated), normalize(vertex_position));
    if (direction < 0.0) {
        discard;
    }
    
    fragColor = color;
}