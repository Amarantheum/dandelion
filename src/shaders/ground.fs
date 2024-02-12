#version 430
out vec4 fragColor;

in vec3 normal_interpolated;
in vec3 vertex_position;

void main() {
    float brightness = dot(normalize(normal_interpolated), normalize(vertex_position));
    brightness = exp(-100.0 * brightness * brightness);
    fragColor = vec4(vec3(1.0) * brightness, 1.0);
}