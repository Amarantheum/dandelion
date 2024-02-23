#version 430
out vec4 fragColor;
in vec3 normal_interpolated;

uniform vec4 color;

void main() {
    fragColor = vec4(color.r, color.g, color.b, 1.0);
}